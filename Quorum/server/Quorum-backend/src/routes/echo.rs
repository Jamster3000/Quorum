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

pub async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        echoed: payload.message,
    })
}