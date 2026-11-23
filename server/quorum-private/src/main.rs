//! This is the main entry point for the Quorum Private server.
//!
//! It sets up the server similar to the public server, tweaked to work with being self hostable by anyone.
//! Use `cargo run -p quorum-private` to build/run the private version of the server, which combines the `quorum-core` shared code.

mod cli;
mod db;
mod routes;

//use reqwest::Client;
use crate::db::schema;
use colored::Colorize;
use quorum_core::db as core_db;
use quorum_core::startup;
use routes::route::create_router;
use std::net::SocketAddr;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Clear the terminal and move the cursor to top left corner
    print!("\x1B[2J\x1B[1;1H");

    startup::print_banner();
    startup::print_initializing();

    // Load configs from .env (only once)
    let timer = startup::create_timer();
    match quorum_core::utility::config::Config::load() {
        Ok(_) => startup::print_step("Loading config", true, startup::elapsed(timer)),
        Err(e) => {
            startup::print_step("Loading config", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    };

    // Load the config once and reuse it
    let config = quorum_core::utility::config::Config::get();

    // Load up and connect to the database
    let timer = startup::create_timer();
    let _db = match core_db::init::init().await {
        Ok(db) => {
            startup::print_step("Opening database", true, startup::elapsed(timer));
            db
        }
        Err(e) => {
            startup::print_step("Opening database", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    };

    // Write the tables to the database if they don't exist
    let timer = startup::create_timer();
    match schema::init(&_db).await {
        Ok(_) => {
            startup::print_step("Initializing schema", true, startup::elapsed(timer));
            let _ = core_db::queries::server_logs::log_startup(
                &_db,
                startup::elapsed(timer).as_millis() as i64,
            )
            .await;
        }
        Err(e) => {
            startup::print_step("Initializing schema", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            let _ = core_db::queries::server_logs::log_error(&_db, e.to_string(), 0).await;
            std::process::exit(1);
        }
    }

    startup::print_ready(config.server_port);

    // Tracks true server uptime from this point forward, used by server:status and server:shutdown.
    // Intentionally created after all startup steps have completed.
    let server_start = Instant::now();

    // Shutdown channel — CLI sends true on server:shutdown, main select! receives it
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel(false);

    // Start the router
    let app = create_router(_db.clone());
    let host = config
        .server_host
        .parse::<std::net::IpAddr>()
        .expect("Invalid SERVER_HOST IP address");
    let addr = SocketAddr::from((host, config.server_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    let server_task = tokio::spawn(async move {
        server
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.changed().await;
            })
            .await
    });

    // Start the CLI input loop in a background task
    cli::spawn_cli(_db.clone(), server_start, shutdown_tx).await;

    let shutdown = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    tokio::select! {
        _ = server_task => {},
        _ = shutdown => {
            println!("\nShutting down...");
            let _ = quorum_core::db::queries::server_logs::log_shutdown(&_db, server_start.elapsed().as_millis() as i64).await;
        }
    }
}
