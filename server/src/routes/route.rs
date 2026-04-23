use crate::db::DB;
use crate::utility::jwt::JwtConfig;
use axum::{
    Router,
    routing::{get, post},
};

use super::auth::{delete_account, get_user_data, login, logout, refresh_token, signup};
use super::echo::echo;
use super::health::health;

pub fn create_router(db: DB, jwt_config: JwtConfig) -> Router {
    Router::new()
        .route("/", get(|| async { "Axum server is running" }))
        .route("/health", get(health))
        .route("/echo", post(echo))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/delete", post(delete_account))
        .route("/auth/me", post(get_user_data))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))
        .with_state((db, jwt_config))
}
