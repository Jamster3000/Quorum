//! File to handle logging server events
//!
//! This module defines functions to log different types of server events (startup, shutdown, errors) into the database.
//! It uses a caching mechanism to minimize database queries for event type IDs, ensuring efficient logging.

use crate::db::DB;
use std::error::Error;
use std::sync::OnceLock;
use surrealdb_types::{RecordId, SurrealValue};

static STARTUP_ID: OnceLock<RecordId> = OnceLock::new();
static SHUTDOWN_ID: OnceLock<RecordId> = OnceLock::new();
static ERROR_ID: OnceLock<RecordId> = OnceLock::new();

#[derive(Debug, SurrealValue)]
struct EventTypeRecord {
    id: RecordId,
}

/// Gets the event type ID for the given event name, creating a new event type if it doesn't already exist.
///
/// This function performs a database query to check if the event type already exists. If it does, it returns the existing ID.
/// If it doesn't exist, it creates a new event type and returns the new ID.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `event_name` - The name of the event type to get or create.
///
/// # Returns
/// * `Ok(RecordId)` - The ID of the event type, either existing or newly created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if creating a new event type fails.
///
/// # Errors
/// * "Failed to create event type" - If the database query to create a new event type does not return a valid record, indicating that the creation failed.
async fn get_or_create_event_type(db: &DB, event_name: String) -> Result<RecordId, Box<dyn Error>> {
    //Check if the event type already exists by trying to get it.
    let mut response = db
        .query("SELECT id FROM server_log_event_types WHERE name = $name")
        .bind(("name", event_name.clone()))
        .await?;

    let result: Option<EventTypeRecord> = response.take(0)?;

    //If event type exists already in `server_log_event_types` return its ID
    if let Some(record) = result {
        Ok(record.id)
    } else {
        //event type doesn't exist, create it then return its ID
        let mut create_response = db
            .query("CREATE server_log_event_types SET name = $name RETURN id")
            .bind(("name", event_name))
            .await?;

        let created: Option<EventTypeRecord> = create_response.take(0)?;
        Ok(created.ok_or("Failed to create event type")?.id)
    }
}

/// Gets the cached event type ID for the given event name, using a OnceLock to cache the ID after the first retrieval.
///
/// This function first checks the provided OnceLock cache for the event type ID. If the ID is already cached, it returns it immediately.
/// If the ID is not cached, it calls `get_or_create_event_type` to retrieve or create the event type ID from the database, caches it in the OnceLock, and then returns it.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `cache` - A reference to a OnceLock that will cache the event type ID after the first retrieval.
/// * `event_name` - The name of the event type to get or create.
///
/// # Returns
/// * `Ok(RecordId)` - The ID of the event type, either retrieved from the cache or obtained from the database.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if creating a new event type fails.
///
/// # Errors
/// * "Failed to create event type" - If the database query to create a new event type does not return a valid record, indicating that the creation failed.
async fn get_cached_event_type(
    db: &DB,
    cache: &OnceLock<RecordId>,
    event_name: &str,
) -> Result<RecordId, Box<dyn Error>> {
    if let Some(id) = cache.get() {
        return Ok(id.clone());
    }

    let id = get_or_create_event_type(db, event_name.to_string()).await?;
    let _ = cache.set(id.clone());
    Ok(id)
}

/// Logs a startup event to the database with the given duration in milliseconds.
///
/// This function retrieves the event type ID for "startup" using the caching mechanism and then creates a new log entry in the `server_logs` table with the event type ID and duration.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `duration_ms` - The duration of the startup event in milliseconds.
///
/// # Returns
/// * `Ok(())` - If the log entry was successfully created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if retrieving the event type ID fails.
///
/// # Errors
/// * "Failed to create event type" - If the database query to create a new event type does not return a valid record, indicating that the creation failed.
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
pub async fn log_startup(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_cached_event_type(db, &STARTUP_ID, "startup").await?;

    db.query("CREATE server_logs SET event_type_id = $event_type_id, duration_ms = $duration_ms")
        .bind(("event_type_id", event_type_id))
        .bind(("duration_ms", duration_ms))
        .await?;

    Ok(())
}

/// Logs a shutdown event to the database with the given duration in milliseconds.
///
/// This function retrieves the event type ID for "shutdown" using the caching mechanism and then creates a new log entry in the `server_logs` table with the event type ID and duration.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `duration_ms` - The duration of the shutdown event in milliseconds.
///
/// # Returns
/// * `Ok(())` - If the log entry was successfully created.
/// * `Err(Box<dyn Error>)` - An error if the database query fails or if retrieving the event type ID fails.
///
/// # Errors
/// * "Failed to create event type" - If the database query to create a new event type does not return a valid record, indicating that the creation failed.
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
pub async fn log_shutdown(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_cached_event_type(db, &SHUTDOWN_ID, "shutdown").await?;

    db.query("CREATE server_logs SET event_type_id = $event_type_id, duration_ms = $duration_ms")
        .bind(("event_type_id", event_type_id))
        .bind(("duration_ms", duration_ms))
        .await?;

    Ok(())
}

/// Logs an error event to the database with the given message and error code.
///
/// This function retrieves the event type ID for "error" using the caching mechanism and then creates a new log entry in the `server_logs` table with the event type ID, message, and error code.
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
/// # Errors
/// * "Failed to create event type" - If the database query to create a new event type does not return a valid record, indicating that the creation failed.
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
pub async fn log_error(db: &DB, message: String, error_code: u32) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_cached_event_type(db, &ERROR_ID, "error").await?;

    db.query(
        "CREATE server_logs SET event_type_id = $event_type_id, message = $message, error_code = $error_code",
    )
    .bind(("event_type_id", event_type_id))
    .bind(("message", message))
    .bind(("error_code", error_code))
    .await?;

    Ok(())
}
