//! This module defines the routes for the Axum server, including authentication and health check endpoints.

use crate::db::DB;
use crate::utility::config::Config;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use std::time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use super::auth::{delete_account, get_user_data, login, logout, refresh_token, signup, update_user_profile};
use super::echo::echo;
use super::health::health;

/// Creates the router with all the defined routes and their handlers.
/// Includes request limiting to avoid getting too many requests at once.
///
/// # Arguments
/// * `db` - The database connection pool.
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
pub fn create_router(db: DB) -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(Config::get().default_per_second)
            .burst_size(Config::get().default_burst_size)
            .finish()
            .unwrap(),
    );

    let limiter = governor_conf.limiter().clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(60));
            limiter.retain_recent();
        }
    });

    let auth_routes = Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/delete", post(delete_account))
        .route("/auth/me", post(get_user_data))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))
        .route("/auth/updateuserprofile", post(update_user_profile));

    let auth_routes = if Config::get().enable_testing {
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_second(Config::get().testing_per_second)
                .burst_size(Config::get().testing_burst_size)
                .finish()
                .unwrap(),
        );
        let limiter = governor_conf.limiter().clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_secs(60));
                limiter.retain_recent();
            }
        });
        auth_routes.layer(GovernorLayer::new(governor_conf))
    } else {
        auth_routes
    };

    Router::new()
        .route("/", get(|| async { "Axum server is running" }))
        .route("/health", get(health))
        .route("/echo", post(echo))
        .merge(auth_routes)
        .with_state(db)
}
