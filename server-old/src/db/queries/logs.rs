use crate::db::DB;
use std::error::Error;

pub struct ServerLogEntry {
    pub timestamp: String,
    pub event_type: String,
    pub duration_ms: Option<f64>,
    pub message: Option<String>,
    pub error_code: Option<f64>,
}

pub struct AuditLogEntry {
    pub created_at: String,
    pub log_type: String,
    pub action: Option<String>,
    pub user_id: Option<String>,
    pub target: Option<String>,
}

/// Retrieves server logs from the database.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `days` - An optional number of days to filter logs. If `None`, retrieves the last 100 logs.
///
/// # Returns
/// * `Ok(Vec<ServerLogEntry>)` - A vector of server log entries if the query is successful.
/// * `Err(Box<dyn Error + Send + Sync>)` - An error if the query fails.
///
/// # Examples
/// ```rust
/// let logs = get_server_logs(&db, Some(7)).await.unwrap(); // Get logs from the last 7 days
/// let recent_logs = get_server_logs(&db, None).await.unwrap(); // Get the last 100 logs
/// ```
pub async fn get_server_logs(
    db: &DB,
    days: Option<u32>,
) -> Result<Vec<ServerLogEntry>, Box<dyn Error + Send + Sync>> {
    let query = match days {
        Some(d) => format!(
            "SELECT timestamp, event_type_id.name AS event_type, duration_ms, message, error_code
             FROM server_logs
             WHERE timestamp >= time::now() - {}d
             ORDER BY timestamp DESC",
            d
        ),
        None => {
            "SELECT timestamp, event_type_id.name AS event_type, duration_ms, message, error_code
                 FROM server_logs
                 ORDER BY timestamp DESC
                 LIMIT 100"
                .to_string()
        }
    };

    let mut response = db.query(&query).await?;
    let records: Vec<serde_json::Value> = response.take(0)?;

    let entries = records
        .into_iter()
        .map(|r| ServerLogEntry {
            timestamp: r["timestamp"].as_str().unwrap_or("unknown").to_string(),
            event_type: r["event_type"].as_str().unwrap_or("unknown").to_string(),
            duration_ms: r["duration_ms"].as_f64(),
            message: r["message"].as_str().map(|s| s.to_string()),
            error_code: r["error_code"].as_f64(),
        })
        .collect();

    Ok(entries)
}

/// Retrieves audit logs from the database.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `days` - An optional number of days to filter logs. If `None`, retrieves the last 100 logs.
///
/// # Returns
/// * `Ok(Vec<AuditLogEntry>)` - A vector of audit log entries if the query is successful.
/// * `Err(Box<dyn Error + Send + Sync>)` - An error if the query fails.
///
/// # Examples
/// ```rust
/// let logs = get_audit_logs(&db, Some(7)).await.unwrap(); // Get logs from the last 7 days
/// let recent_logs = get_audit_logs(&db, None).await.unwrap(); // Get the last 100 logs
/// ```
pub async fn get_audit_logs(
    db: &DB,
    days: Option<u32>,
) -> Result<Vec<AuditLogEntry>, Box<dyn Error + Send + Sync>> {
    let query = match days {
        Some(d) => format!(
            "SELECT created_at, log_type_id.name AS log_type, action_type_id.name AS action, user_id, target_type_table_id
             FROM audit_logs
             WHERE created_at >= time::now() - {}d
             ORDER BY created_at DESC",
            d
        ),
        None => "SELECT created_at, log_type_id.name AS log_type, action_type_id.name AS action, user_id, target_type_table_id
                 FROM audit_logs
                 ORDER BY created_at DESC
                 LIMIT 100".to_string(),
    };

    let mut response = db.query(&query).await?;
    let records: Vec<serde_json::Value> = response.take(0)?;

    let entries = records
        .into_iter()
        .map(|r| AuditLogEntry {
            created_at: r["created_at"].as_str().unwrap_or("unknown").to_string(),
            log_type: r["log_type"].as_str().unwrap_or("unknown").to_string(),
            action: r["action"].as_str().map(|s| s.to_string()),
            user_id: r["user_id"].as_str().map(|s| s.to_string()),
            target: r["target_type_table_id"].as_str().map(|s| s.to_string()),
        })
        .collect();

    Ok(entries)
}
