//! This module is a shared module for both the public and private server binaries

use crate::cli::{self, registry::CommandDef};
use crate::db::init::{self, DB};
use crate::startup;
use crate::utility::config::Config;
use axum::Router;
use colored::Colorize;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};
use tokio::sync::watch;

type TestSuite = fn() -> Pin<Box<dyn Future<Output = ()> + Send>>;
type RouterBuilder = fn(DB) -> Router;

pub struct ServerOptions {
    /// `include_str!` of this binary's `initial.surql`
    pub schema: &'static str,
    /// This binary's `create_router` (Which the public binary adds)
    pub router: RouterBuilder,
    /// This binary's CLI commands, layered on top of the core ones
    pub extra_commands: &'static [CommandDef],
    /// The public server's test-suite hook; also gates the readiness wait
    pub run_tests: Option<TestSuite>,
    /// Public release builds refuse Ctrl+C and point operators at `server:shutdown`
    pub block_ctrl_c: bool,
}

/// Executes an embedded SurrealQL schema against the database.
///
/// Used at startup and by the `server:update-database` CLI command.
pub async fn init_schema(
    db: &DB,
    schema: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    db.query(schema).await?.check()?;
    Ok(())
}

/// Shared server lifecycle. Does not return while the server is running.
pub async fn run_server(options: ServerOptions) {
    print!("\x1B[2J\x1B[1;1H");

    startup::print_banner();
    startup::print_initializing();

    let db = match bootstrap(&options).await {
        Ok(db) => db,
        Err(_) => std::process::exit(1),
    };

    let config = Config::get();

    startup::print_ready(config.server_port);

    // Tracks true server uptime from this point forward, used by server:status and
    // server:shutdown. Intentionally created after all startup steps have completed.
    let server_start = Instant::now();

    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    let app = (options.router)(db.clone());
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

    let mut server_task = tokio::spawn(async move {
        server
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.changed().await;
            })
            .await
    });

    if options.run_tests.is_some() && !wait_for_listener(addr).await {
        eprintln!("{}", "Server failed to start in time.".red());
        std::process::exit(1);
    }

    if config.enable_testing
        && let Some(suite) = options.run_tests
    {
        suite().await;
    }

    cli::spawn_cli(
        options.extra_commands,
        db.clone(),
        server_start,
        shutdown_tx,
    )
    .await;

    if options.block_ctrl_c {
        loop {
            tokio::select! {
                _ = &mut server_task => break,
                result = tokio::signal::ctrl_c() => match result {
                    Ok(()) => {
                        println!("\nYou cannot shut down the server with Ctrl+C. Use server:shutdown.");
                    }
                    Err(_) => {
                        println!("{}", "Failed to listen for Ctrl+C".red());
                    }
                },
            }
        }
    } else {
        tokio::select! {
            _ = &mut server_task => {},
            _ = tokio::signal::ctrl_c() => {
                println!("\nShutting down...");
                let _ = crate::db::queries::server_logs::log_shutdown(
                    &db,
                    server_start.elapsed().as_millis() as i64,
                )
                .await;
            }
        }
    }
}

/// Loads config and database, then applies the schema.
///
/// Returns `Err(())` after printing the failed step; the caller exits with code 1.
async fn bootstrap(options: &ServerOptions) -> Result<DB, ()> {
    let timer = startup::create_timer();
    match Config::load() {
        Ok(_) => startup::print_step("Loading config", true, startup::elapsed(timer)),
        Err(e) => {
            startup::print_step("Loading config", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            return Err(());
        }
    }

    let timer = startup::create_timer();
    let db = match init::init().await {
        Ok(db) => {
            startup::print_step("Opening database", true, startup::elapsed(timer));
            db
        }
        Err(e) => {
            startup::print_step("Opening database", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            return Err(());
        }
    };

    let timer = startup::create_timer();
    match init_schema(&db, options.schema).await {
        Ok(_) => {
            startup::print_step("Initializing schema", true, startup::elapsed(timer));
            let _ = crate::db::queries::server_logs::log_startup(
                &db,
                startup::elapsed(timer).as_millis() as i64,
            )
            .await;
        }
        Err(e) => {
            startup::print_step("Initializing schema", false, startup::elapsed(timer));
            eprintln!("{}", format!("  Error: {}", e).red());
            let _ = crate::db::queries::server_logs::log_error(&db, e.to_string(), 0).await;
            return Err(());
        }
    }

    Ok(db)
}

/// Waits until the listener accepts TCP connections, replacing the old HTTP
/// health probe. Returns false if the server did not come up in time.
async fn wait_for_listener(addr: SocketAddr) -> bool {
    for _ in 0..20 {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    false
}
