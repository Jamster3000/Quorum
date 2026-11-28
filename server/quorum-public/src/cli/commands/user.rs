use colored::Colorize;
use quorum_core::cli::commands::help::RULE;
use quorum_core::cli::registry::CmdCtx;

use crate::db::queries::auth::{delete_user_by_id, list_users};

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

    match delete_user_by_id(&ctx.db, id).await {
        Ok(_) => println!("{}", format!("  User {} deleted.", id).green()),
        Err(e) => println!("{}", format!("  Failed to delete user: {}", e).red()),
    }
}

/// Lists user accounts, optionally filtered by a search term matched against
/// the user's ID, username, or email.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` contains an optional search term.
pub async fn list(ctx: CmdCtx) {
    let raw = ctx.raw.trim();
    let search = if raw.is_empty() { None } else { Some(raw) };

    match list_users(&ctx.db, search).await {
        Ok(users) if users.is_empty() => {
            println!(
                "{}",
                match search {
                    Some(term) => format!("No users found matching '{}'.", term),
                    None => "No users found.".to_string(),
                }
                .dimmed()
            );
        }
        Ok(users) => {
            println!();
            println!(
                "{}",
                format!(
                    "  Users ({}){}",
                    users.len(),
                    match search {
                        Some(term) => format!(" matching '{}'", term),
                        None => String::new(),
                    }
                )
                .cyan()
                .bold()
            );
            println!("{}", RULE.dimmed());
            println!();
            println!(
                "    {:<18} {:<22} {:<28} {:<12} {}",
                "Username".white().bold(),
                "ID".white().bold(),
                "Email".white().bold(),
                "Created".white().bold(),
                "Status".white().bold()
            );

            for user in &users {
                let email = user.email.clone().unwrap_or_else(|| "-".to_string());
                let created = user
                    .created_at
                    .as_deref()
                    .map(short_date)
                    .unwrap_or_else(|| "-".to_string());
                let status = if user.is_deleted {
                    "deleted".red().to_string()
                } else if user.is_banned {
                    "banned".red().to_string()
                } else {
                    "active".green().to_string()
                };

                println!(
                    "    {:<18} {:<22} {:<28} {:<12} {}",
                    user.username.white(),
                    user.id.dimmed(),
                    email.dimmed(),
                    created,
                    status
                );
            }

            println!();
            println!(
                "  {}",
                "Run `user:delete <id>` to delete a user account. IDs include the `users:` prefix."
                    .dimmed()
            );
            println!();
        }
        Err(e) => println!("{}", format!("  Failed to list users: {}", e).red()),
    }
}

/// Shortens a datetime string to just its date part, e.g.
/// `2025-01-02T10:20:30Z` becomes `2025-01-02`.
///
/// # Arguments
/// * `value` - The raw datetime string from the database.
fn short_date(value: &str) -> String {
    value.split('T').next().unwrap_or(value).to_string()
}
