//! This is the main entry point for the Quorum Public server.
//!
//! To build/run the server use `cargo run -p quorum-public`

mod db;
mod models;
mod routes;
mod tests;
mod utility;

use crate::db::schema;
use crate::utility::docker;
use colored::Colorize;
use quorum_core::cli;
use quorum_core::db as core_db;
use quorum_core::startup;
use reqwest::Client;
use routes::route::create_router;
use std::net::SocketAddr;
use std::time::Duration;
use std::time::Instant;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

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

    startup::print_step(
        "Loading environment",
        true,
        startup::elapsed(startup::create_timer()),
    );

    // Load the config once and reuse it
    let config = quorum_core::utility::config::Config::get();

    // Load up and connect to the database
    let timer = startup::create_timer();
    let _db = match core_db::init().await {
        Ok(db) => {
            startup::print_step("Connecting to database", true, startup::elapsed(timer));
            db
        }
        Err(_) => {
            let docker_timer = startup::create_timer();
            match docker::ensure_containers_running().await {
                Ok(()) => {
                    startup::print_step("Starting Docker", true, startup::elapsed(docker_timer));

                    let retry_timer = startup::create_timer();
                    match core_db::init().await {
                        Ok(db) => {
                            startup::print_step(
                                "Connecting to database",
                                true,
                                startup::elapsed(retry_timer),
                            );
                            db
                        }
                        Err(e) => {
                            startup::print_step(
                                "Connecting to database",
                                false,
                                startup::elapsed(retry_timer),
                            );
                            eprintln!("{}", format!("  Error: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                Err(docker_err) => {
                    startup::print_step("Connecting to database", false, startup::elapsed(timer));
                    startup::print_step("Starting Docker", false, startup::elapsed(docker_timer));
                    eprintln!("{}", format!("  Error: {}", docker_err).red());
                    std::process::exit(1);
                }
            }
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

    let health_url = format!(
        "http://{}:{}/health",
        config.server_host, config.server_port
    );
    let client = Client::new();
    let mut is_ready = false;
    let max_retries = 20;
    let retry_delay = Duration::from_millis(50);

    for _ in 0..max_retries {
        match client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                is_ready = true;
                break;
            }
            Ok(_) => {
                tokio::time::sleep(retry_delay).await;
                continue;
            }
            Err(_) => {
                tokio::time::sleep(retry_delay).await;
                continue;
            }
        }
    }

    if !is_ready {
        eprintln!("{}", "Server failed to start in time.".red());
        std::process::exit(1);
    }

    // Run the tests if enabled
    if config.enable_testing {
        tests::run_all_tests().await;
    }

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
