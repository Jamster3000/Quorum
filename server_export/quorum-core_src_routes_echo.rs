//! A simple echo route that returns the message sent in the request.
//! THis is useful for testing that the server responds to a given request and that the request is properly formatted.

use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub echoed: String,
}

/// Echoes back the message sent in the request.
///
/// # Arguments
/// * `payload` - The JSON payload containing the message to be echoed.
///
/// # Returns
/// A JSON response containing the echoed message.
///
/// # Example
/// ```
/// use axum::Json;
/// use serde_json::json;
///
/// let payload = Json(json!({ "message": "Hello, world!" }));
/// let response = echo(payload).await;
/// assert_eq!(response.echoed, "Hello, world!");
/// ```
pub async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        echoed: payload.message,
    })
}
