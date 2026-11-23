//! File to handle logging server events
//!
//! This module defines functions to log different types of server events (startup, shutdown, errors) into the database.
//! Event type lookups go through the shared get-or-create cache in `super::type_records`,
//! so each event type is only queried from the database once per run.

use super::type_records::get_or_create_type;
use crate::db::init::DB;
use std::error::Error;

const EVENT_TYPE_TABLE: &str = "server_log_event_types";

/// Logs a startup event to the database with the given duration in milliseconds.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `duration_ms` - The duration of the startup event in milliseconds.
///
/// # Returns
/// * `Ok(())` - If the log entry was successfully created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if retrieving the event type ID fails.
///
/// # Example
/// ```
/// use crate::db::DB;
/// use crate::db::queries::server_logs;
/// async fn example_log_startup(db: &DB) {
///     let duration_ms = 1500;
///     match server_logs::log_startup(db, duration_ms).await {
///         Ok(()) => println!("Startup event logged successfully"),
///         Err(e) => eprintln!("Error logging startup event: {}", e),
///     }
/// }
/// ```
pub async fn log_startup(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
    log_duration_event(db, "startup", duration_ms).await
}

/// Logs a shutdown event to the database with the given duration in milliseconds.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `duration_ms` - The duration of the shutdown event in milliseconds.
///
/// # Returns
/// * `Ok(())` - If the log entry was successfully created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if retrieving the event type ID fails.
///
/// # Example
/// ```
/// use crate::db::DB;
/// use crate::db::queries::server_logs;
/// async fn example_log_shutdown(db: &DB) {
///     let duration_ms = 1200;
///     match server_logs::log_shutdown(db, duration_ms).await {
///         Ok(()) => println!("Shutdown event logged successfully"),
///         Err(e) => eprintln!("Error logging shutdown event: {}", e),
///     }
/// }
/// ```
pub async fn log_shutdown(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
    log_duration_event(db, "shutdown", duration_ms).await
}

/// Logs an error event to the database with the given message and error code.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `message` - A string describing the error message to log.
/// * `error_code` - A numeric code representing the error to log.
///
/// # Returns
/// * `Ok(())` - If the log entry was successfully created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if retrieving the event type ID fails.
///
/// # Example
/// ```
/// use crate::db::DB;
/// use crate::db::queries::server_logs;
/// async fn example_log_error(db: &DB) {
///     let message = "An unexpected error occurred".to_string();
///     let error_code = 500;
///     match server_logs::log_error(db, message, error_code).await {
///         Ok(()) => println!("Error event logged successfully"),
///         Err(e) => eprintln!("Error logging error event: {}", e),
///     }
/// }
/// ```
pub async fn log_error(
    db: &DB,
    message: String,
    error_code: u32,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let event_type_id = get_or_create_type(db, EVENT_TYPE_TABLE, "error").await?;

    db.query(
        "CREATE server_logs SET event_type_id = $event_type_id, message = $message, error_code = $error_code",
    )
        .bind(("event_type_id", event_type_id))
        .bind(("message", message))
        .bind(("error_code", error_code))
        .await?;

    Ok(())
}

/// Shared body for `log_startup` and `log_shutdown`, which only differ by event name.
async fn log_duration_event(
    db: &DB,
    event_name: &str,
    duration_ms: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let event_type_id = get_or_create_type(db, EVENT_TYPE_TABLE, event_name).await?;

    db.query("CREATE server_logs SET event_type_id = $event_type_id, duration_ms = $duration_ms")
        .bind(("event_type_id", event_type_id))
        .bind(("duration_ms", duration_ms))
        .await?;

    Ok(())
}
