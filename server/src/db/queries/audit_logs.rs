//! File to handle logging audit events
//!
//! This module provides functionality to log audit events into the database. It includes caching mechanisms
//! for log types and action types to optimize performance and reduce database queries.
//!
//! `log_audit_event` is the only required public function here that is importable to other files.

use crate::db::DB;
use crate::models::server::AuditEvent;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Mutex, OnceLock};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};

static LOG_TYPE_CACHE: OnceLock<Mutex<HashMap<String, RecordId>>> = OnceLock::new();
static ACTION_TYPE_CACHE: OnceLock<Mutex<HashMap<String, RecordId>>> = OnceLock::new();

#[derive(Debug, SurrealValue)]
struct TypeRecord {
    id: RecordId,
}

/// Logs an audit event.
///
/// This logs an audit event to the database, which are used to keep a log and record
/// of things that happen on the server (be it directly in the server or through API calls interacting with the server).
///
/// The `action` field in `AuditEvent` uses normalisation, so repeated `action` strings are instead referenced in the `audit_action_type` table,
/// rather than repeating the same string across multiple audit log entries.
///
/// The `target_type_table` and `target_type_table_id` fields are used to reference a specific row in the database by searching the
/// `target_type_table_id` inside of the `target_type_table` table. Due to how SurrealDB works, the database only needs that `RecordId`
/// as that includes both the table and id. `target_type_table` is only needed by this function to know what table `target_type_table_id` is pointing towards.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `event` - An `AuditEvent` struct containing the fields for the audit log entry.
///   Only `log_type` is required — all other fields default to `None` via `..Default::default()`.
///
/// # Returns
/// * `Ok(())` - If the audit event was successfully logged.
/// * `Err(Box<dyn Error>)` - If the logging operation failed.
///
/// # Examples
/// ```rust
/// // Full example with all fields
/// use models::server::AuditEvent;
///
/// let _ = db::queries::audit_logs::log_audit_event(
///     &db,
///     AuditEvent {
///         log_type: "server_event".to_string(),
///         action: Some("Server startup".to_string()),
///         target_type_table: Some("server_logs".to_string()),
///         target_type_table_id: Some("1pen66hlvglaf46q9q8k".to_string()),
///         new_value: Some("Server started successfully".to_string()),
///         old_value: Some("Server was not running".to_string()),
///         user_id: Some("zd0wx5u17prfcw4hn2uf".to_string()),
///     }
/// ).await;
/// ```
///
/// ```rust
/// // Minimal example with only required parameters
/// use models::server::AuditEvent;
///
/// let _ = db::queries::audit_logs::log_audit_event(
///     &db,
///     AuditEvent {
///         log_type: "server_event".to_string(),
///         user_id: "zd0wx5u17prfcw4hn2uf".to_string(),
///         ..Default::default()
///     }
/// ).await;
/// ```
pub async fn log_audit_event(
    db: &DB,
    event: AuditEvent,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let log_type_id = get_cached_log_type(db, &event.log_type).await?;

    let action_type_id = match &event.action {
        Some(a) => Some(get_cached_action_type(db, a).await?),
        None => None,
    };

    let user_record_id = event.user_id.map(|id| RecordId {
        table: "users".into(),
        key: RecordIdKey::String(id),
    });

    let target_table_id = match (event.target_type_table, event.target_type_table_id) {
        (Some(table), Some(id)) => Some(RecordId {
            table: table.into(),
            key: RecordIdKey::String(id),
        }),
        _ => None,
    };

    let _response = db
        .query(
            "CREATE audit_logs SET
                log_type_id = $log_type_id,
                action_type_id = $action_type_id,
                target_type_table_id = $target_type_table_id,
                new_value = $new_value,
                old_value = $old_value,
                user_id = $user_id",
        )
        .bind(("log_type_id", log_type_id))
        .bind(("action_type_id", action_type_id))
        .bind(("target_type_table_id", target_table_id))
        .bind(("new_value", event.new_value))
        .bind(("old_value", event.old_value))
        .bind(("user_id", user_record_id))
        .await?
        .check()?;

    Ok(())
}

/// Retrieves the cached log type ID for a given log name, or creates it if it doesn't exist.
///
/// This function checks the `LOG_TYPE_CACHE` for the specified `log_name`.
/// If it exists, it returns the cached `RecordId`. If not, it queries the database to find or create the log type and updates the cache accordingly.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `log_name` - The name of the log type to retrieve or create.
///
/// # Returns
/// A `RecordId` representing the log type ID associated with the given `log_name`.
async fn get_cached_log_type(
    db: &DB,
    log_name: &str,
) -> Result<RecordId, Box<dyn Error + Send + Sync>> {
    let cache = LOG_TYPE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if let Some(id) = cache.lock().unwrap().get(log_name).cloned() {
        return Ok(id);
    }

    let id = get_or_create_type(db, "log_type", log_name).await?;
    cache
        .lock()
        .unwrap()
        .insert(log_name.to_string(), id.clone());
    Ok(id)
}

/// Retrieves the cached action type ID for a given log name, or creates it if it doesn't exist.
///
/// This function checks the `ACTION_TYPE_CACHE` for the specified `action_name`.
/// If it exists, it returns the cached `RecordId`. If not, it queries the database to find or create the action type and updates the cache accordingly.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `action_name` - The name of the action type to retrieve or create.
///
/// # Returns
/// A `RecordId` representing the action type ID associated with the given `action_name`.
async fn get_cached_action_type(
    db: &DB,
    action_name: &str,
) -> Result<RecordId, Box<dyn Error + Send + Sync>> {
    let cache = ACTION_TYPE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if let Some(id) = cache.lock().unwrap().get(action_name).cloned() {
        return Ok(id);
    }

    let id = get_or_create_type(db, "audit_action_type", action_name).await?;
    cache
        .lock()
        .unwrap()
        .insert(action_name.to_string(), id.clone());
    Ok(id)
}

/// Retrieves or creates a type record in the specified table based on the given name.
///
/// This function checks if a record with the specified name exists in the given table.
/// If it exists, it returns the `RecordId`. If not, it creates a new record with that name and returns the new `RecordId`.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `table` - The name of the table to query or insert into (e.g., "log_type" or "audit_action_type").
/// * `name` - The name of the type to retrieve or create.
///
/// # Returns
/// A `RecordId` representing the ID of the existing or newly created type record.
async fn get_or_create_type(
    db: &DB,
    table: &str,
    name: &str,
) -> Result<RecordId, Box<dyn Error + Send + Sync>> {
    let query = format!("SELECT id FROM {} WHERE name = $name LIMIT 1", table);
    let response = db.query(&query).bind(("name", name)).await?.check()?;

    let mut response = response;
    let result: Option<TypeRecord> = response.take(0)?;

    if let Some(record) = result {
        return Ok(record.id);
    }

    let create_query = format!("CREATE {} SET name = $name RETURN id", table);
    let create_response = db
        .query(&create_query)
        .bind(("name", name))
        .await?
        .check()?;

    let mut create_response = create_response;
    let created: Option<TypeRecord> = create_response.take(0)?;
    Ok(created.ok_or("Failed to create type record")?.id)
}
