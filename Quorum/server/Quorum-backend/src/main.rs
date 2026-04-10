mod db;
mod startup;

use axum::{
    routing::{get, post},
    Json, Router,
};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

//structs for request and response payloads
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
    dotenvy::dotenv().ok();

    startup::print_banner();
    startup::print_initializing();

    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16");

    let timer = startup::create_timer();
    startup::print_step("Loading environment", true, startup::elapsed_ms(timer));

    let timer = startup::create_timer();
    match db::init().await {
        Ok(_db) => {
            startup::print_step("Connecting to database", true, startup::elapsed_ms(timer));
            
            let timer = startup::create_timer();
            match db::schema::init(&_db).await {
                Ok(_) => {
                    startup::print_final_step("Initializing schema", true, startup::elapsed_ms(timer));
                }
                Err(e) => {
                    startup::print_final_step("Initializing schema", false, startup::elapsed_ms(timer));
                    eprintln!("{}", format!("  Error: {}", e).red());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            startup::print_step("Connecting to database", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    }

    startup::print_ready(port);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/echo", post(echo));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Axum server is running"
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        echoed: payload.message,
    })
}