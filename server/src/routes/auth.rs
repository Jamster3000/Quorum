//! Authentication-related route handlers
//! Handles user signup, login, account deletion, data retrieval, token refresh, and logout.

use axum::{Json, extract::State, http::StatusCode};

use crate::db::DB;
use crate::db::queries::auth;
use crate::models::user::User;
use crate::models::user::{
    AuthTokenResponse, DeleteAccountRequest, GetUserDataRequest, LoginRequest, SignupRequest,
    TokenResponse, UpdateUserProfileRequest, UserDataResponse,
};
use crate::utility::config::Config;
use crate::models::server::AuditEvent;
use crate::db::queries::audit_logs::log_audit_event;

use chrono;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub user_id: String,
    pub refresh_token: String,
}

async fn verify_user_credentials(
    db: &DB,
    username_or_email: &str,
    password: &str,
) -> Result<User, (StatusCode, String)> {
    auth::verify_user_credentials(db, username_or_email, password)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid username/email or password".to_string(),
            )
        })
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
    let signup_result = auth::signup_user(
        &db,
        &payload.username,
        payload.email.as_deref(),
        &payload.password,
    )
    .await;

    if let Err(e) = signup_result {
        let message = e.to_string();
        let formatted = format!("Failed to create user: {}", message);

        let status = if message.contains("Invalid password length")
            || message.contains("idx_username")
            || message.contains("idx_email")
        {
            StatusCode::BAD_REQUEST
        } else if message.contains("idx_username") || message.contains("UNIQUE") {
            StatusCode::CONFLICT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        let _ = log_audit_event(&db, AuditEvent {
            log_type: "signup_failed".to_string(),
            action: Some(message.clone()),
            ..Default::default()
        }).await;

        return (
            status,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: formatted,
            }),
        );
    }

    let user = signup_result.unwrap();

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
    if auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at)
        .await
        .is_err()
    {
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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "signup_success".to_string(),
        target_type_table: Some("users".to_string()),
        target_type_table_id: Some(user_id.clone()),
        user_id: Some(user_id.clone()),
        ..Default::default()
    }).await;

    (
        StatusCode::CREATED,
        Json(AuthTokenResponse {
            success: true,
            user: Some(user.to_response()),
            tokens: Some(TokenResponse {
                access_token,
                refresh_token,
                expires_in: crate::utility::config::Config::get().jwt_access_minutes * 60,
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
            let _ = log_audit_event(&db, AuditEvent {
                log_type: "login_failed".to_string(),
                action: Some(e.clone()),
                ..Default::default()
            }).await;

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

    let refresh_token = match crate::utility::jwt::generate_refresh_token(&user_id, &user.username)
    {
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
    if auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at)
        .await
        .is_err()
    {
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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "login_success".to_string(),
        target_type_table: Some("users".to_string()),
        target_type_table_id: Some(user_id.clone()),
        user_id: Some(user_id.clone()),
        ..Default::default()
    }).await;

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

pub async fn delete_account(
    State(db): State<DB>,
    Json(payload): Json<DeleteAccountRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    let _user =
        match auth::verify_user_credentials(&db, &payload.username_or_email, &payload.password)
            .await
        {
            Ok(user) => user,
            Err((status, message)) => {
                let _ = log_audit_event(&db, AuditEvent {
                    log_type: "delete_account_failed".to_string(),
                    action: Some(message.clone()),
                    target_type_table: Some("users".to_string()),
                    target_type_table_id: Some(payload.user_id.clone()),
                    ..Default::default()
                }).await;

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

    if auth::delete_user_by_id(&db, &payload.user_id)
        .await
        .is_err()
    {
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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "delete_account_success".to_string(),
        ..Default::default()
    }).await;

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

#[axum::debug_handler]
pub async fn get_user_data(
    State(db): State<DB>,
    Json(payload): Json<GetUserDataRequest>,
) -> (StatusCode, Json<UserDataResponse>) {
    let user =
        match verify_user_credentials(&db, &payload.username_or_email, &payload.password).await {
            Ok(user) => user,
            Err((status, message)) => {
                let _ = log_audit_event(&db, AuditEvent {
                    log_type: "get_user_data_failed".to_string(),
                    action: Some(message.clone()),
                    target_type_table: Some("users".to_string()),
                    target_type_table_id: Some(payload.username_or_email.clone()),
                    user_id: Some(payload.user_id),
                    ..Default::default()
                }).await;

                return (
                    status,
                    Json(UserDataResponse {
                        success: false,
                        data: None,
                        message,
                    }),
                );
            }
        };

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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "get_user_data_success".to_string(),
        target_type_table: Some("users".to_string()),
        target_type_table_id: Some(payload.user_id.clone()),
        user_id: Some(payload.user_id),
        ..Default::default()
    }).await;

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
/// * `State(_db)` - Shared state containing the database connection (not used but required by Axum)
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
    State(db): State<DB>,
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
            let user_id = payload.user_id.clone();

            let _ = log_audit_event(&db, AuditEvent {
                log_type: "refresh_token_failed".to_string(),
                action: Some("Invalid or expired refresh token".to_string()),
                target_type_table_id: Some(user_id.clone()),
                user_id: Some(user_id),
                ..Default::default()
            }).await;

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

    //Generate access token
    let access_token =
        match crate::utility::jwt::generate_access_token(&claims.sub, &claims.username) {
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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "refresh_token_success".to_string(),
        target_type_table: Some("users".to_string()),
        target_type_table_id: Some(claims.sub.clone()),
        user_id: Some(claims.sub.clone()),
        ..Default::default()
    }).await;

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
    if crate::utility::jwt::verify_token(&payload.refresh_token).is_err() {
        let user_id = payload.user_id.clone();

        let _ = log_audit_event(&db, AuditEvent {
            log_type: "logout_failed".to_string(),
            action: Some("Invalid or expired refresh token".to_string()),
            user_id: Some(user_id),
            ..Default::default()
        }).await;

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

    if auth::revoke_refresh_token(&db, &payload.refresh_token)
        .await
        .is_err()
    {
        let user_id = payload.user_id.clone();

        let _ = log_audit_event(&db, AuditEvent {
            log_type: "logout_failed".to_string(),
            action: Some("Failed to revoke refresh token".to_string()),
            user_id: Some(user_id),
            ..Default::default()
        }).await;

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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "logout_success".to_string(),
        user_id: Some(payload.user_id),
        ..Default::default()
    }).await;

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

pub async fn update_user_profile(
    State(db): State<DB>,
    Json(payload): Json<UpdateUserProfileRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    let user =
        match auth::update_user_profile(&db, &payload.user_id, &payload.email, &payload.username)
            .await
        {
            Ok(user) => user,
            Err((status, message)) => {
                let _ = log_audit_event(&db, AuditEvent {
                    log_type: "update_user_profile_failed".to_string(),
                    action: Some(message.clone()),
                    target_type_table: Some("users".to_string()),
                    target_type_table_id: Some(payload.user_id.clone()),
                    user_id: Some(payload.user_id),
                    ..Default::default()
                }).await;

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

    let _ = log_audit_event(&db, AuditEvent {
        log_type: "update_user_profile_success".to_string(),
        target_type_table: Some("users".to_string()),
        target_type_table_id: Some(payload.user_id.clone()),
        user_id: Some(payload.user_id),
        ..Default::default()
    }).await;

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: Some(user.to_response()),
            tokens: None,
            message: "Profile updated successfully".to_string(),
        }),
    )
}
