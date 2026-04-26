//! Database schema management.
//! This module provides functions to initialize and manage the database schema.

use std::error::Error;

/// Initializes the database schema by executing the initial schema script.
/// This function reads the schema from a file and executes it against the provided database connection.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// 
/// # Returns
/// * `Ok(())` if the schema was initialized successfully.
/// * `Err(Box<dyn Error>)` if there was an error during initialization.
/// 
/// # Example
/// ```
/// use crate::db::schema;
/// async fn setup_database(db: &crate::db::DB) -> Result<(), Box<dyn Error>> {
///     schema::init(db).await?;
///     Ok(())
/// }
///```
pub async fn init(db: &crate::db::DB) -> Result<(), Box<dyn Error>> {
    let schema = include_str!("../../../schema/initial.surql");

    db.query(schema).await?.check()?;

    Ok(())
}
