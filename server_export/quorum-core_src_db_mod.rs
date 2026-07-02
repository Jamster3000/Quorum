use crate::utility::config::Config;
use std::error::Error;
use surrealdb::Surreal;
use surrealdb::engine::local::RocksDb;

pub mod queries;

pub type DB = Surreal<surrealdb::engine::local::Db>;

pub async fn init() -> Result<DB, Box<dyn Error>> {
    let config = Config::get();

    let db = Surreal::new::<RocksDb>(config.surreal_data_path.as_str())
        .await
        .map_err(|e| -> Box<dyn Error> {
            format!(
                "Failed to open embedded database at '{}'\nHint: Check that this path is writable.\n\nError: {}",
                config.surreal_data_path, e
            )
            .into()
        })?;

    db.use_ns(&config.surreal_ns)
        .use_db(&config.surreal_db)
        .await
        .map_err(|e| -> Box<dyn Error> {
            format!(
                "Failed to select namespace/database\nError: {}",
                e
            )
            .into()
        })?;

    Ok(db)
}