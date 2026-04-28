//! Authentication related database queries
//!
//! This file contains functions for Creating an account (signup), login, token management and deleteing a user account.

use crate::db::DB;
use crate::models::user::User;
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
    password_hash: &str,
) -> Result<User, Box<dyn Error>> {
    let mut response = db
        .query(
            "CREATE users SET username = $username, email = $email, password_hash = $password_hash",
        )
        .bind(("username", username.to_string()))
        .bind(("email", email.map(|e| e.to_string())))
        .bind(("password_hash", password_hash.to_string()))
        .await?;

    let user: Vec<User> = response.take(0)?;
    user.into_iter()
        .next()
        .ok_or("Failed to create user".into())
}

/// Retrieves a user from the database by their username or email
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `username_or_email` - The username or email address to search for
///
/// # Returns
/// * `Ok(User)` - The user object if a matching user was found
/// * `Err(Box<dyn Error>)` - An error if the operation failed, such as if there was a database error or if no user was found
///
/// # Errors
/// * "User not found" - If no user was found with the given username or email
///
/// # Example
/// ```
/// use crate::db::DB;
/// use crate::db::queries::auth;
/// async fn example_get_user(db: &DB) {
///     let username_or_email = "existing_user";
///     match auth::get_user_by_username_or_email(db, username_or_email).await {
///         Ok(user) => println!("User found: {:?}", user),
///         Err(e) => eprintln!("Error retrieving user: {}", e),
///     }
/// }
///```
pub async fn get_user_by_username_or_email(
    db: &DB,
    username_or_email: &str,
) -> Result<User, Box<dyn Error>> {
    let mut response = db
		.query("SELECT id, username, email, password_hash, created_at, last_login, is_banned, is_deleted FROM users WHERE username = $value OR email = $value LIMIT 1")
		.bind(("value", username_or_email.to_string()))
		.await?;

    let user: Vec<User> = response.take(0)?;
    user.into_iter().next().ok_or("User not found".into())
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
pub async fn delete_user_by_id(db: &DB, user_id: &str) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
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
) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
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
pub async fn revoke_refresh_token(db: &DB, refresh_token: &str) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
    tokens::revoke_refresh_token(db, refresh_token).await
}
