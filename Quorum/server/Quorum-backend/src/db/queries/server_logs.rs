use crate::db::DB;
use std::error::Error;
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, SurrealValue)]
struct EventTypeRecord {
    id: RecordId,
}

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
        //event type doesn't exist, create it then return it's ID
        let mut create_response = db
            .query("CREATE server_log_event_types SET name = $name RETURN id")
            .bind(("name", event_name))
            .await?;

        let created: Option<EventTypeRecord> = create_response.take(0)?;
        Ok(created.ok_or("Failed to create event type")?.id)
    }
}

pub async fn log_startup(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_or_create_event_type(db, "startup".to_string()).await?;

    db.query(
        "CREATE server_logs SET event_type_id = $event_type_id, duration_ms = $duration_ms",
    )
    .bind(("event_type_id", event_type_id))
    .bind(("duration_ms", duration_ms))
    .await?;

    Ok(())
}

pub async fn log_shutdown(db: &DB, duration_ms: i64) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_or_create_event_type(db, "shutdown".to_string()).await?;

    db.query(
        "CREATE server_logs SET event_type_id = $event_type_id, duration_ms = $duration_ms",
    )
    .bind(("event_type_id", event_type_id))
    .bind(("duration_ms", duration_ms))
    .await?;

    Ok(())
}

pub async fn log_error(db: &DB, message: String, error_code: u32) -> Result<(), Box<dyn Error>> {
    let event_type_id = get_or_create_event_type(db, "error".to_string()).await?;

    db.query(
        "CREATE server_logs SET event_type_id = $event_type_id, message = $message, error_code = $error_code",
    )
    .bind(("event_type_id", event_type_id))
    .bind(("message", message))
    .bind(("error_code", error_code))
    .await?;

    Ok(())
}