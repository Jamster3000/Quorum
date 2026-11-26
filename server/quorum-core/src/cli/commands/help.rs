//! Main cli file used for getting help with server commands

use crate::cli::registry::{CmdCtx, CommandDef, find};
use colored::Colorize;

pub const RULE: &str = "  ──────────────────────────────────────────────────────────────";

/// Handler for the `help` command.
///
/// With no arguments prints a summary of every command, grouped by namespace.
/// With a command name prints the detailed help entry for that command.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` holds an optional command name.
pub async fn help(ctx: CmdCtx) {
    match ctx.raw.trim().is_empty() {
        true => print_all(ctx.commands),
        false => print_command(ctx.commands, ctx.raw.trim()),
    }
}

/// Prints a summary of all available commands, grouped by namespace.
///
/// Namespaces are derived from the command names in the table (everything
/// before the first `:`), in first-appearance order.
///
/// # Arguments
/// * `commands` - The active command table.
pub fn print_all(commands: &[&CommandDef]) {
    println!();
    println!("{}", "  Available Commands".cyan().bold());
    println!("{}", RULE.dimmed());

    let mut namespaces: Vec<&str> = Vec::new();

    for entry in commands {
        let ns = entry.name.split(':').next().unwrap_or(entry.name);
        if !namespaces.contains(&ns) {
            namespaces.push(ns);
        }
    }

    for ns in namespaces {
        println!();
        println!("  {}", ns.white().bold());

        for entry in commands
            .iter()
            .filter(|e| e.name == ns || e.name.starts_with(&format!("{}:", ns)))
        {
            let auth_marker = if entry.requires_auth {
                " *".yellow().to_string()
            } else {
                String::new()
            };
            println!(
                "    {:<30} {}{}",
                entry.name.green(),
                entry.summary.dimmed(),
                auth_marker
            );
        }
    }

    println!();
    println!(
        "  {} {}",
        "*".yellow(),
        "marked commands require server:login before use.".dimmed()
    );
    println!(
        "  {}",
        "Run `help <command>` for detailed usage on any command.".dimmed()
    );
    println!();
}

/// Prints detailed help for a specific command, including its description,
/// usage, parameters, and auth requirements.
///
/// # Arguments
/// * `commands` - The active command table.
/// * `command` - The command name to look up, e.g. `server:status`.
pub fn print_command(commands: &[&CommandDef], command: &str) {
    match find(commands, command) {
        None => {
            println!("{} {}", "No help entry found for:".red(), command.white());
            println!("{}", "Run `help` to see all available commands.".dimmed());
        }
        Some(e) => {
            println!();
            println!("  {}", e.name.cyan().bold());
            println!(
                "{}",
                "  ──────────────────────────────────────────────────────────────".dimmed()
            );
            println!(
                "  {:<16} {}",
                "Description:".white(),
                e.description.dimmed()
            );
            println!("  {:<16} {}", "Usage:".white(), e.usage.green());
            println!(
                "  {:<16} {}",
                "Auth required:".white(),
                if e.requires_auth {
                    "yes".yellow().to_string()
                } else {
                    "no".dimmed().to_string()
                }
            );
            println!(
                "  {:<16} {}",
                "Admin permissions required:".white(),
                if e.requires_admin {
                    "yes".yellow().to_string()
                } else {
                    "no".dimmed().to_string()
                }
            );

            if !e.params.is_empty() {
                println!("  {}", "Parameters:".white());
                for (name, desc) in e.params {
                    println!("    {:<16} {}", name.green(), desc.dimmed());
                }
            }

            println!();
        }
    }
}
