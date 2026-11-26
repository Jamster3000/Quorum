//! This file includse query functions for getting stats and information about the database
//!
//! This file includes functions to retrieve various statistics and information about the database,
//! such as the number of records in specific tables, the size of the database, and other relevant metrics. These functions are useful for monitoring and analyzing the performance and usage of the database.
//!
//! Table sizes are *estimates*: each table's byte size is extrapolated from the serialized
//! length of a small bounded sample of rows, so the queries stay cheap (and memory-flat) no
//! matter how large the tables grow.

use crate::db::init::DB;
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

/// Number of rows sampled (at most) to estimate a table's size.
const SIZE_SAMPLE_LIMIT: usize = PAGE_SIZE;

/// Counts every record in a table without loading any of them into memory.
///
/// Uses a server-side aggregate (`GROUP ALL`) so only a single number crosses the wire,
/// regardless of how many rows the table holds.
async fn count_table(db: &DB, table: &str) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let mut response = db
        .query("SELECT count() FROM type::table($table) GROUP ALL")
        .bind(("table", table.to_string()))
        .await?;

    let counts: Vec<serde_json::Value> = response.take(0)?;
    Ok(counts
        .first()
        .and_then(|c| c["count"].as_u64())
        .map(|n| n as usize)
        .unwrap_or(0))
}

/// Estimates a table's byte size from a bounded serialized sample.
///
/// Measures the JSON length of the first `SIZE_SAMPLE_LIMIT` rows and scales that by the
/// table's total row count. This is an approximation of on-disk storage, not an exact
/// figure, but it keeps the cost of `db:stats` constant instead of scanning whole tables.
async fn estimate_table_size(
    db: &DB,
    table: &str,
    count: usize,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if count == 0 {
        return Ok(0);
    }

    let mut response = db
        .query("SELECT * FROM type::table($table) LIMIT $limit")
        .bind(("table", table.to_string()))
        .bind(("limit", SIZE_SAMPLE_LIMIT))
        .await?;

    let sample: Vec<serde_json::Value> = response.take(0)?;
    if sample.is_empty() {
        return Ok(0);
    }

    let sample_bytes = serde_json::to_string(&sample).map(|s| s.len()).unwrap_or(0);
    Ok(sample_bytes / sample.len() * count)
}

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
/// ```rust,ignore
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
        let count = count_table(db, name).await?;
        let size_bytes = estimate_table_size(db, name, count).await?;

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
/// * `Ok(TableRecords)` - A struct containing the records, total count, current page, total pages, and estimated size in bytes if the query is successful.
/// * `Err(Box<dyn Error + Send + Sync>)` - An error if the query fails or if the table does not exist.
///
/// # Examples
/// ```rust,ignore
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

    let total = count_table(db, table).await?;
    let total_pages = (total + PAGE_SIZE - 1).max(1) / PAGE_SIZE;
    let page = page.clamp(1, total_pages);
    let start = (page - 1) * PAGE_SIZE;

    let mut response = db
        .query("SELECT * FROM type::table($table) ORDER BY id LIMIT $limit START $start")
        .bind(("table", table.to_string()))
        .bind(("limit", PAGE_SIZE))
        .bind(("start", start))
        .await?;

    let records: Vec<serde_json::Value> = response.take(0)?;

    let size_bytes = if records.is_empty() {
        0
    } else {
        let page_bytes = serde_json::to_string(&records)
            .map(|s| s.len())
            .unwrap_or(0);
        page_bytes / records.len() * total
    };

    Ok(TableRecords {
        records,
        total,
        page,
        total_pages,
        size_bytes,
    })
}
