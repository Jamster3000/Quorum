use crate::db::DB;
use std::error::Error;

pub struct TableStat {
    pub name: String,
    pub count: usize,
    pub size_bytes: usize,
}

pub struct DbStats {
    pub tables: Vec<TableStat>,
    pub total_size_bytes: usize,
    pub total_rows: usize,
}

pub struct TableRecords {
    pub records: Vec<serde_json::Value>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
    pub size_bytes: usize,
}

const PAGE_SIZE: usize = 20;

/// Retrieves statistics about the database, including table names, record counts, and sizes.
///
/// # Arguments
/// * `db` - A reference to the database connection.
///
/// # Returns
/// * `Ok(DbStats)` - A struct containing statistics about the database if the query is successful.
/// * `Err(Box<dyn Error + Send + Sync>)` - An error if the query fails.
///
/// # Examples
/// ```rust
/// let stats = get_stats(&db).await.unwrap();
/// println!("Total tables: {}", stats.tables.len());
/// for table in stats.tables {
///     println!("Table: {}, Count: {}, Size: {} bytes", table.name, table.count, table.size_bytes);
/// }
/// ```
pub async fn get_stats(db: &DB) -> Result<DbStats, Box<dyn Error + Send + Sync>> {
    let mut info_response = db.query("INFO FOR DB").await?;
    let info: Option<serde_json::Value> = info_response.take(0)?;

    let table_names: Vec<String> = info
        .as_ref()
        .and_then(|v| v["tables"].as_object())
        .map(|t| t.keys().cloned().collect())
        .unwrap_or_default();

    let mut tables = Vec::new();

    for name in &table_names {
        let mut records_response = db.query(format!("SELECT * FROM {}", name)).await?;

        let records: Vec<serde_json::Value> = records_response.take(0)?;
        let count = records.len();
        let size_bytes = serde_json::to_string(&records)
            .map(|s| s.len())
            .unwrap_or(0);

        tables.push(TableStat {
            name: name.clone(),
            count,
            size_bytes,
        });
    }

    tables.sort_by(|a, b| a.name.cmp(&b.name));

    let total_size_bytes = tables.iter().map(|t| t.size_bytes).sum();
    let total_rows = tables.iter().map(|t| t.count).sum();

    Ok(DbStats {
        tables,
        total_size_bytes,
        total_rows,
    })
}

/// Retrieves records from a specific table in the database, with pagination support.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `table` - The name of the table to retrieve records from.
/// * `page` - The page number to retrieve (1-based index).
///
/// # Returns
/// * `Ok(TableRecords)` - A struct containing the records, total count, current page, total pages, and size in bytes if the query is successful.
/// * `Err(Box<dyn Error + Send + Sync>)` - An error if the query fails or if the table does not exist.
///
/// # Examples
/// ```rust
/// let table_records = get_table(&db, "users", 1).await.unwrap();
/// println!("Total records: {}", table_records.total);
/// for record in table_records.records {
///     println!("{:?}", record);
/// }
/// ```
pub async fn get_table(
    db: &DB,
    table: &str,
    page: usize,
) -> Result<TableRecords, Box<dyn Error + Send + Sync>> {
    let mut info_response = db.query("INFO FOR DB").await?;
    let info: Option<serde_json::Value> = info_response.take(0)?;

    let exists = info
        .as_ref()
        .and_then(|v| v["tables"].as_object())
        .map(|t| t.contains_key(table))
        .unwrap_or(false);

    if !exists {
        return Err(format!("Table '{}' does not exist.", table).into());
    }

    let mut all_response = db.query(format!("SELECT * FROM {}", table)).await?;

    let all_records: Vec<serde_json::Value> = all_response.take(0)?;
    let total = all_records.len();
    let total_pages = (total + PAGE_SIZE - 1).max(1) / PAGE_SIZE;
    let page = page.clamp(1, total_pages);

    let start = (page - 1) * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(total);
    let records = all_records[start..end].to_vec();

    let size_bytes = serde_json::to_string(&all_records)
        .map(|s| s.len())
        .unwrap_or(0);

    Ok(TableRecords {
        records,
        total,
        page,
        total_pages,
        size_bytes,
    })
}
