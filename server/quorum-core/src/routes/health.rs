//! Health check endpoint.
//! This endpoint is used to check if the server is running and healthy.

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub critical: Option<&'static str>,
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
    //roll random number, return the number IF it's 20
    //generate a number between 1 and 20

    let random_roll = rand::random::<u8>() % 20 + 1;

    if random_roll == 20 {
        return Json(HealthResponse {
            status: "ok",
            critical: Some("That's a 20. Critical Success!"),
        });
    }

    Json(HealthResponse {
        status: "ok",
        critical: None,
    })
}
