use crate::cli::AdminSession;
use crate::db::DB;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::watch;

pub async fn logs(db: &DB, raw: &str) {
    let days: Option<u32> = raw.trim().parse().ok();

    if !raw.trim().is_empty() && days.is_none() {
        println!("{}", "Usage: server:logs OR server:logs <days>".red());
        return;
    }

    match crate::db::queries::logs::get_server_logs(db, days).await {
        Err(e) => println!("{}", format!("  Failed to get logs: {}", e).red()),
        Ok(entries) => {
            println!();
            println!("{}", "  Server Logs".cyan().bold());
            if let Some(d) = days {
                println!("  {}", format!("Last {} days", d).dimmed());
            } else {
                println!("  {}", "Latest 100 entries".dimmed());
            }
            println!("{}", "  ─────────────────────────────────────────────────────".dimmed());

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
                    println!("  {}", parts.join("  ·  "));
                }
            }
            println!();
        }
    }
}

pub async fn audit(db: &DB, raw: &str) {
    let days: Option<u32> = raw.trim().parse().ok();

    if !raw.trim().is_empty() && days.is_none() {
        println!("{}", "Usage: server:audit OR server:audit <days>".red());
        return;
    }

    match crate::db::queries::logs::get_audit_logs(db, days).await {
        Err(e) => println!("{}", format!("  Failed to get audit logs: {}", e).red()),
        Ok(entries) => {
            println!();
            println!("{}", "  Audit Logs".cyan().bold());
            if let Some(d) = days {
                println!("  {}", format!("Last {} days", d).dimmed());
            } else {
                println!("  {}", "Latest 100 entries".dimmed());
            }
            println!("{}", "  ─────────────────────────────────────────────────────".dimmed());

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


pub async fn status(server_start: Instant) {
    let elapsed = server_start.elapsed();

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
    println!("{}", "  ─────────────────────────────────".dimmed());
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

pub async fn signup(db: &DB) {
    let username = prompt("  Username: ");
    let password = prompt_password("  Password: ");
    let email = prompt("  Email (optional, leave blank): ");

    let email = if email.is_empty() { None } else { Some(email) };

    match crate::db::queries::auth::signup_user(db, &username, email.as_deref(), &password).await {
        Ok(user) => println!(
            "{}",
            format!(
                "  Created user: {} ({})",
                user.username,
                format!("{:?}", user.id.key)
            )
            .green()
        ),
        Err(e) => println!("{}", format!("  Failed to create user: {}", e).red()),
    }
}

pub async fn login(db: &DB, session: &Arc<Mutex<AdminSession>>) {
    {
        let sess = session.lock().unwrap();
        if sess.is_valid() {
            println!(
                "{}",
                format!(
                    "Already logged in as {}. Run server:logout first.",
                    sess.username.as_deref().unwrap_or("unknown")
                )
                .yellow()
            );
            return;
        }
    }

    let username = prompt("  Username: ");
    let password = prompt_password("  Password: ");

    match crate::db::queries::auth::verify_user_credentials(db, &username, &password).await {
        Ok(user) => {
            let mut sess = session.lock().unwrap();
            sess.login(user.username.clone(), user.is_admin);
            println!(
                "{}",
                format!("  Logged in as {}.", user.username).green().bold()
            );
            println!(
                "{}",
                format!(
                    "  Session will expire after {} minutes of inactivity.",
                    crate::cli::SESSION_TIMEOUT_MINS
                )
                .dimmed()
            );
        }
        Err(_) => {
            println!("{}", "  Invalid username or password.".red());
        }
    }

    println!();
}

pub fn logout(session: &Arc<Mutex<AdminSession>>) {
    let mut sess = session.lock().unwrap();
    if !sess.logged_in {
        println!("{}", "Not currently logged in.".yellow());
        return;
    }
    let username = sess.username.clone().unwrap_or_default();
    sess.logout();
    println!("{}", format!("Logged out: {}.", username).green());
}

pub async fn shutdown(db: &DB, server_start: Instant, shutdown_tx: &watch::Sender<bool>) {
    println!("{}", "\nShutting down gracefully...".yellow().bold());

    let uptime_ms = server_start.elapsed().as_millis() as i64;
    let _ = crate::db::queries::server_logs::log_shutdown(db, uptime_ms).await;

    let _ = shutdown_tx.send(true);
    std::process::exit(0);
}

pub async fn make_admin(db: &DB, username: &str, session: &Arc<Mutex<AdminSession>>) {
    match crate::db::queries::auth::make_admin(db, username).await {
        Ok(_) => {
            println!("{}", format!("  {} is now an admin.", username).green());

            let mut sess = session.lock().unwrap();
            if sess.username.as_deref() == Some(username) {
                sess.update_is_admin(true);
            }
        }
        Err(e) => println!("{}", format!("  Failed: {}", e).red()),
    }
}

pub fn reload() {
    match crate::utility::config::Config::reload() {
        Ok(_) => println!("{}", "  Config reloaded successfully.".green()),
        Err(e) => println!("{}", format!("  Failed to reload config: {}", e).red()),
    }
}

fn prompt(label: &str) -> String {
    print!("{}", label.white());
    std::io::Write::flush(&mut std::io::stdout()).ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

fn prompt_password(label: &str) -> String {
    print!("{}", label.white());
    std::io::Write::flush(&mut std::io::stdout()).ok();

    rpassword::read_password().unwrap_or_default()
}