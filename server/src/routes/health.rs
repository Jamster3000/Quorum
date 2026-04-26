//! Health check endpoint.
//! This endpoint is used to check if the server is running and healthy.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Health check endpoint handler.
/// This endpoint returns a JSON response with the status of the server.
///
/// # Returns
/// A JSON response with the status of the server.
///
/// # Example
/// ```
/// use axum::Json;
/// 
/// let response = health().await;
/// assert_eq!(response.status, "ok");
/// ```
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
