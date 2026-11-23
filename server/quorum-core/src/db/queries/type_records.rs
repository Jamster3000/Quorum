//! Shared get-or-create helpers for name-keyed lookup tables.
//!
//! `log_type`, `audit_action_type` and `server_log_event_types` all follow the
//! same pattern: one row per name, referenced by `RecordId` from log entries.
//! A single process-wide cache keeps each (table, name) pair to one database
//! hit for the lifetime of the server.

use crate::db::init::DB;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Mutex, OnceLock};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, SurrealValue)]
struct TypeRecord {
    id: RecordId,
}

static CACHE: OnceLock<Mutex<HashMap<(&'static str, String), RecordId>>> = OnceLock::new();

/// Returns the `RecordId` of the row with this name in the lookup table,
/// creating it if it doesn't exist. The result is cached in-process.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `table` - The lookup table to query (e.g. `"log_type"`, `"server_log_event_types"`).
/// * `name` - The name to look up or create.
///
/// # Returns
/// A `RecordId` representing the row for the given name.
pub(crate) async fn get_or_create_type(
    db: &DB,
    table: &'static str,
    name: &str,
) -> Result<RecordId, Box<dyn Error + Send + Sync>> {
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if let Some(id) = cache
        .lock()
        .unwrap()
        .get(&(table, name.to_string()))
        .cloned()
    {
        return Ok(id);
    }

    let id = get_or_create_in_db(db, table, name).await?;
    cache
        .lock()
        .unwrap()
        .insert((table, name.to_string()), id.clone());
    Ok(id)
}

/// Looks the name up in the database, creating the row if it is missing.
async fn get_or_create_in_db(
    db: &DB,
    table: &str,
    name: &str,
) -> Result<RecordId, Box<dyn Error + Send + Sync>> {
    let select = format!("SELECT id FROM {table} WHERE name = $name LIMIT 1");
    let mut response = db.query(&select).bind(("name", name)).await?.check()?;

    if let Some(record) = response.take::<Option<TypeRecord>>(0)? {
        return Ok(record.id);
    }

    let create = format!("CREATE {table} SET name = $name RETURN id");
    let mut create_response = db.query(&create).bind(("name", name)).await?.check()?;

    create_response
        .take::<Option<TypeRecord>>(0)?
        .ok_or_else(|| "Failed to create type record".into())
        .map(|record| record.id)
}
