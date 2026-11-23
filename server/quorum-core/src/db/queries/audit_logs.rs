//! File to handle logging audit events
//!
//! This module provides functionality to log audit events into the database.
//! Lookups for `log_type` and `audit_action_type` go through the shared
//! get-or-create cache in `super::type_records`, so each distinct log type and
//! action string is normalised into its lookup table and only queried once.
//!
//! `log_audit_event` is the only required public function here that is importable to other files.

use super::type_records::get_or_create_type;
use crate::db::init::DB;
use crate::models::server::AuditEvent;
use std::error::Error;
use surrealdb_types::{RecordId, RecordIdKey};

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
    let log_type_id = get_or_create_type(db, "log_type", &event.log_type).await?;

    let action_type_id = match &event.action {
        Some(a) => Some(get_or_create_type(db, "audit_action_type", a).await?),
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
