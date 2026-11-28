use colored::Colorize;
use quorum_core::cli::commands::help::RULE;
use quorum_core::cli::registry::CmdCtx;

use crate::db::queries::auth::{delete_user_by_id, get_user_full, list_users};

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
            print_user_table(
                &users,
                match search {
                    Some(term) => format!(" matching '{}'", term),
                    None => String::new(),
                },
            );
            println!(
                "  {}",
                "Run `user:show <search>` for a single user's full details, or `user:delete <id>` to delete a user account."
                    .dimmed()
            );
            println!();
        }
        Err(e) => println!("{}", format!("  Failed to list users: {}", e).red()),
    }
}

/// Shows the full details of a single user account.
///
/// Resolves the search term to a user via `user:list` matching (ID,
/// username, or email); if multiple users match, prints the matches and
/// asks for a more specific term. Sensitive fields (password hash, 2FA
/// secret, backup codes) are never displayed.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` contains the search term.
pub async fn show(ctx: CmdCtx) {
    let search = ctx.raw.trim();

    if search.is_empty() {
        println!("{}", "Usage: user:show <id | username | email>".red());
        return;
    }

    let candidates = match list_users(&ctx.db, Some(search)).await {
        Ok(c) => c,
        Err(e) => {
            println!("{}", format!("  Failed to look up user: {}", e).red());
            return;
        }
    };

    match candidates.len() {
        0 => println!("{}", format!("No users found matching '{}'.", search).red()),
        1 => match get_user_full(&ctx.db, &candidates[0].id).await {
            Ok(Some(record)) => print_user_details(&record),
            Ok(None) => println!("{}", "User record not found.".red()),
            Err(e) => println!("{}", format!("  Failed to get user: {}", e).red()),
        },
        _ => {
            println!(
                "{}",
                format!(
                    "{} users match '{}'. Be more specific, or use `user:list {}`:",
                    candidates.len(),
                    search,
                    search
                )
                .yellow()
            );
            print_user_table(&candidates, format!(" matching '{}'", search));
        }
    }
}

/// Prints the user summary table with columns sized to the data, so long
/// IDs or usernames can never push the rows out of alignment.
///
/// # Arguments
/// * `users` - The users to display.
/// * `suffix` - Text appended to the heading, e.g. " matching 'alice'".
fn print_user_table(users: &[crate::db::queries::auth::UserSummary], suffix: String) {
    let width_username = users
        .iter()
        .map(|u| u.username.len())
        .max()
        .unwrap_or(0)
        .max("Username".len());
    let width_id = users
        .iter()
        .map(|u| u.id.len())
        .max()
        .unwrap_or(0)
        .max("ID".len());
    let width_email = users
        .iter()
        .map(|u| u.email.clone().unwrap_or_else(|| "-".to_string()).len())
        .max()
        .unwrap_or(0)
        .max("Email".len());

    println!();
    println!(
        "{}",
        format!("  Users ({}){}", users.len(), suffix).cyan().bold()
    );
    println!("{}", RULE.dimmed());
    println!();
    println!(
        "    {:<width_u$} {:<width_i$} {:<width_e$} {:<12} {}",
        "Username".white().bold(),
        "ID".white().bold(),
        "Email".white().bold(),
        "Created".white().bold(),
        "Status".white().bold(),
        width_u = width_username,
        width_i = width_id,
        width_e = width_email
    );

    for user in users {
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
            "    {:<width_u$} {:<width_i$} {:<width_e$} {:<12} {}",
            user.username.white(),
            user.id.dimmed(),
            email.dimmed(),
            created,
            status,
            width_u = width_username,
            width_i = width_id,
            width_e = width_email
        );
    }
}

/// Prints every (non-sensitive) field of a full user record.
///
/// # Arguments
/// * `record` - The raw user record from the database.
fn print_user_details(record: &serde_json::Value) {
    let id = record["id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_default();

    println!();
    println!("{}", format!("  {}", id).cyan().bold());
    println!("{}", RULE.dimmed());

    let status = if record["is_deleted"].as_bool().unwrap_or(false) {
        "deleted".red().to_string()
    } else if record["is_banned"].as_bool().unwrap_or(false) {
        "banned".red().to_string()
    } else {
        "active".green().to_string()
    };

    let rows: Vec<(&str, String)> = vec![
        ("Username", str_field(record, "username")),
        ("Email", record["email"].as_str().unwrap_or("-").to_string()),
        ("Admin", bool_field(record, "is_admin")),
        ("Status", status),
        ("Created", date_field(record, "created_at")),
        ("Last login", date_field(record, "last_login")),
        (
            "Ban expiry",
            record["ban_expiry"].as_str().unwrap_or("-").to_string(),
        ),
        (
            "Locked until",
            record["account_locked_until"]
                .as_str()
                .unwrap_or("-")
                .to_string(),
        ),
        (
            "Scheduled deletion",
            record["scheduled_deletion_at"]
                .as_str()
                .unwrap_or("-")
                .to_string(),
        ),
        (
            "Avatar",
            record["avatar_url"].as_str().unwrap_or("-").to_string(),
        ),
        (
            "User status",
            record["user_status_id"].as_str().unwrap_or("-").to_string(),
        ),
        (
            "Custom status",
            record["custom_status_id"]
                .as_str()
                .unwrap_or("-")
                .to_string(),
        ),
    ];

    for (label, value) in rows {
        println!("  {:<20} {}", format!("{}:", label).white(), value.dimmed());
    }

    println!();
    println!(
        "  {}",
        format!("Delete this user with `user:delete {}`.", id).dimmed()
    );
    println!();
}

/// Reads a string field from a record, falling back to "-".
///
/// # Arguments
/// * `record` - The raw user record.
/// * `key` - The field to read.
fn str_field(record: &serde_json::Value, key: &str) -> String {
    record[key].as_str().unwrap_or("-").to_string()
}

/// Reads an optional bool field from a record.
///
/// # Arguments
/// * `record` - The raw user record.
/// * `key` - The field to read.
fn bool_field(record: &serde_json::Value, key: &str) -> String {
    match record[key].as_bool() {
        Some(true) => "yes".to_string(),
        Some(false) => "no".to_string(),
        None => "-".to_string(),
    }
}

/// Reads a datetime field from a record, shortened to its date part.
///
/// # Arguments
/// * `record` - The raw user record.
/// * `key` - The field to read.
fn date_field(record: &serde_json::Value, key: &str) -> String {
    match record[key].as_str() {
        Some(v) => short_date(v),
        None => "-".to_string(),
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
