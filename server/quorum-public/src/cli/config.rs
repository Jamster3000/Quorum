//! Handlers for the `config` command.
//!
//! `config show` prints every configuration field, grouped by section, and
//! `config <key> <value>` updates a single setting through the config schema.

use colored::Colorize;
use quorum_core::cli::help::RULE;
use quorum_core::cli::registry::CmdCtx;
use quorum_core::utility::config::Config;

/// Changes the value of a configuration setting in the config file.
///
/// # Arguments
/// * `config_name` - The configuration key name (e.g., "server_host", "jwt_access_minutes")
/// * `config_value` - The new value as a string (parsed to the field's type)
pub fn change_config_value(config_name: &str, config_value: &str) {
    match Config::update(config_name, config_value) {
        Ok(_) => println!("{}", "Config value updated successfully.".green()),
        Err(e) => println!("{}", format!("Failed to update config value: {}", e).red()),
    }
}

/// Prints the full configuration, grouped into sections.
///
/// The grouping follows the same sections the old box table used: server,
/// database, auth, and testing/rate limits. Rendering matches the layout used
/// by the other commands (title, rule, then label/value rows).
pub fn print_all() {
    let cfg = Config::get();

    println!();
    println!("{}", "  Server Configuration".cyan().bold());
    println!("{}", RULE.dimmed());

    println!();
    println!("  {}", "Server".white().bold());
    println!(
        "    {:<22} {}",
        "server_host:".white(),
        cfg.server_host.green()
    );
    println!(
        "    {:<22} {}",
        "server_port:".white(),
        cfg.server_port.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "server_url:".white(),
        cfg.server_url.green()
    );

    println!();
    println!("  {}", "Database".white().bold());
    println!(
        "    {:<22} {}",
        "surreal_data_path:".white(),
        cfg.surreal_data_path.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "surreal_ns:".white(),
        cfg.surreal_ns.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "surreal_db:".white(),
        cfg.surreal_db.to_string().green()
    );

    println!();
    println!("  {}", "Authentication".white().bold());
    let short_jwt = if cfg.jwt_secret.len() > 30 {
        format!("{}...", &cfg.jwt_secret[..30])
    } else {
        cfg.jwt_secret.clone()
    };
    println!("    {:<22} {}", "jwt_secret:".white(), short_jwt.dimmed());
    println!(
        "    {:<22} {}",
        "jwt_access_minutes:".white(),
        format!("{} min", cfg.jwt_access_minutes).green()
    );
    println!(
        "    {:<22} {}",
        "jwt_refresh_days:".white(),
        format!("{} days", cfg.jwt_refresh_days).green()
    );

    println!();
    println!("  {}", "Testing & Rate Limits".white().bold());
    println!(
        "    {:<22} {}",
        "enable_testing:".white(),
        if cfg.enable_testing {
            "true".green().bold()
        } else {
            "false".red()
        }
    );
    println!(
        "    {:<22} {}",
        "default_per_second:".white(),
        cfg.default_per_second.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "default_burst_size:".white(),
        cfg.default_burst_size.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "testing_per_second:".white(),
        cfg.testing_per_second.to_string().green()
    );
    println!(
        "    {:<22} {}",
        "testing_burst_size:".white(),
        cfg.testing_burst_size.to_string().green()
    );

    println!();
}

/// Handler for the `config` command.
///
/// With `show`, prints the full configuration table. With a key and a value,
/// updates that configuration setting.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` contains the subcommand or key and value.
///
/// # Example
/// ```rust,ignore
/// config("show").await; // prints the configuration
/// ```
pub async fn config(ctx: CmdCtx) {
    let mut parts = ctx.raw.split_whitespace();
    match parts.next() {
        Some("show") => print_all(),
        Some(key) => {
            let value = parts.collect::<Vec<_>>().join(" ");
            change_config_value(key, &value);
        }
        None => println!("{}", "Usage: config OR config <key> <value>".red()),
    }
}
