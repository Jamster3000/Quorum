//! This module contains the functions for the `config` command in the CLI.

use quorum_core::utility::config::Config;
use colored::Colorize;

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
    println!("{}", "┌────────────────────────────────────────────────────────────┐".dimmed());
    println!("│ {} │", "SERVER CONFIGURATION".cyan().bold());
    println!("{}", "├────────────────────────────────────────────────────────────┤".dimmed());

    println!("│ {:<22} : {} │", "server_host".yellow(), format!("{:<33}", cfg.server_host.white()));
    println!("│ {:<22} : {} │", "server_port".yellow(), format!("{:<33}", cfg.server_port.to_string().white()));
    println!("│ {:<22} : {} │", "server_url".yellow(), format!("{:<33}", cfg.server_url.white()));

    println!("{}", "├────────────────────────────────────────────────────────────┤".dimmed());
    println!("│ {:<22} : {} │", "surreal_data_path".yellow(), format!("{:<33}", cfg.surreal_data_path.white()));
    println!("│ {:<22} : {} │", "surreal_ns".yellow(), format!("{:<33}", cfg.surreal_ns.white()));
    println!("│ {:<22} : {} │", "surreal_db".yellow(), format!("{:<33}", cfg.surreal_db.white()));

    println!("{}", "├────────────────────────────────────────────────────────────┤".dimmed());
    let short_jwt = if cfg.jwt_secret.len() > 30 {
        format!("{}...", &cfg.jwt_secret[..30])
    } else {
        cfg.jwt_secret.clone()
    };
    println!("│ {:<22} : {} │", "jwt_secret".yellow(), format!("{:<33}", short_jwt.dimmed()));
    println!("│ {:<22} : {} │", "jwt_access_minutes".yellow(), format!("{:<33}", format!("{} min", cfg.jwt_access_minutes).white()));
    println!("│ {:<22} : {} │", "jwt_refresh_days".yellow(), format!("{:<33}", format!("{} days", cfg.jwt_refresh_days).white()));

    println!("{}", "├────────────────────────────────────────────────────────────┤".dimmed());
    let testing_status = if cfg.enable_testing { "true".green().bold() } else { "false".red() };
    println!("│ {:<22} : {:<42} │", "enable_testing".yellow(), testing_status);
    println!("│ {:<22} : {} │", "default_per_second".yellow(), format!("{:<33}", cfg.default_per_second.to_string().white()));
    println!("│ {:<22} : {} │", "default_burst_size".yellow(), format!("{:<33}", cfg.default_burst_size.to_string().white()));
    println!("│ {:<22} : {} │", "testing_per_second".yellow(), format!("{:<33}", cfg.testing_per_second.to_string().white()));
    println!("│ {:<22} : {} │", "testing_burst_size".yellow(), format!("{:<33}", cfg.testing_burst_size.to_string().white()));

    println!("{}", "└────────────────────────────────────────────────────────────┘".dimmed());
    println!();
}