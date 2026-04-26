use axum::{Json, extract::State, http::StatusCode};

use crate::db::DB;
use crate::db::queries::auth;
use crate::models::user::User;
use crate::models::user::{
    AuthResponse, AuthTokenResponse, DeleteAccountRequest, GetUserDataRequest, LoginRequest,
    SignupRequest, TokenResponse, UserDataResponse,
};
use crate::utility::jwt::JwtConfig;
use crate::utility::password;

use chrono;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

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

fn extract_user_id(user: &User) -> Result<String, String> {
    match &user.id.key {
        surrealdb_types::RecordIdKey::String(s) => Ok(s.to_string()),
        surrealdb_types::RecordIdKey::Number(n) => Ok(n.to_string()),
        _ => Err("Invalid user ID format".to_string()),
    }
}

pub async fn signup(
    State((db, jwt_config)): State<(DB, JwtConfig)>,
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

    let access_token =
        match crate::utility::jwt::generate_access_token(&user_id, &user.username, &jwt_config) {
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

    let refresh_token =
        match crate::utility::jwt::generate_refresh_token(&user_id, &user.username, &jwt_config) {
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

    let expires_at = chrono::Utc::now().timestamp() + (jwt_config.refresh_expiry_days * 86400);
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
                expires_in: jwt_config.access_expiry_minutes * 60,
            }),
            message: "User created successfully".to_string(),
        }),
    )
}

pub async fn login(
    State((db, jwt_config)): State<(DB, JwtConfig)>,
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

    let access_token =
        match crate::utility::jwt::generate_access_token(&user_id, &user.username, &jwt_config) {
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

    let refresh_token =
        match crate::utility::jwt::generate_refresh_token(&user_id, &user.username, &jwt_config) {
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

    let expires_at = chrono::Utc::now().timestamp() + (jwt_config.refresh_expiry_days * 86400);
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
                expires_in: jwt_config.access_expiry_minutes * 60,
            }),
            message: "Login successful".to_string(),
        }),
    )
}

pub async fn delete_account(
    State((db, _jwt_config)): State<(DB, JwtConfig)>,
    Json(payload): Json<DeleteAccountRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    let user =
        match verify_user_credentials(&db, &payload.username_or_email, &payload.password).await {
            Ok(user) => user,
            Err((status, message)) => {
                return (
                    status,
                    Json(AuthResponse {
                        success: false,
                        user: None,
                        message,
                    }),
                );
            }
        };

    let user_id_string = match extract_user_id(&user) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(AuthResponse {
                    success: false,
                    user: None,
                    message: e,
                }),
            );
        }
    };

    if user_id_string != payload.user_id {
        return (
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                success: false,
                user: None,
                message: "User ID mismatch".to_string(),
            }),
        );
    }

    match auth::delete_user_by_id(&db, &user_id_string).await {
        Ok(_) => (
            StatusCode::OK,
            Json(AuthResponse {
                success: true,
                user: None,
                message: "Account deleted successfully".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                success: false,
                user: None,
                message: format!("Failed to delete account: {}", e),
            }),
        ),
    }
}

pub async fn get_user_data(
    State((db, _jwt_config)): State<(DB, JwtConfig)>,
    Json(payload): Json<GetUserDataRequest>,
) -> (StatusCode, Json<UserDataResponse>) {
    let user =
        match verify_user_credentials(&db, &payload.username_or_email, &payload.password).await {
            Ok(user) => user,
            Err((status, message)) => {
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

    let mut data = serde_json::Map::new();

    for field in &payload.fields {
        match field.as_str() {
            "id" => {
                if let Ok(user_id_string) = extract_user_id(&user) {
                    data.insert("id".to_string(), serde_json::Value::String(user_id_string));
                }
            }
            "username" => {
                data.insert(
                    "username".to_string(),
                    serde_json::Value::String(user.username.clone()),
                );
            }
            "email" => {
                data.insert(
                    "email".to_string(),
                    serde_json::to_value(&user.email).unwrap_or(serde_json::Value::Null),
                );
            }
            "created_at" => {
                data.insert(
                    "created_at".to_string(),
                    serde_json::Value::String(user.created_at.to_string()),
                );
            }
            "is_banned" => {
                data.insert(
                    "is_banned".to_string(),
                    serde_json::Value::Bool(user.is_banned),
                );
            }
            "is_deleted" => {
                data.insert(
                    "is_deleted".to_string(),
                    serde_json::Value::Bool(user.is_deleted),
                );
            }
            _ => {}
        }
    }

    (
        StatusCode::OK,
        Json(UserDataResponse {
            success: true,
            data: Some(data),
            message: "User data retrieved".to_string(),
        }),
    )
}

pub async fn refresh_token(
    State((db, jwt_config)): State<(DB, JwtConfig)>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, Json<AuthTokenResponse>) {
    // Verify token
    let claims = match crate::utility::jwt::verify_token(&payload.refresh_token, &jwt_config) {
        Ok(claims) => claims,
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

    // Verify token is refresh type
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

    // Validate refresh token in database
    if let Err(_) = auth::validate_refresh_token(&db, &claims.sub, &payload.refresh_token).await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(AuthTokenResponse {
                success: false,
                user: None,
                tokens: None,
                message: "Refresh token not found or revoked".to_string(),
            }),
        );
    }

    // Generate new access token (reuse config)
    let access_token = match crate::utility::jwt::generate_access_token(
        &claims.sub,
        &claims.username,
        &jwt_config,
    ) {
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
                expires_in: jwt_config.access_expiry_minutes * 60,
            }),
            message: "Token refreshed successfully".to_string(),
        }),
    )
}

pub async fn logout(
    State((db, _jwt_config)): State<(DB, JwtConfig)>,
    Json(payload): Json<RefreshTokenRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    match auth::revoke_refresh_token(&db, &payload.refresh_token).await {
        Ok(_) => (
            StatusCode::OK,
            Json(AuthResponse {
                success: true,
                user: None,
                message: "Logged out successfully".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                success: false,
                user: None,
                message: format!("Failed to logout: {}", e),
            }),
        ),
    }
}
