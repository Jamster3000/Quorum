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
        let mut records_response = db
            .query(format!("SELECT * FROM {}", name))
            .await?;

        let records: Vec<serde_json::Value> = records_response.take(0)?;
        let count = records.len();
        let size_bytes = serde_json::to_string(&records)
            .map(|s| s.len())
            .unwrap_or(0);

        tables.push(TableStat { name: name.clone(), count, size_bytes });
    }

    tables.sort_by(|a, b| a.name.cmp(&b.name));

    let total_size_bytes = tables.iter().map(|t| t.size_bytes).sum();
    let total_rows = tables.iter().map(|t| t.count).sum();

    Ok(DbStats { tables, total_size_bytes, total_rows })
}

pub struct TableRecords {
    pub records: Vec<serde_json::Value>,
    pub total: usize,
    pub page: usize,
    pub total_pages: usize,
    pub size_bytes: usize,
}

const PAGE_SIZE: usize = 20;

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

    let mut all_response = db
        .query(format!("SELECT * FROM {}", table))
        .await?;

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

    Ok(TableRecords { records, total, page, total_pages, size_bytes })
}