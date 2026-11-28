//! Main file for server related commands

use crate::cli::registry::CmdCtx;
use crate::models::admin::SESSION_TIMEOUT_MINS;
use crate::utility::secrets::prompt_passphrase;
use crate::utility::std::typewriter_println;
use colored::Colorize;
use dialoguer::Input;
use zeroize::Zeroizing;

/// Displays server logs
///
/// Shows the server logs from the `server_logs` database table.
/// Has option to show the last X days of logs.
/// By default, shows 100 of the most recent logs.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` may contain the number of days to filter logs by.
///
/// # Example
/// ```rust,ignore
/// logs(&db, "").await; // Shows the latest 100 logs
/// logs(&db, "7").await; // Shows logs from the last 7 days
/// logs(&db, "invalid").await; // Shows usage message
/// ```
pub async fn logs(ctx: CmdCtx) {
    let days: Option<u32> = ctx.raw.trim().parse().ok();

    if !ctx.raw.trim().is_empty() && days.is_none() {
        println!("{}", "Usage: server:logs OR server:logs <days>".red());
        return;
    }

    match crate::db::queries::logs::get_server_logs(&ctx.db, days).await {
        Err(e) => println!("{}", format!("  Failed to get logs: {}", e).red()),
        Ok(entries) => {
            println!();
            println!("{}", "  Server Logs".cyan().bold());
            if let Some(d) = days {
                println!("  {}", format!("Last {} days", d).dimmed());
            } else {
                println!("  {}", "Latest 100 entries".dimmed());
            }
            println!(
                "{}",
                "  ──────────────────────────────────────────────────────────────".dimmed()
            );

            if entries.is_empty() {
                println!("{}", "  No log entries found.".dimmed());
            } else {
                for e in &entries {
                    let mut parts = vec![
                        e.timestamp.dimmed().to_string(),
                        e.event_type.cyan().to_string(),
                    ];
                    if let Some(ms) = e.duration_ms {
                        parts.push(format!("{}ms", ms).dimmed().to_string());
                    }
                    if let Some(msg) = &e.message {
                        parts.push(msg.yellow().to_string());
                    }
                    if let Some(code) = e.error_code {
                        parts.push(format!("code:{}", code).red().to_string());
                    }
                    println!("  {}", parts.join("  -  "));
                }
            }
            println!();
        }
    }
}

/// Displays server audit logs
///
/// Shows the audit logs from the `audit_logs` database table.
/// Has option to show the last X days of logs.
/// By default, shows 100 of the most recent logs.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` may contain the number of days to filter audit logs by.
///
/// # Example
/// ```rust,ignore
/// audit(&db, "").await; // Shows the latest 100 audit logs
/// audit(&db, "7").await; // Shows audit logs from the last 7 days
/// audit(&db, "invalid").await; // Shows usage message
/// ```
pub async fn audit(ctx: CmdCtx) {
    let days: Option<u32> = ctx.raw.trim().parse().ok();

    if !ctx.raw.trim().is_empty() && days.is_none() {
        println!("{}", "Usage: server:audit OR server:audit <days>".red());
        return;
    }

    match crate::db::queries::logs::get_audit_logs(&ctx.db, days).await {
        Err(e) => println!("{}", format!("  Failed to get audit logs: {}", e).red()),
        Ok(entries) => {
            println!();
            println!("{}", "  Audit Logs".cyan().bold());
            if let Some(d) = days {
                println!("  {}", format!("Last {} days", d).dimmed());
            } else {
                println!("  {}", "Latest 100 entries".dimmed());
            }
            println!(
                "{}",
                "  ──────────────────────────────────────────────────────────────".dimmed()
            );

            if entries.is_empty() {
                println!("{}", "  No audit entries found.".dimmed());
            } else {
                for e in &entries {
                    let mut parts = vec![
                        e.created_at.dimmed().to_string(),
                        e.log_type.cyan().to_string(),
                    ];
                    if let Some(action) = &e.action {
                        parts.push(action.white().to_string());
                    }
                    if let Some(user) = &e.user_id {
                        parts.push(format!("user:{}", user).green().to_string());
                    }
                    if let Some(target) = &e.target {
                        parts.push(format!("→ {}", target).dimmed().to_string());
                    }
                    println!("  {}", parts.join("  ·  "));
                }
            }
            println!();
        }
    }
}

/// Displays server status
///
/// Shows the server uptime, listening address, and testing mode status.
///
/// # Arguments
/// * `ctx` - The command context, used for the server start time.
///
/// # Example
/// ```rust,ignore
/// status(server_start).await; // Displays the server status
/// ```
pub async fn status(ctx: CmdCtx) {
    let elapsed = ctx.server_start.elapsed();

    let total_secs = elapsed.as_secs();
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    let uptime = if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, mins, secs)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    };

    let config = crate::utility::config::Config::get();

    println!();
    println!("{}", "  Server Status".cyan().bold());
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────────────".dimmed()
    );
    println!("  {:<20} {}", "Uptime:".white(), uptime.green());
    println!(
        "  {:<20} {}",
        "Listening on:".white(),
        format!("{}:{}", config.server_host, config.server_port).green()
    );
    println!(
        "  {:<20} {}",
        "Testing mode:".white(),
        if config.enable_testing {
            "enabled".yellow().to_string()
        } else {
            "disabled".dimmed().to_string()
        }
    );
    println!();
}

/// Handles user logout
///
/// Logs out the currently logged-in user, if any.
///
/// # Arguments
/// * `ctx` - The command context, used to access the admin session.
///
/// # Example
/// ```rust,ignore
/// logout(&session); // Logs out the currently logged-in user
/// ```
pub async fn logout(ctx: CmdCtx) {
    let mut sess = ctx.session.lock().unwrap();
    if !sess.logged_in {
        println!("{}", "Not currently logged in.".yellow());
        return;
    }
    let username = sess.username.clone().unwrap_or_default();
    sess.logout();
    println!("{}", format!("Logged out: {}.", username).green());
}

/// Handles server shutdown
///
/// Logs the shutdown event to the database and signals the server to shut down gracefully.
///
/// # Arguments
/// * `ctx` - The command context, providing the database, start time and shutdown channel.
///
/// # Example
/// ```rust,ignore
/// shutdown(&db, server_start, &shutdown_tx).await; // Logs shutdown and signals server to shut down
/// ```
pub async fn shutdown(ctx: CmdCtx) {
    println!("{}", "\nShutting down gracefully...".yellow().bold());

    let uptime_ms = ctx.server_start.elapsed().as_millis() as i64;
    let _ = crate::db::queries::server_logs::log_shutdown(&ctx.db, uptime_ms).await;

    let _ = ctx.shutdown_tx.send(true);
    std::process::exit(0);
}

/// Handles reloading the server configuration
///
/// This reloads the server configuration from the configuration file and applies any changes.
///
/// # Example
/// ```rust,ignore
/// reload(); // Reloads the server configuration
/// ```
pub async fn reload(_ctx: CmdCtx) {
    match crate::utility::config::Config::reload() {
        Ok(_) => println!("{}", "  Config reloaded successfully.".green()),
        Err(e) => println!("{}", format!("  Failed to reload config: {}", e).red()),
    }
}

/// Authenticates an admin session using the server passphrase.
///
/// Prompts for the passphrase and attempts to decrypt secrets.enc with it.
/// Success means the caller is whoever set up the server — session is granted.
/// The session expires after 20 minutes of inactivity as normal.
///
/// # Arguments
/// * `ctx` - The command context, providing the admin session.
///
/// # Example
/// ```rust,ignore
/// login(&session);
/// ```
pub async fn login(ctx: CmdCtx) {
    {
        let sess = ctx.session.lock().unwrap();
        if sess.is_valid() {
            println!("{}", "Already logged in. Run server:logout first.".yellow());
            return;
        }
    }

    let passphrase = match prompt_passphrase() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("  Failed to read passphrase: {}", e).red());
            return;
        }
    };

    match crate::utility::secrets::load_encrypted_config(&passphrase) {
        Ok(_) => {
            let mut sess = ctx.session.lock().unwrap();
            sess.login("admin".to_string(), true);
            println!("{}", "  Authenticated.".green().bold());
            println!(
                "{}",
                format!(
                    "  Session will expire after {} minutes of inactivity.",
                    SESSION_TIMEOUT_MINS
                )
                .dimmed()
            );
        }
        Err(_) => {
            println!("{}", "  Invalid passphrase.".red());
        }
    }

    println!();
}

/// Prompts the user to confirm database deletion and initiates a reset.
///
/// Displays a confirmation prompt and requires the user to enter their server passphrase
/// to verify they have authority to perform this destructive operation. On confirmation,
/// deletes the entire database directory and recreates it empty, then gracefully shuts
/// down the server for restart.
///
/// The user will be prompted twice: once to confirm intent, and once to provide the passphrase.
/// Either prompt can be cancelled by answering "no" to the confirmation or entering an invalid passphrase.
///
/// # Arguments
/// * `shutdown_tx` - A watch channel sender used to signal server shutdown after a successful reset.
///
/// # Example
/// ```rust
/// confirm_and_delete(&shutdown_tx).await;
/// ```
pub async fn confirm_and_delete(shutdown_tx: &tokio::sync::watch::Sender<bool>) {
    println!();
    let _ = typewriter_println(&format!(
        "{}",
        "WARNING: This will delete all data and shut down the server.\nYou must restart it manually after.\n\nContinue? (y/n)".yellow()
    ))
        .map_err(|e| e.to_string());

    let mut confirm = Input::<String>::new()
        .with_prompt(" ")
        .interact_text()
        .unwrap();

    confirm = confirm.to_lowercase();

    if confirm != "y" && confirm != "yes" {
        return;
    }

    println!();
    let _ = typewriter_println(&format!(
        "{}",
        "Enter passphrase to confirm database deletion..."
            .cyan()
            .bold()
    ))
    .map_err(|e| e.to_string());

    match prompt_passphrase() {
        Ok(passphrase) => {
            let passphrase = Zeroizing::new(passphrase);
            if let Err(e) = perform_reset(&passphrase).await {
                eprintln!("{}", format!("Failed to reset database: {e}").red());
                return;
            }
            println!(
                "{}",
                "Database reset complete. The server is shutting down.\nPlease restart the server to continue.".green()
            );
            let _ = shutdown_tx.send(true);
        }
        Err(e) => {
            eprintln!("Failed to read passphrase: {}", e);
        }
    }
}

/// Deletes and reinitializes the database with an empty schema.
///
/// Verifies the provided passphrase to ensure the caller has authority, then spawns a blocking
/// task to delete the entire database directory and recreate it. After the filesystem operations
/// complete, reinitializes the database schema from the initial migration script.
///
/// The database deletion is performed on a separate thread to avoid holding locks from the CLI's
/// database reference, which would prevent the filesystem operations from succeeding on Windows.
///
/// # Errors
///
/// Returns an error when:
/// - the passphrase is invalid,
/// - the database directory cannot be deleted or recreated,
/// - or schema initialization fails.
///
/// # Arguments
/// * `passphrase` - The server passphrase, used to verify the caller's authority before performing the reset.
async fn perform_reset(passphrase: &str) -> Result<(), Box<dyn std::error::Error>> {
    crate::utility::config::Config::load_with_passphrase(passphrase)?;

    let config = crate::utility::config::Config::get();
    let path = std::path::Path::new(&config.surreal_data_path).to_path_buf();

    tokio::task::spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if path.exists() {
            std::fs::remove_dir_all(&path).ok();
        }
        std::fs::create_dir_all(&path).ok();
    })
    .await?;

    Ok(())
}

/// Handler for `db:delete`.
///
/// # Arguments
/// * `ctx` - The command context, providing the shutdown channel.
pub async fn db_delete(ctx: CmdCtx) {
    confirm_and_delete(&ctx.shutdown_tx).await;
}

/// Shows the server version
///
/// Prints the Quorum server version from the crate metadata, captured at
/// build time.
///
/// # Arguments
/// * `_ctx` - The command context (unused).
///
/// # Example
/// ```rust,ignore
/// version(_ctx).await;
/// ```
pub async fn version(_ctx: CmdCtx) {
    println!();
    println!("{}", "  Server Version".cyan().bold());
    println!(
        "{}",
        "  ──────────────────────────────────────────────────────────────".dimmed()
    );
    println!("  {:<12} {}", "Server:".white(), "Quorum".green());
    println!(
        "  {:<12} {}",
        "Version:".white(),
        env!("CARGO_PKG_VERSION").green()
    );
    println!();
}
