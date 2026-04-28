//! Authentication-related route handlers
//! Handles user signup, login, account deletion, data retrieval, token refresh, and logout.

use axum::{Json, extract::State, http::StatusCode};

use crate::db::DB;
use crate::db::queries::auth;
use crate::models::user::User;
use crate::models::user::{
    AuthTokenResponse, DeleteAccountRequest, GetUserDataRequest, LoginRequest,
    SignupRequest, TokenResponse, UserDataResponse,
};
use crate::utility::password;
use crate::utility::config::Config;

use chrono;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Verifies user credentials and returns the user if valid
///
/// # Arguments
/// * `db` - Database connection
/// * `username_or_email` - Username or email provided by the user
/// * `password` - Password provided by the user
///
/// # Returns
/// * `Ok(User)` - If credentials are valid
/// * `Err((StatusCode, String))` - If credentials are invalid or an error occurs
///
/// # Error
/// * `StatusCode::UNAUTHORIZED` - If username/email or password is incorrect
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error verifying the password
async fn verify_user_credentials(
    db: &DB,
    username_or_email: &str,
    password: &str,
) -> Result<User, (StatusCode, String)> {
    let user = auth::get_user_by_username_or_email(db, username_or_email)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid username/email or password".to_string(),
            )
        })?;

    let password_hash = user.password_hash.as_ref().ok_or((
        StatusCode::UNAUTHORIZED,
        "Invalid username/email or password".to_string(),
    ))?;

    let is_valid = password::verify_password(password, password_hash)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error verifying password".to_string(),
            )
        })?;

    is_valid.then_some(user).ok_or((
        StatusCode::UNAUTHORIZED,
        "Invalid username/email or password".to_string(),
    ))
}

/// Extracts the user ID as a string from the User struct
///
/// # Arguments
/// * `user` - The User struct from which to extract the ID
///
/// # Returns
/// * `Ok(String)` - The user ID as a string if extraction is successful
/// * `Err(String)` - An error message if the ID format is invalid
fn extract_user_id(user: &User) -> Result<String, String> {
    match &user.id.key {
        surrealdb_types::RecordIdKey::String(s) => Ok(s.to_string()),
        surrealdb_types::RecordIdKey::Number(n) => Ok(n.to_string()),
        _ => Err("Invalid user ID format".to_string()),
    }
}

/// Handles user signup requests
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration
/// * `Json(payload)` - The signup request payload containing username, email, and password
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response containing the authentication token or error message
///
/// # Error
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error processing the password, creating the user, generating tokens, or storing the refresh token
/// * `StatusCode::CONFLICT` - If the username already exists
/// * `StatusCode::BAD_REQUEST` - If there is a user ID mismatch during account deletion
///
/// # Example
/// ```rust
/// let payload = SignupRequest {
///     username: "newuser".to_string(),
///     email: Some("newuser@example.com".to_string()),
///     password: "password123".to_string(),
/// };
/// ```
pub async fn signup(
    State(db): State<DB>,
    Json(payload): Json<SignupRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    let password_hash = match password::hash_password(&payload.password).await {
        Ok(hash) => hash,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to process password".to_string(),
                }),
            );
        }
    };

    let user = match auth::signup_user(
        &db,
        &payload.username,
        payload.email.as_deref(),
        &password_hash,
    )
    .await
    {
        Ok(user) => user,
        Err(e) => {
            let message = if e.to_string().contains("idx_username") {
                "Username already exists".to_string()
            } else {
                format!("Failed to create user: {}", e)
            };
            return (
                StatusCode::CONFLICT,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message,
                }),
            );
        }
    };

    let user_id = match extract_user_id(&user) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: e,
                }),
            );
        }
    };

    let access_token = match crate::utility::jwt::generate_access_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to generate access token".to_string(),
                }),
            );
        }
    };

    let refresh_token = match crate::utility::jwt::generate_refresh_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to generate refresh token".to_string(),
                }),
            );
        }
    };

    let config = crate::utility::config::Config::get();
    let expires_at = chrono::Utc::now().timestamp() + (config.jwt_refresh_days * 86400);
    if let Err(_) = auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Failed to store refresh token".to_string(),
            }),
        );
    }

    (
        StatusCode::CREATED,
        Json(AuthTokenResponse {
            success: true,
            user: Some(user.to_response()),
            tokens: Some(TokenResponse {
                access_token,
                refresh_token,
                expires_in: Config::get().jwt_access_minutes * 60,
            }),
            message: "User created successfully".to_string(),
        }),
    )
}

/// Handles user login requests
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration
/// * `Json(payload)` - The login request payload containing username/email and password
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response containing the authentication token or error message
///
/// # Error
/// * `StatusCode::UNAUTHORIZED` - If the username/email or password is incorrect
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error verifying credentials, generating tokens, or storing the refresh token
///
/// # Example
/// ```rust
/// let payload = LoginRequest {
///     username_or_email: "user@example.com".to_string(),
///     password: "password123".to_string(),
/// };
/// ```
pub async fn login(
    State(db): State<DB>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    let user =
        match verify_user_credentials(&db, &payload.username_or_email, &payload.password).await {
            Ok(user) => user,
            Err((status, message)) => {
                return (
                    status,
                    Json(AuthTokenResponse {
                        success: false,
                        user: None,
                        tokens: None,
                        message,
                    }),
                );
            }
        };

    let user_id = match extract_user_id(&user) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: e,
                }),
            );
        }
    };

    let access_token = match crate::utility::jwt::generate_access_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to generate access token".to_string(),
                }),
            );
        }
    };

    let refresh_token = match crate::utility::jwt::generate_refresh_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to generate refresh token".to_string(),
                }),
            );
        }
    };

    let expires_at = chrono::Utc::now().timestamp() + (Config::get().jwt_refresh_days * 86400);
    if let Err(_) = auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Failed to store refresh token".to_string(),
            }),
        );
    }

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: Some(user.to_response()),
            tokens: Some(TokenResponse {
                access_token,
                refresh_token,
                expires_in: Config::get().jwt_access_minutes * 60,
            }),
            message: "Login successful".to_string(),
        }),
    )
}

/// Handles account deletion requests
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration (JWT config is not used in this handler)
/// * `Json(payload)` - The account deletion request payload containing username/email, password, and user ID
///
/// # Returns
/// * `(StatusCode, Json<AuthResponse>)` - The HTTP status code and JSON response indicating success or failure of account deletion
///
/// # Error
/// * `StatusCode::UNAUTHORIZED` - If the username/email or password is incorrect
/// * `StatusCode::BAD_REQUEST` - If there is a user ID mismatch during account deletion
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error deleting the account
///
/// # Example
/// ```rust
/// let payload = DeleteAccountRequest {
///     username_or_email: "user@example.com".to_string(),
///     password: "password123".to_string(),
///     user_id: "user_id".to_string(),
/// };
/// ```
pub async fn delete_account(
    State(db): State<DB>,
    Json(payload): Json<DeleteAccountRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    let user = match auth::get_user_by_username_or_email(&db, &payload.username_or_email).await {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Invalid username/email or password".to_string(),
                }),
            );
        }
    };

    let password_hash = match user.password_hash.as_ref() {
        Some(hash) => hash,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Invalid username/email or password".to_string(),
                }),
            );
        }
    };

    let is_valid = match password::verify_password(&payload.password, password_hash).await {
        Ok(valid) => valid,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Error verifying password".to_string(),
                }),
            );
        }
    };

    if !is_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Invalid username/email or password".to_string(),
            }),
        );
    }

    if let Err(_) = auth::delete_user_by_id(&db, &payload.user_id).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Failed to delete account".to_string(),
            }),
        );
    }

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: None,
            tokens: None,
            message: "Account deleted successfully".to_string(),
        }),
    )
}

/// Handles user data retrieval requests
/// This request can be used to get any information about a user. It benefits better than having a separate endpoint for each field,
/// and it also allows the client to specify which fields they want to retrieve, reducing unnecessary data transfer.
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration (JWT config is not used in this handler)
/// * `Json(payload)` - The user data retrieval request payload containing username/email, password, and list of fields to retrieve
///
/// # Returns
/// * `(StatusCode, Json<UserDataResponse>)` - The HTTP status code and JSON response containing the requested user data or an error message
///
/// # Error
/// * `StatusCode::UNAUTHORIZED` - If the username/email or password is incorrect
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error verifying credentials or extracting user ID
///
/// # Example
/// ```rust
/// let payload = GetUserDataRequest {
///     username_or_email: "user@example.com".to_string(),
///     password: "password123".to_string(),
///     fields: vec!["id".to_string(), "username".to_string(), "email".to_string()],
/// };
/// ```
pub async fn get_user_data(
    State(db): State<DB>,
    Json(payload): Json<GetUserDataRequest>,
) -> (StatusCode, Json<UserDataResponse>) {
    let user = match auth::get_user_by_username_or_email(&db, &payload.username_or_email).await {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(UserDataResponse {
                    success: false,
                    data: None,
                    message: "Invalid username/email or password".to_string(),
                }),
            );
        }
    };

    let password_hash = match user.password_hash.as_ref() {
        Some(hash) => hash,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(UserDataResponse {
                    success: false,
                    data: None,
                    message: "Invalid username/email or password".to_string(),
                }),
            );
        }
    };

    let is_valid = match password::verify_password(&payload.password, password_hash).await {
        Ok(valid) => valid,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UserDataResponse {
                    success: false,
                    data: None,
                    message: "Error verifying password".to_string(),
                }),
            );
        }
    };

    if !is_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(UserDataResponse {
                success: false,
                data: None,
                message: "Invalid username/email or password".to_string(),
            }),
        );
    }

    let mut user_data = serde_json::Map::new();

    for field in &payload.fields {
        match field.as_str() {
            "id" => {
                user_data.insert(
                    "id".to_string(),
                    serde_json::Value::String(format!("{:?}", user.id.key)),
                );
            }
            "username" => {
                user_data.insert(
                    "username".to_string(),
                    serde_json::Value::String(user.username.clone()),
                );
            }
            "email" => {
                if let Some(email) = &user.email {
                    user_data.insert(
                        "email".to_string(),
                        serde_json::Value::String(email.clone()),
                    );
                }
            }
            "created_at" => {
                user_data.insert(
                    "created_at".to_string(),
                    serde_json::Value::String(user.created_at.to_string()),
                );
            }
            _ => {}
        }
    }

    (
        StatusCode::OK,
        Json(UserDataResponse {
            success: true,
            data: Some(user_data),
            message: "User data retrieved successfully".to_string(),
        }),
    )
}

/// Handles token refresh requests
/// This endpoint allows clients to obtain a new access token using a valid refresh token. It verifies the refresh token, checks its validity in the database, and generates a new access token if everything is valid. The refresh token is not rotated in this implementation, but it can be easily modified to do so if desired.
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration
/// * `Json(payload)` - The refresh token request payload containing the refresh token
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response containing the new access token or an error message
///
/// # Error
/// * `StatusCode::UNAUTHORIZED` - If the refresh token is invalid, expired, of the wrong type, or not found in the database
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error generating the new access token
///
/// # Example
/// ```rust
/// let payload = RefreshTokenRequest {
///     refresh_token: "valid_refresh_token".to_string(),
/// };
/// ```
pub async fn refresh_token(
    State(_db): State<DB>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    // Verify token
    let claims = match crate::utility::jwt::verify_token(&payload.refresh_token) {
        Ok(claims) => {
            if claims.token_type != "refresh" {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(AuthTokenResponse {
                        success: false,
                        user: None,
                        tokens: None,
                        message: "Invalid token type".to_string(),
                    }),
                );
            }
            claims
        }
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Invalid or expired refresh token".to_string(),
                }),
            );
        }
    };

    let access_token = match crate::utility::jwt::generate_access_token(&claims.sub, &claims.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to generate access token".to_string(),
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: None,
            tokens: Some(TokenResponse {
                access_token,
                refresh_token: payload.refresh_token,
                expires_in: Config::get().jwt_access_minutes * 60,
            }),
            message: "Token refreshed successfully".to_string(),
        }),
    )
}

/// Handles user logout requests
/// This endpoint allows clients to log out by revoking the provided refresh token. It removes the refresh token from the database,
/// effectively invalidating it and preventing any further use for obtaining new access tokens.
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection and JWT configuration (JWT config is not used in this handler)
/// * `Json(payload)` - The logout request payload containing the refresh token to be revoked
///
/// # Returns
/// * `(StatusCode, Json<AuthResponse>)` - The HTTP status code and JSON response indicating success or failure of the logout operation
///
/// # Error
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error revoking the refresh token
///
/// # Example
/// ```rust
/// let payload = RefreshTokenRequest {
///     refresh_token: "valid_refresh_token".to_string(),
/// };
/// ```
pub async fn logout(
    State(db): State<DB>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    if let Err(_) = auth::revoke_refresh_token(&db, &payload.refresh_token).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Failed to logout".to_string(),
            }),
        );
    }

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: None,
            tokens: None,
            message: "Logged out successfully".to_string(),
        }),
    )
}
