//! Handlers for the `config` command.
//!
//! Everything here is driven by `CONFIG_SCHEMA`: adding a field to the
//! schema in `config_schema.rs` automatically adds it to `config show` and
//! makes it updatable through `config <key> <value>` with no change here.

use super::help::RULE;
use crate::cli::registry::CmdCtx;
use crate::utility::config::Config;
use crate::utility::config_schema::{CONFIG_SCHEMA, config_sections};
use colored::Colorize;

/// Handler for the `config` command.
///
/// With `show`, prints the full configuration. With a key and a value,
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
/// Sections and fields come straight from `CONFIG_SCHEMA`, in schema order,
/// so no field is ever listed here by hand. Sensitive fields are masked.
pub fn print_all() {
    let cfg = Config::get();

    println!();
    println!("{}", "  Server Configuration".cyan().bold());
    println!("{}", RULE.dimmed());

    for section in config_sections() {
        println!();
        println!("  {}", section.white().bold());

        for field in CONFIG_SCHEMA.iter().filter(|f| f.section == section) {
            let value = match cfg.field_str(field.name) {
                Some(v) if field.sensitive => mask(&v),
                Some(v) => v.green().to_string(),
                None => "<missing>".red().to_string(),
            };
            println!("    {:<22} {}", format!("{}:", field.name).white(), value);
        }
    }

    println!();
}

/// Masks a sensitive value, keeping only a short prefix for identification.
///
/// # Arguments
/// * `value` - The raw sensitive value.
fn mask(value: &str) -> String {
    if value.len() > 8 {
        format!("{}...", &value[..4]).dimmed().to_string()
    } else {
        "••••••".dimmed().to_string()
    }
}
