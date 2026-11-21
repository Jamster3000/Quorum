//! Authentication-related route handlers
//! Handles user signup, login, account deletion, data retrieval, token refresh, and logout.

use axum::{Json, extract::State, http::StatusCode};

use crate::db::queries::auth;
use crate::models::user::User;
use crate::models::user::{
    AuthTokenResponse, DeleteAccountRequest, GetUserDataRequest, LoginRequest, SignupRequest,
    TokenResponse, UpdateUserProfileRequest, UserDataResponse,
};
use crate::utility::auth_common::generate_backup_codes;
use quorum_core::db::init::DB;
use quorum_core::db::queries::audit_logs::log_audit_event;
use quorum_core::models::server::AuditEvent;
use quorum_core::utility::config::Config;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub user_id: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    #[serde(flatten)]
    pub auth: AuthTokenResponse,

    pub backup_codes: Option<Vec<String>>,
}

/// Builds the standard failure response that is usually returned by most auth endpoints
fn auth_error(
    status: StatusCode,
    message: impl Into<String>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    (
        status,
        Json(AuthTokenResponse {
            success: false,
            user: None,
            tokens: None,
            message: message.into(),
        }),
    )
}

/// Writes an audit event in the background so the response isn't held up by the write.
/// Failures are ignored, matching the previous `let _ = log_audit_event(...)` behaviour.
fn audit(db: &DB, event: AuditEvent) {
    let db = db.clone();
    tokio::spawn(async move {
        let _ = log_audit_event(&db, event).await;
    });
}

/// Logs a failure audit event and returns the standard failure response
fn audited_failure(
    db: &DB,
    event: AuditEvent,
    status: StatusCode,
    message: impl Into<String>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    audit(db, event);
    auth_error(status, message)
}

/// Verifies user credentials (username/email and password) against the database
///
/// # Arguments
/// * `db` - A reference to the database connection
/// * `username_or_email` - The username or email provided by the user
/// * `password` - The password provided by the user
///
/// # Returns
/// * `Ok(User)` - The User struct if the credentials are valid
/// * `Err((StatusCode, String))` - A tuple containing the HTTP status code and an error message if the credentials are invalid
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
/// * `(StatusCode, Json<SignupResponse>)` - The HTTP status code and JSON response containing the authentication token, backup codes if applicable, or an error message
///
/// # Error
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error processing the password, creating the user, generating tokens, or storing the refresh token
/// * `StatusCode::CONFLICT` - If the username already exists
/// * `StatusCode::BAD_REQUEST` - If the password length is invalid or the email already exists
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
) -> (StatusCode, Json<SignupResponse>) {
    let mut plain_backup_codes = None;
    let mut hash_backup_codes = None;

    if payload.email.is_none() {
        let backup_codes = Some(generate_backup_codes());

        plain_backup_codes = Some(
            backup_codes
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|code| code.plain.clone())
                .collect::<Vec<String>>(),
        );
        hash_backup_codes = Some(
            backup_codes
                .as_ref()
                .unwrap()
                .iter()
                .map(|code| code.hash.clone())
                .collect::<Vec<String>>(),
        );
    }

    let signup_result = auth::signup_user(
        &db,
        &payload.username,
        payload.email.as_deref(),
        &payload.password,
        hash_backup_codes,
    )
    .await;

    let user = match signup_result {
        Ok(user) => user,
        Err(e) => {
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

            audit(
                &db,
                AuditEvent {
                    log_type: "signup_failed".to_string(),
                    action: Some(message),
                    ..Default::default()
                },
            );

            let (status, Json(auth)) = auth_error(status, formatted);

            return (
                status,
                Json(SignupResponse {
                    auth,
                    backup_codes: None,
                }),
            );
        }
    };

    let config = Config::get();

    let user_id = match extract_user_id(&user) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SignupResponse {
                    auth: AuthTokenResponse {
                        success: false,
                        user: None,
                        tokens: None,
                        message: e,
                    },
                    backup_codes: None,
                }),
            );
        }
    };

    let access_token = match crate::utility::jwt::generate_access_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SignupResponse {
                    auth: AuthTokenResponse {
                        success: false,
                        user: None,
                        tokens: None,
                        message: "Failed to generate access token".to_string(),
                    },
                    backup_codes: None,
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
                Json(SignupResponse {
                    auth: AuthTokenResponse {
                        success: false,
                        user: None,
                        tokens: None,
                        message: "Failed to generate refresh token".to_string(),
                    },
                    backup_codes: None,
                }),
            );
        }
    };

    let expires_at = chrono::Utc::now().timestamp() + (config.jwt_refresh_days * 86400);

    if auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at)
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SignupResponse {
                auth: AuthTokenResponse {
                    success: false,
                    user: None,
                    tokens: None,
                    message: "Failed to store refresh token".to_string(),
                },
                backup_codes: None,
            }),
        );
    }

    audit(
        &db,
        AuditEvent {
            log_type: "signup_success".to_string(),
            target_type_table: Some("users".to_string()),
            target_type_table_id: Some(user_id.clone()),
            user_id: Some(user_id),
            ..Default::default()
        },
    );

    (
        StatusCode::CREATED,
        Json(SignupResponse {
            auth: AuthTokenResponse {
                success: true,
                user: Some(user.to_response()),
                tokens: Some(TokenResponse {
                    access_token,
                    refresh_token,
                    expires_in: config.jwt_access_minutes * 60,
                }),
                message: "User created successfully".to_string(),
            },
            backup_codes: plain_backup_codes,
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
            Err((status, message)) => return auth_error(status, message),
        };

    let user_id = match extract_user_id(&user) {
        Ok(id) => id,
        Err(e) => {
            return audited_failure(
                &db,
                AuditEvent {
                    log_type: "login_failed".to_string(),
                    action: Some(e.clone()),
                    ..Default::default()
                },
                StatusCode::INTERNAL_SERVER_ERROR,
                e,
            );
        }
    };

    let config = Config::get();

    let access_token = match crate::utility::jwt::generate_access_token(&user_id, &user.username) {
        Ok(token) => token,
        Err(_) => {
            return auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate access token",
            );
        }
    };

    let refresh_token = match crate::utility::jwt::generate_refresh_token(&user_id, &user.username)
    {
        Ok(token) => token,
        Err(_) => {
            return auth_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to generate refresh token",
            );
        }
    };

    let expires_at = chrono::Utc::now().timestamp() + (config.jwt_refresh_days * 86400);
    if auth::store_refresh_token(&db, &user_id, &refresh_token, expires_at)
        .await
        .is_err()
    {
        return auth_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to store refresh token",
        );
    }

    audit(
        &db,
        AuditEvent {
            log_type: "login_success".to_string(),
            target_type_table: Some("users".to_string()),
            target_type_table_id: Some(user_id.clone()),
            user_id: Some(user_id),
            ..Default::default()
        },
    );

    (
        StatusCode::OK,
        Json(AuthTokenResponse {
            success: true,
            user: Some(user.to_response()),
            tokens: Some(TokenResponse {
                access_token,
                refresh_token,
                expires_in: config.jwt_access_minutes * 60,
            }),
            message: "Login successful".to_string(),
        }),
    )
}

/// Handles user account deletion requests
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection
/// * `Json(payload)` - The account deletion request payload containing username/email, password, and user ID
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response indicating success or failure of the account deletion
///
/// # Example
/// ```rust
/// let payload = DeleteAccountRequest {
///     username_or_email: "john_doe",
///     password: "secret_password",
///     user_id: "user_id"
/// };
/// ```
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
                return audited_failure(
                    &db,
                    AuditEvent {
                        log_type: "delete_account_failed".to_string(),
                        action: Some(message.clone()),
                        target_type_table: Some("users".to_string()),
                        target_type_table_id: Some(payload.user_id.clone()),
                        ..Default::default()
                    },
                    status,
                    message,
                );
            }
        };

    if auth::delete_user_by_id(&db, &payload.user_id)
        .await
        .is_err()
    {
        return auth_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete account",
        );
    }

    audit(
        &db,
        AuditEvent {
            log_type: "delete_account_success".to_string(),
            ..Default::default()
        },
    );

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
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection
/// * `Json(payload)` - The user data retrieval request payload containing username/email, password, and requested fields
///
/// # Returns
/// * `(StatusCode, Json<UserDataResponse>)` - The HTTP status code and JSON response containing the requested user data or an error message
///
/// # Example
/// ```rust
/// let payload = GetUserDataRequest {
///     username_or_email: "john_doe",
///     password: "secret_password",
///     user_id: "user_id",
///     fields: vec!["id".to_string(), "username".to_string()],
/// };
/// ```
pub async fn get_user_data(
    State(db): State<DB>,
    Json(payload): Json<GetUserDataRequest>,
) -> (StatusCode, Json<UserDataResponse>) {
    let user =
        match verify_user_credentials(&db, &payload.username_or_email, &payload.password).await {
            Ok(user) => user,
            Err((status, message)) => {
                audit(
                    &db,
                    AuditEvent {
                        log_type: "get_user_data_failed".to_string(),
                        action: Some(message.clone()),
                        target_type_table: Some("users".to_string()),
                        target_type_table_id: Some(payload.username_or_email.clone()),
                        user_id: Some(payload.user_id),
                        ..Default::default()
                    },
                );

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
                // Bare ID, falling back to Debug only for unexpected key types
                let id = extract_user_id(&user).unwrap_or_else(|_| format!("{:?}", user.id.key));
                user_data.insert("id".to_string(), serde_json::Value::String(id));
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

    audit(
        &db,
        AuditEvent {
            log_type: "get_user_data_success".to_string(),
            target_type_table: Some("users".to_string()),
            target_type_table_id: Some(payload.user_id.clone()),
            user_id: Some(payload.user_id),
            ..Default::default()
        },
    );

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
/// * `State(db)` - Shared state containing the database connection
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
///     user_id: "user_id".to_string(),
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
                return auth_error(StatusCode::UNAUTHORIZED, "Invalid token type");
            }
            claims
        }
        Err(_) => {
            let user_id = payload.user_id;

            return audited_failure(
                &db,
                AuditEvent {
                    log_type: "refresh_token_failed".to_string(),
                    action: Some("Invalid or expired refresh token".to_string()),
                    target_type_table_id: Some(user_id.clone()),
                    user_id: Some(user_id),
                    ..Default::default()
                },
                StatusCode::UNAUTHORIZED,
                "Invalid or expired refresh token",
            );
        }
    };

    let user_id = claims.sub.clone();

    if auth::validate_refresh_token(&db, &user_id, &payload.refresh_token)
        .await
        .is_err()
    {
        return audited_failure(
            &db,
            AuditEvent {
                log_type: "refresh_token_failed".to_string(),
                action: Some("Refresh token not found or revoked".to_string()),
                target_type_table: Some("users".to_string()),
                target_type_table_id: Some(user_id.clone()),
                user_id: Some(user_id),
                ..Default::default()
            },
            StatusCode::UNAUTHORIZED,
            "Invalid or expired refresh token",
        );
    }

    // Generate access token
    let access_token =
        match crate::utility::jwt::generate_access_token(&claims.sub, &claims.username) {
            Ok(token) => token,
            Err(_) => {
                return auth_error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to generate access token",
                );
            }
        };

    audit(
        &db,
        AuditEvent {
            log_type: "refresh_token_success".to_string(),
            target_type_table: Some("users".to_string()),
            target_type_table_id: Some(user_id.clone()),
            user_id: Some(user_id),
            ..Default::default()
        },
    );

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
/// * `State(db)` - Shared state containing the database connection
/// * `Json(payload)` - The logout request payload containing the refresh token to be revoked
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response indicating success or failure of the logout operation
///
/// # Error
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error revoking the refresh token
///
/// # Example
/// ```rust
/// let payload = RefreshTokenRequest {
///     user_id: "user_id".to_string(),
///     refresh_token: "valid_refresh_token".to_string(),
/// };
/// ```
pub async fn logout(
    State(db): State<DB>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    if crate::utility::jwt::verify_token(&payload.refresh_token).is_err() {
        return audited_failure(
            &db,
            AuditEvent {
                log_type: "logout_failed".to_string(),
                action: Some("Invalid or expired refresh token".to_string()),
                user_id: Some(payload.user_id),
                ..Default::default()
            },
            StatusCode::UNAUTHORIZED,
            "Invalid or expired refresh token",
        );
    }

    if auth::revoke_refresh_token(&db, &payload.refresh_token)
        .await
        .is_err()
    {
        return audited_failure(
            &db,
            AuditEvent {
                log_type: "logout_failed".to_string(),
                action: Some("Failed to revoke refresh token".to_string()),
                user_id: Some(payload.user_id),
                ..Default::default()
            },
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to logout",
        );
    }

    audit(
        &db,
        AuditEvent {
            log_type: "logout_success".to_string(),
            user_id: Some(payload.user_id),
            ..Default::default()
        },
    );

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

/// Handles user profile update requests
/// This endpoint allows clients to update their user profile information,
/// such as username and email. It verifies the provided data, updates the user profile in the database, and logs the operation.
///
/// # Arguments
/// * `State(db)` - Shared state containing the database connection
/// * `Json(payload)` - The profile update request payload containing the user ID and new profile information
///
/// # Returns
/// * `(StatusCode, Json<AuthTokenResponse>)` - The HTTP status code and JSON response indicating success or failure of the profile update operation
///
/// # Example
/// ```rust
/// let payload = UpdateUserProfileRequest {
///     user_id: "user_id".to_string(),
///     username: Some("new_username".to_string()),
///     email: Some("new_email@example.com".to_string()),
/// };
/// ```
pub async fn update_user_profile(
    State(db): State<DB>,
    Json(payload): Json<UpdateUserProfileRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    if payload.username.trim().is_empty() {
        return auth_error(StatusCode::BAD_REQUEST, "Username cannot be empty");
    }

    let user = match auth::update_user_profile(&db, &payload).await {
        Ok(user) => user,
        Err((status, message)) => {
            return audited_failure(
                &db,
                AuditEvent {
                    log_type: "update_user_profile_failed".to_string(),
                    action: Some(message.clone()),
                    target_type_table: Some("users".to_string()),
                    target_type_table_id: Some(payload.user_id.clone()),
                    user_id: Some(payload.user_id),
                    ..Default::default()
                },
                status,
                message,
            );
        }
    };

    audit(
        &db,
        AuditEvent {
            log_type: "update_user_profile_success".to_string(),
            target_type_table: Some("users".to_string()),
            target_type_table_id: Some(payload.user_id.clone()),
            user_id: Some(payload.user_id),
            ..Default::default()
        },
    );

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
