//! Entry point for the Quorum backend server.
//!
//! This module initializes the entire backend application, including:
//! - Loading environment variables from `.env`
//! - Establishing database connections to SurrealDB
//! - Loading and validating JWT configuration
//! - Initializing the database schema
//! - Warming up the password hasher
//! - Starting the Axum HTTP server
//! - Running functional tests (if enabled)
//!
//! The server listens on the configured `SERVER_PORT` (default: 3000) and coordinates
//! all application startup procedures with detailed console logging via the `startup` module.

mod db;
mod models;
mod routes;
mod startup;
mod tests;
mod utility;

use utility::docker;

use routes::route::create_router;

use colored::Colorize;
use std::net::SocketAddr;

/// Starts the backend server and initializes all required components.
///
/// Performs the following initialization steps in order:
/// 1. Loads environment variables from `.env` file
/// 2. Clears the terminal and displays banner
/// 3. Resolves the server port from `SERVER_PORT` environment variable (default: 3000)
/// 4. Establishes connection to SurrealDB
/// 5. Loads JWT configuration from environment variables
/// 6. Initializes database schema (creates tables if they don't exist)
/// 7. Warms up the Argon2 password hasher for optimal performance
/// 8. Starts the Axum HTTP server
/// 9. Optionally runs functional tests (controlled by `ENABLE_TESTS` env var)
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    //Clear the terminal and move the cursor to top left corner
    print!("\x1B[2J\x1B[1;1H");

    startup::print_banner(); //outputs the name using ASCII
    startup::print_initializing(); //output the initializing text

    //Load configs from .env
    let timer = startup::create_timer();
    match utility::config::Config::load() {
        Ok(config_load) => config_load,
        Err(e) => {
            startup::print_step("Loading config", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            std::process::exit(1);
        }
    };
    startup::print_step("Loading config", true, startup::elapsed_ms(timer));

    let timer = startup::create_timer();
    startup::print_step("Loading environment", true, startup::elapsed_ms(timer));

    //Load up and connect to the database
    let timer = startup::create_timer();
    let _db = match db::init().await {
        Ok(db) => {
            startup::print_step("Connecting to database", true, startup::elapsed_ms(timer));
            db
        }
        Err(_) => {
            let docker_timer = startup::create_timer();
            match docker::ensure_containers_running().await {
                Ok(()) => {
                    startup::print_step("Starting Docker", true, startup::elapsed_ms(docker_timer));

                    let retry_timer = startup::create_timer();
                    match db::init().await {
                        Ok(db) => {
                            startup::print_step(
                                "Connecting to database",
                                true,
                                startup::elapsed_ms(retry_timer),
                            );
                            db
                        }
                        Err(e) => {
                            startup::print_step(
                                "Connecting to database",
                                false,
                                startup::elapsed_ms(retry_timer),
                            );
                            eprintln!("{}", format!("  Error: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                Err(docker_err) => {
                    startup::print_step(
                        "Connecting to database",
                        false,
                        startup::elapsed_ms(timer),
                    );
                    startup::print_step(
                        "Starting Docker",
                        false,
                        startup::elapsed_ms(docker_timer),
                    );
                    eprintln!("{}", format!("  Error: {}", docker_err).red());
                    std::process::exit(1);
                }
            }
        }
    };

    //Write the tables to the database if they don't exist (initial.squrl)
    let timer = startup::create_timer();
    match db::schema::init(&_db).await {
        Ok(_) => {
            startup::print_step("Initializing schema", true, startup::elapsed_ms(timer));
            let _ = db::queries::server_logs::log_startup(&_db, startup::elapsed_ms(timer) as i64)
                .await;
        }
        Err(e) => {
            startup::print_step("Initializing schema", false, startup::elapsed_ms(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            let _ = db::queries::server_logs::log_error(&_db, e.to_string(), 0).await;
            std::process::exit(1);
        }
    }

    startup::print_ready(utility::config::Config::get().server_port);

    //start the router
    let app = create_router(_db.clone());
    let host = utility::config::Config::get()
        .server_host
        .parse::<std::net::IpAddr>()
        .expect("Invalid SERVER_HOST IP address");
    let addr = SocketAddr::from((host, utility::config::Config::get().server_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    //run the tests to ensure the server is fully functional without errors
    if utility::config::Config::get().enable_testing {
        //spawn server in background
        //to ensure the server runs smoothly at the same time as running the tests
        //the sever is spawned in its own thread whilst the tests run on main thread
        let server_task = tokio::spawn(async move { server.await });

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        tests::run_all_tests().await;

        //Prettify the Ctrl+C shutdown and log the shutdown
        let shutdown = async {
            let _ = tokio::signal::ctrl_c().await;
        };

        tokio::select! {
            _ = server_task => {},
            _ = shutdown => {
                println!("\nShutting down...");
                let _ = db::queries::server_logs::log_shutdown(&_db, startup::elapsed_ms(timer) as i64).await;
            }
        }
    } else {
        let shutdown = async {
            let _ = tokio::signal::ctrl_c().await;
        };

        tokio::select! {
            _ = server => {},
            _ = shutdown => {
                println!("\nShutting down...");
                let _ = db::queries::server_logs::log_shutdown(&_db, startup::elapsed_ms(timer) as i64).await;
            }
        }
    }
}
