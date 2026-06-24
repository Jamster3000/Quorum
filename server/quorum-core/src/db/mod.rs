//! Database initialization and connection management.
//!
//! This module handles establishing connections to SurrealDB using WebSocket protocol.
//! It loads database credentials and configuration from environment variables and provides
//! helpful error messages if connection or authentication fails.

use crate::utility::config::Config;
use std::error::Error;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

pub mod queries;

pub type DB = Surreal<surrealdb::engine::remote::ws::Client>;

/// Initializes and authenticates a connection to SurrealDB.
/// Loads database connection parameters from the centralized config, establishes a WebSocket
/// connection to SurrealDB, authenticates with root credentials, and selects the target namespace
/// and database. If any step fails, returns a descriptive error message with helpful troubleshooting hints.
///
/// # Returns
/// * `Ok(DB)` - A connected and authenticated SurrealDB client ready for queries
/// * `Err(Box<dyn Error>)` - An error with a descriptive message and troubleshooting hint
///
/// # Errors
/// * Connection refused - SurrealDB is not running or the port is blocked
/// * Cannot resolve hostname - The `SURREAL_URL` is invalid or DNS resolution failed
/// * Authentication failed - `SURREAL_USER` or `SURREAL_PASS` credentials are incorrect
/// * Database selection failed - The namespace or database name does not exist
///
/// # Example
/// ```rust
/// use crate::db;
///
/// #[tokio::main]
/// async fn main() {
///     let db = db::init().await.expect("Failed to connect to database");
/// }
/// ```
pub async fn init() -> Result<DB, Box<dyn Error>> {
    let config = Config::get();
    let url = &config.surreal_url;
    let user = &config.surreal_user;
    let pass = &config.surreal_pass;
    let ns = &config.surreal_ns;
    let db_name = &config.surreal_db;

    let db = Surreal::new::<Ws>(url).await.map_err(|e| -> Box<dyn Error> {
        let error_msg = e.to_string();

        let hint = if error_msg.contains("10061") || error_msg.contains("actively refused") {
            "Hint: Connection refused. Is SurrealDB running?\n       Run: `docker compose up -d`"
        } else if error_msg.contains("No such host") || error_msg.contains("11001") {
            "Hint: Cannot resolve hostname. Check your SURREAL_URL in .env"
        } else if error_msg.contains("timeout") {
            "Hint: Connection timeout. Is the firewall blocking port 8000?"
        } else {
            "Hint: Check that SurrealDB is accessible at the URL in .env"
        };

        format!("Failed to connect to SurrealDB at {}\n{}\n\nError: {}", url, hint, e).into()
    })?;

    db.signin(Root {
        username: user.to_string(),
        password: pass.to_string(),
    })
    .await
    .map_err(|e| -> Box<dyn Error> {
        let hint = "Hint: Check SURREAL_USER and SURREAL_PASS in .env match docker-compose.yml";
        format!("Failed to authenticate\n{}\n\nError: {}", hint, e).into()
    })?;

    db.use_ns(ns).use_db(db_name).await.map_err(|e| -> Box<dyn Error> {
        let hint = "Hint: Verify SURREAL_NS and SURREAL_DB in .env exist. Run schema/initial.surql to create them.";
        format!("Failed to select database\n{}\n\nError: {}", hint, e).into()
    })?;

    Ok(db)
}
