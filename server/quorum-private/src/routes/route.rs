//! Routes for the public Quorum server.

use axum::{
    Router,
    routing::{get, post},
};
use quorum_core::db::DB;
use quorum_core::routes::{echo::echo, health::health};
use quorum_core::utility::config::Config;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use http::header::{HeaderName, HeaderValue};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn create_router(db: DB) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let security_headers = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ));

    Router::new()
        .route("/", get(|| async { "Quorum public server is running" }))
        .route("/health", get(health))
        .route("/echo", post(echo))
        .layer(cors)
        .layer(security_headers)
        .with_state(db)
}
