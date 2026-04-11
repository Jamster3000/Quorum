use axum::{
    Router,
    routing::{get, post},
};

use super::echo::echo;
use super::health::health;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Axum server is running" }))
        .route("/health", get(health))
        .route("/echo", post(echo))
}
