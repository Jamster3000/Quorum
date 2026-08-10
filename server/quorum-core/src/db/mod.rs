use crate::utility::config::Config;
use std::error::Error;
use std::path::Path;
use surrealdb::Surreal;
use surrealdb::engine::local::SurrealKv;

pub mod queries;

pub type DB = Surreal<surrealdb::engine::local::Db>;

pub async fn init() -> Result<DB, Box<dyn Error>> {
    let config = Config::get();
    let path = &config.surreal_data_path;

    let parent_dir = Path::new(path).parent().unwrap_or(Path::new("."));
    let parent_exists = parent_dir.exists();
    let parent_writable = parent_exists
        && std::fs::metadata(parent_dir)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false);

    let db = Surreal::new::<SurrealKv>(path).await.map_err(|e| {
        let mut hint = String::new();
        if !parent_exists {
            hint.push_str("Parent directory does not exist. ");
        } else if !parent_writable {
            hint.push_str("Parent directory is not writable. ");
        } else {
            hint.push_str(
                "Check if the path is valid, writable, and not locked by another process. ",
            );
        }
        hint.push_str("If the file exists, it may be corrupted.");

        format!(
            "Failed to open embedded database at '{}'\nHint: {}\n\nError: {}",
            path, hint, e
        )
    })?;

    db.use_ns(&config.surreal_ns)
        .use_db(&config.surreal_db)
        .await
        .map_err(|e| format!("Failed to select namespace/database\nError: {}", e))?;

    Ok(db)
}
