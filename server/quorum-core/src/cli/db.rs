//! This file includes direct server commands that interact or are about the surreal database/

use crate::db::DB;
use colored::Colorize;

/// Format bytes into a human-readable string
///
/// # Arguments
/// * `bytes` - The number of bytes to format.
///
/// # Returns
/// * `String` - A human-readable string representing the size.
fn format_size(bytes: usize) -> String {
    if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.2} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}

/// Fetch and display database statistics
///
/// Fetches statistics about the database, including the number of rows and estimated size for each table, and prints them to the console.
///
/// # Arguments
/// * `db` - A reference to the database connection.
///
/// # Example
/// ```
/// let db = DB::connect("database_url").await?;
/// db::stats(&db).await;
/// ```
pub async fn stats(db: &DB) {
    println!("{}", "  Fetching database stats...".dimmed());

    match crate::db::queries::stats::get_stats(db).await {
        Err(e) => println!("{}", format!("  Failed to get stats: {}", e).red()),
        Ok(stats) => {
            println!();
            println!("{}", "  Database Stats".cyan().bold());
            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );
            println!(
                "  {:<38} {:>8}  {:>12}",
                "Table".white().bold(),
                "Rows".white().bold(),
                "Est. Size".white().bold()
            );
            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );

            for t in &stats.tables {
                let count_str = if t.count > 0 {
                    t.count.to_string().green().to_string()
                } else {
                    t.count.to_string().dimmed().to_string()
                };

                println!(
                    "  {:<38} {:>8}  {:>12}",
                    t.name.white(),
                    count_str,
                    format_size(t.size_bytes).dimmed()
                );
            }

            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );
            println!(
                "  {:<38} {:>8}  {:>12}",
                "Total".white().bold(),
                stats.total_rows.to_string().cyan(),
                format_size(stats.total_size_bytes).cyan()
            );
            println!();
        }
    }
}

/// Fetch and display table data
///
/// Fetches data from a specific table in the database and prints it to the console.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `raw` - The raw command input string.
///
/// # Example
/// ```
/// let db = DB::connect("database_url").await?;
/// db::table(&db, "users").await;
/// ```
pub async fn table(db: &DB, raw: &str) {
    let parts: Vec<&str> = raw.splitn(2, ", ").collect();
    let table_name = parts[0].trim();
    let page: usize = parts
        .get(1)
        .and_then(|p| p.trim().parse().ok())
        .unwrap_or(1);

    if table_name.is_empty() {
        println!("{}", "Usage: db:table <name>".red());
        println!("{}", "       db:table <name>, <page>".red());
        return;
    }

    match crate::db::queries::stats::get_table(db, table_name, page).await {
        Err(e) => println!("{}", format!("  {}", e).red()),
        Ok(result) => {
            println!();
            println!(
                "  {} {}  {}",
                table_name.cyan().bold(),
                format!("— {} records", result.total).dimmed(),
                format!("(est. {})", format_size(result.size_bytes)).dimmed()
            );
            println!(
                "  {}",
                format!("Page {} of {}", result.page, result.total_pages).dimmed()
            );
            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );

            if result.records.is_empty() {
                println!("{}", "  No records found.".dimmed());
            } else {
                for record in &result.records {
                    let pretty =
                        serde_json::to_string_pretty(record).unwrap_or_else(|_| record.to_string());
                    for line in pretty.lines() {
                        println!("  {}", line.white());
                    }
                    println!("{}", "  ·".dimmed());
                }
            }

            println!(
                "{}",
                "  ─────────────────────────────────────────────────────".dimmed()
            );
            if result.total_pages > 1 {
                println!(
                    "  {}",
                    format!("Run `db:table {}, <page>` to navigate.", table_name).dimmed()
                );
            }
            println!();
        }
    }
}
