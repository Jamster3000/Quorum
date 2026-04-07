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

#[derive(Deserialize)]
struct SearchRequest {
	query: String,
}

#[derive(Serialize)]
struct SearchResponse {
	query: String,
	results: String,
}

async fn search(Json(payload): Json<SearchRequest>) -> Json<SearchResponse> {
	let results;
	let userQuery = payload.query.to_lowercase();

	if (userQuery == "test") {
		results = "If you are seeing this message, it works".to_string();
	} else {
		results = "No results found".to_string();
	}

	Json(SearchResponse {
		query: payload.query,
		results,
	})
}

#[tokio::main]
async fn main() {
	let app = Router::new()
	    .route("/", get(root))
		.route("/health", get(health))
		.route("/echo", post(echo))
		.route("/search", post(search));

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

