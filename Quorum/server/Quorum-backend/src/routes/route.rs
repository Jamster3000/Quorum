use axum::{routing::{get, post}, Router};

use super::health::health;
use super::echo::echo;

pub fn create_router() -> Router {
	Router::new()
        .route("/", get(|| async { "Axum server is running" }))
		.route("/health", get(health))
		.route("/echo", post(echo))
}