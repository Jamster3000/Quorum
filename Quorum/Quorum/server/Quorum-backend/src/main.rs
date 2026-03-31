use axum:: {
	routing::{get, post},
	Json, Router,
};
use serde::{ Deserialize, Serialize };
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
	status: &'static str,
}

#[derive(Deserialize)]
struct EchoRequest {
	message: String,
}

#[derive(Serialize)]
struct EchoResponse {
	echoed: String,
}

#[tokio::main]
async fn main() {
	let app = Router::new()
	    .route("/", get(root))
		.route("/health", get(health))
		.route("/echo", post(echo));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

	println!("Listening on http://{}", addr);
	axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
	"Axum server is runnign"
}

async fn health() -> Json<HealthResponse> {
	Json(HealthResponse { status: "ok" })
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
	Json(EchoResponse {
		echoed: payload.message,
	})
}

