//! This module defines the routes for the Axum server, including authentication and health check endpoints.

use crate::db::DB;
use crate::utility::jwt::JwtConfig;
use axum::{
    Router,
    routing::{get, post},
};

use super::auth::{delete_account, get_user_data, login, logout, refresh_token, signup};
use super::echo::echo;
use super::health::health;

/// Creates the router with all the defined routes and their handlers.
///
/// # Arguments
/// * `db` - The database connection pool.
/// * `jwt_config` - The JWT configuration for authentication.
///
/// # Returns
/// A configured `Router` instance with all routes and their handlers.
/// 
/// # Example
/// ```rust
/// let db = DB::new("mongodb://localhost:27017").await.unwrap();
/// let jwt_config = JwtConfig::new
///     .with_secret("your_secret_key");
/// let router = create_router(db, jwt_config);
/// ```
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
