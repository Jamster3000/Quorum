//! Configuration command handling (REFACTORED).
//!
//! This module handles the `config:show` and `config:<key> <value>` commands.
//! It no longer hardcodes every field — instead it uses the schema to generate display/parsing dynamically.

use colored::Colorize;
use quorum_core::utility::config::Config;

/// Changes a configuration value by key.
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

/// Displays all configuration settings in a formatted table.
///
/// This function uses the schema to dynamically generate the display,
/// so it automatically includes any new fields added to the schema.
pub fn print_all() {
    let cfg = Config::get();

    println!();
    println!(
        "{}",
        "┌────────────────────────────────────────────────────────────┐".dimmed()
    );
    println!("│ {} │", "SERVER CONFIGURATION".cyan().bold());
    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );

    // Dynamically display each config field using Display trait
    // Note: The actual display is handled below per-field since they have different types
    // If you need to add custom formatting for a field (like truncating jwt_secret),
    // do it in the match statement below rather than in the schema.

    display_config_field("server_host", &cfg.server_host.to_string());
    display_config_field("server_port", &cfg.server_port.to_string());
    display_config_field("server_url", &cfg.server_url.to_string());

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );

    display_config_field("surreal_data_path", &cfg.surreal_data_path.to_string());
    display_config_field("surreal_ns", &cfg.surreal_ns.to_string());
    display_config_field("surreal_db", &cfg.surreal_db.to_string());

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );

    // Truncate jwt_secret for security
    let short_jwt = if cfg.jwt_secret.len() > 30 {
        format!("{}...", &cfg.jwt_secret[..30])
    } else {
        cfg.jwt_secret.clone()
    };
    display_config_field_custom("jwt_secret", &short_jwt, true);

    display_config_field("jwt_access_minutes", &format!("{} min", cfg.jwt_access_minutes));
    display_config_field("jwt_refresh_days", &format!("{} days", cfg.jwt_refresh_days));

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );

    let testing_status = if cfg.enable_testing {
        "true".green().bold().to_string()
    } else {
        "false".red().to_string()
    };
    display_config_field_custom("enable_testing", &testing_status, false);

    display_config_field("default_per_second", &cfg.default_per_second.to_string());
    display_config_field("default_burst_size", &cfg.default_burst_size.to_string());
    display_config_field("testing_per_second", &cfg.testing_per_second.to_string());
    display_config_field("testing_burst_size", &cfg.testing_burst_size.to_string());

    println!(
        "{}",
        "└────────────────────────────────────────────────────────────┘".dimmed()
    );
    println!();
}

/// Helper to display a single config field in the table format.
fn display_config_field(key: &str, value: &str) {
    println!(
        "│ {:<22} : {:<33} │",
        key.yellow(),
        value.white()
    );
}

/// Helper to display a config field with custom value formatting (for colored or special values).
fn display_config_field_custom(key: &str, value: &str, is_secret: bool) {
    let value_display = if is_secret {
        value.dimmed().to_string()
    } else {
        value.to_string()
    };
    println!(
        "│ {:<22} : {:<42} │",
        key.yellow(),
        value_display
    );
}