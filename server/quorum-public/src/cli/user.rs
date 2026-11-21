use super::registry::CmdCtx;
use colored::Colorize;

/// Deletes a user by ID from the database.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` contains the user ID to delete.
///
/// # Example
/// ```rust,ignore
/// delete(&db, "user_id").await;
/// ```
pub async fn delete(ctx: CmdCtx) {
    let id = ctx.raw.trim();

    if id.is_empty() {
        println!("{}", "Usage: user:delete <id>".red());
        return;
    }

    match crate::db::queries::auth::delete_user_by_id(&ctx.db, id).await {
        Ok(_) => println!("{}", format!("  User {} deleted.", id).green()),
        Err(e) => println!("{}", format!("  Failed to delete user: {}", e).red()),
    }
}
