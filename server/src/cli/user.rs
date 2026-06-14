use crate::db::DB;
use colored::Colorize;

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