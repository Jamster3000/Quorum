use serde::{Deserialize, Serialize};
use surrealdb_types::{Datetime, RecordId, SurrealValue};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct User {
    pub id: RecordId,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: Datetime,
    pub last_login: Option<Datetime>,
    pub is_banned: bool,
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteAccountRequest {
    pub username_or_email: String,
    pub password: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetUserDataRequest {
    pub user_id: String,
    pub username_or_email: String,
    pub password: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UserDataResponse {
    pub success: bool,
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub created_at: Datetime,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct AuthTokenResponse {
    pub success: bool,
    pub user: Option<UserResponse>,
    pub tokens: Option<TokenResponse>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub user_id: String,
    pub email: String,
    pub username: String,
}

impl User {
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: format!("{:?}", self.id.key),
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at,
        }
    }
}
