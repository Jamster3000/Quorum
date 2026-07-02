use colored::Colorize;
use quorum_core::db::DB;

/// Deletes a user by ID from the database.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `raw` - A string slice containing the user ID to delete.
///
/// # Example
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     let db = DB::new("sqlite:memory:").await.unwrap();
///     delete(&db, "user_id").await;
/// }
/// ```
pub async fn delete(db: &DB, raw: &str) {
    let id = raw.trim();

    if id.is_empty() {
        println!("{}", "Usage: user:delete <id>".red());
        return;
    }

    match crate::db::queries::auth::delete_user_by_id(db, id).await {
        Ok(_) => println!("{}", format!("  User {} deleted.", id).green()),
        Err(e) => println!("{}", format!("  Failed to delete user: {}", e).red()),
    }
}
