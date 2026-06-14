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

use super::auth::{
    delete_account, get_user_data, login, logout, refresh_token, signup, update_user_profile,
};
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
pub fn create_router(db: DB) -> Router {
    Router::new()
        .route("/", get(|| async { "Axum server is running" }))
        .route("/health", get(health))
        .route("/echo", post(echo))
        .merge(auth_routes())
        .with_state(db)
}

/// Builds the auth route group with an appropriate rate limiter.
///
/// In testing mode, a more permissive rate limit is applied so the test suite
/// can run without hitting the default limits.
fn auth_routes() -> Router<DB> {
    let config = Config::get();

    let (per_second, burst_size) = if config.enable_testing {
        (config.testing_per_second, config.testing_burst_size)
    } else {
        (config.default_per_second, config.default_burst_size)
    };

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(per_second)
            .burst_size(burst_size)
            .finish()
            .unwrap(),
    );

    // Periodically evict expired rate limit entries to keep memory bounded
    let limiter = governor_conf.limiter().clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(60));
        limiter.retain_recent();
    });

    Router::new()
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/delete", post(delete_account))
        .route("/auth/me", post(get_user_data))
        .route("/auth/refresh", post(refresh_token))
        .route("/auth/logout", post(logout))
        .route("/auth/updateuserprofile", post(update_user_profile))
        .layer(GovernorLayer::new(governor_conf))
}