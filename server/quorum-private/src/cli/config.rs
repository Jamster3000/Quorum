//! This module contains the functions for the `config` command in the CLI.

use colored::Colorize;
use quorum_core::utility::config::Config;

/// Changes the value of a configuration setting in the config file.
///
/// # Arguments
/// * `config_name` - The name of the configuration setting to change.
/// * `config_value` - The new value for the configuration setting.
///
/// # Example
/// ```rust
/// change_config_value("server_host", "0.0.0.0");
/// change_config_value("server_port", "8080");
///```
pub fn change_config_value(config_name: &str, config_value: &str) {
    let result = Config::update(config_name, config_value);

    if let Err(e) = result {
        println!("Failed to update config value: {}", e);
    } else {
        println!("Config value updated successfully.");
    }
}

/// Prints all the configuration settings in a formatted table.
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

    println!(
        "│ {:<22} : {:<33} │",
        "server_host".yellow(),
        cfg.server_host.white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "server_port".yellow(),
        cfg.server_port.to_string().white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "server_url".yellow(),
        cfg.server_url.white()
    );

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "surreal_data_path".yellow(),
        cfg.surreal_data_path.white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "surreal_ns".yellow(),
        cfg.surreal_ns.white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "surreal_db".yellow(),
        cfg.surreal_db.white()
    );

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );
    let short_jwt = if cfg.jwt_secret.len() > 30 {
        format!("{}...", &cfg.jwt_secret[..30])
    } else {
        cfg.jwt_secret.clone()
    };
    println!(
        "│ {:<22} : {:<33} │",
        "jwt_secret".yellow(),
        short_jwt.dimmed()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "jwt_access_minutes".yellow(),
        format!("{} min", cfg.jwt_access_minutes).white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "jwt_refresh_days".yellow(),
        format!("{} days", cfg.jwt_refresh_days).white()
    );

    println!(
        "{}",
        "├────────────────────────────────────────────────────────────┤".dimmed()
    );
    let testing_status = if cfg.enable_testing {
        "true".green().bold()
    } else {
        "false".red()
    };
    println!(
        "│ {:<22} : {:<42} │",
        "enable_testing".yellow(),
        testing_status
    );
    println!(
        "│ {:<22} : {:<33} │",
        "default_per_second".yellow(),
        cfg.default_per_second.to_string().white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "default_burst_size".yellow(),
        cfg.default_burst_size.to_string().white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "testing_per_second".yellow(),
        cfg.testing_per_second.to_string().white()
    );
    println!(
        "│ {:<22} : {:<33} │",
        "testing_burst_size".yellow(),
        cfg.testing_burst_size.to_string().white()
    );

    println!(
        "{}",
        "└────────────────────────────────────────────────────────────┘".dimmed()
    );
    println!();
}
