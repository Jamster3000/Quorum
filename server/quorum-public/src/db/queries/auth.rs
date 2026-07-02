//! Authentication related database queries
//!
//! This file contains functions for Creating an account (signup), login, token management and deleteing a user account.

use crate::db::queries::tokens;
use crate::models::user::UpdateUserProfileRequest;
use crate::models::user::User;
use crate::utility::auth_common::check_email_address;
use axum::http::StatusCode;
use quorum_core::db::DB;
use std::error::Error;

/// Creates a new user account in the database
///
/// # Arguments
///* `db` - A reference to the database connection
/// * `username` - The desired username for the new account
/// * `email` - An optional email address for the new account
/// * `password_hash` - The hashed password for the new account
///
/// # Returns
/// * `Ok(User)` - The newly created user object if the operation was successful
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if the username is already taken or if there was a database error
///
/// # Errors
/// * "Failed to create user" - If the database query did not return a user object, indicating that the user creation failed
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_signup(db: &DB) {
///     let username = "new_user";
///     let email = Some("123@example.com");
///     let password_hash = "hashed_password";
///     match auth::signup_user(db, username, email, password_hash).await {
///         Ok(user) => println!("User created: {:?}", user),
///         Err(e) => eprintln!("Error creating user: {}", e),
///     }
/// }
///```
pub async fn signup_user(
    db: &DB,
    username: &str,
    email: Option<&str>,
    password: &str,
    email_backup_codes: Option<Vec<String>>,
) -> Result<User, Box<dyn Error + Send + Sync>> {
    if password.len() < 12 || password.len() > 35 {
        return Err("Invalid password length".into());
    }

    if let Some(email) = email {
        if !check_email_address(email) {
            return Err("Invalid email address".into());
        }
    }

    let password_hash = crate::utility::auth_common::hash(password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    let mut response = db
        .query(
            "CREATE users SET
            username = $username,
            email = $email,
            password_hash = $password_hash,
            email_backup_codes = $email_backup_codes",
        )
        .bind(("username", username.to_string()))
        .bind(("email", email.map(|e| e.to_string())))
        .bind(("password_hash", password_hash))
        .bind(("email_backup_codes", email_backup_codes))
        .await?;

    let user: Vec<User> = response.take(0)?;
    user.into_iter()
        .next()
        .ok_or("Failed to create user".into())
}

/// Deletes a user from the database by their ID
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `user_id` - The ID of the user to delete
///
/// # Returns
/// * `Ok(())` - If the user was successfully deleted
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if there was a database error
///
/// # Errors
/// * "Failed to delete user" - If the database query did not execute successfully, indicating that the user could not be deleted
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_delete_user(db: &DB) {
///     let user_id = "user_id_to_delete";
///     match auth::delete_user_by_id(db, user_id).await {
///         Ok(()) => println!("User deleted successfully"),
///         Err(e) => eprintln!("Error deleting user: {}", e),
///     }
/// }
///```
pub async fn delete_user_by_id(db: &DB, user_id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokens::delete_all_user_tokens(db, user_id).await?;

    let query = format!("DELETE FROM users:{}", user_id);
    db.query(&query).await?;
    Ok(())
}

/// Stores a refresh token in the database for a specific user
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `user_id` - The ID of the user for whom the refresh token is being stored
/// * `refresh_token` - The refresh token string to store
/// * `expires_at` - The expiration time of the refresh token as a Unix timestamp
///
/// # Returns
/// * `Ok(())` - If the refresh token was successfully stored
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if there was a database error
///
/// # Errors
/// * "Failed to store refresh token" - If the database query did not execute successfully, indicating that the refresh token could not be stored
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_store_refresh_token(db: &DB) {
///     let user_id = "user_id";
///     let refresh_token = "refresh_token_string";
///     let expires_at = 1700000000; // Example expiration timestamp
///     match auth::store_refresh_token(db, user_id, refresh_token, expires_at).await {
///         Ok(()) => println!("Refresh token stored successfully"),
///         Err(e) => eprintln!("Error storing refresh token: {}", e),
///     }
/// }
///```
pub async fn store_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
    expires_at: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokens::store_refresh_token(db, user_id, refresh_token, expires_at).await
}

/// Revokes a refresh token, preventing it from being used for future authentication
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `refresh_token` - The refresh token string to revoke
///
/// # Returns
/// * `Ok(())` - If the refresh token was successfully revoked
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if there was a database error
///
/// # Errors
/// * "Failed to revoke refresh token" - If the database query did not execute successfully, indicating that the refresh token could not be revoked
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_revoke_refresh_token(db: &DB) {
///     let refresh_token = "refresh_token_string";
///     match auth::revoke_refresh_token(db, refresh_token).await {
///         Ok(()) => println!("Refresh token revoked successfully"),
///         Err(e) => eprintln!("Error revoking refresh token: {}", e),
///     }
/// }
///```
pub async fn revoke_refresh_token(
    db: &DB,
    refresh_token: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokens::revoke_refresh_token(db, refresh_token).await
}

/// Get's user's data to allow them to be verified
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `username` - The username of the user to retrieve
/// * `password` - The password of the user to retrieve
///
/// # Returns
/// * `Ok(User)` - The user object if the operation was successful
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_get_user(db: &DB) {
///     let username = "existing_user";
///     let password = "user_password";
///     match auth::get_user(db, username, password).await {
///         Ok(user) => println!("User retrieved: {:?}", user),
///         Err(e) => eprintln!("Error retrieving user: {}", e),
///     }
/// }
///```
pub async fn verify_user_credentials(
    db: &DB,
    username_or_email: &str,
    password: &str,
) -> Result<User, (StatusCode, String)> {
    let mut response = db
        .query("SELECT * FROM users WHERE (username = $value OR email = $value) AND is_banned = false AND is_deleted = false LIMIT 1")
        .bind(("value", username_or_email.to_string()))
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;

    let user: Option<User> = response.take::<Option<User>>(0).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;

    match user {
        Some(u) => {
            if let Some(ref hash) = u.password_hash {
                if crate::utility::auth_common::verify(password, hash).unwrap_or(false) {
                    Ok(u)
                } else {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        "Invalid username or password".to_string(),
                    ))
                }
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid username or password".to_string(),
                ))
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )),
    }
}

/// Updates a user's profile information in the database
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `user_id` - The ID of the user whose profile is being updated
/// * `email` - The new email address for the user
/// * `username` - The new username for the user
///
/// # Returns
/// * `Ok(User)` - The updated user object if the operation was successful
/// * `Err((StatusCode, String))` - An error if the operation failed, containing an HTTP status code and an error message
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_update_user_profile(db: &DB) {
///     let user_id = "user_id";
///     let new_email = "test@example.com";
///     let new_username = "new_username";
///     match auth::update_user_profile(db, user_id, new_email, new_username).await {
///         Ok(user) => println!("User profile updated: {:?}", user),
///         Err((status, message)) => eprintln!("Error updating user profile ({}): {}", status, message),
///     }
/// }
///```
pub async fn update_user_profile(
    db: &DB,
    payload: &UpdateUserProfileRequest,
) -> Result<User, (StatusCode, String)> {
    let mut response = db
        .query(format!(
            "UPDATE ONLY users:{} SET email = $email, username = $username RETURN AFTER",
            payload.user_id
        ))
        .bind(("email", payload.email.clone()))
        .bind(("username", payload.username.clone()))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;

    let user: Option<User> = response.take(0).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to parse user".to_string(),
        )
    })?;

    user.ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))
}

/// Promotes a user to admin status in the database
///
/// This function uses `#[allow(dead_code)]` due to rust providing a false positive warning
/// that this is dead code and not used, despite obvious usage in `cli/server.rs` -> called from `cli/mod.rs`
/// # Arguments
/// * `db` - A reference to the database connection
/// * `username` - The username of the user to promote to admin
///
/// # Returns
/// * `Ok(())` - If the user was successfully promoted to admin
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if an admin already exists or if there was a database error
///
/// # Examples
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_make_admin(db: &DB) {
///     let username = "user_to_promote";
///     match auth::make_admin(db, username).await {
///         Ok(()) => println!("User promoted to admin successfully"),
///         Err(e) => eprintln!("Error promoting user to admin: {}", e),
///     }
/// }
///```
#[allow(dead_code)]
pub async fn make_admin(db: &DB, username: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check bootstrap flag first
    let mut response = db
        .query("SELECT first_admin_bootstrapped FROM server_config LIMIT 1")
        .await?;

    let bootstrapped: Option<bool> = response.take("first_admin_bootstrapped")?;
    if bootstrapped.unwrap_or(false) {
        return Err("An admin already exists. This command is one time only.".into());
    }

    // Promote the user
    db.query("UPDATE users SET is_admin = true WHERE username = $username")
        .bind(("username", username))
        .await?;

    // Lock the bootstrap
    db.query("UPDATE server_config SET first_admin_bootstrapped = true")
        .await?;

    Ok(())
}

pub async fn validate_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tokens::validate_refresh_token(db, user_id, refresh_token).await
}