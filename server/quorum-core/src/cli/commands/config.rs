//! Handlers for the `config` command.
//!
//! Everything here is driven by `CONFIG_SCHEMA`: adding a field to the
//! schema in `config_schema.rs` automatically adds it to `config show`,
//! `config keys`, `config <key>`, and makes it updatable through
//! `config <key> <value>` with no change here.

use super::help::RULE;
use crate::cli::registry::CmdCtx;
use crate::utility::config::Config;
use crate::utility::config_schema::{CONFIG_SCHEMA, config_sections};
use colored::Colorize;

/// Handler for the `config` command.
///
/// With no arguments or `show`, prints the full configuration. With `keys`,
/// lists every configuration key with its description and default. With a
/// key, prints that key's current value and details. With a key and a
/// value, updates that configuration setting.
///
/// # Arguments
/// * `ctx` - The command context; `ctx.raw` contains the subcommand, key, or key and value.
///
/// # Example
/// ```rust,ignore
/// config("show").await;            // prints the configuration
/// config("keys").await;            // lists all configuration keys
/// config("server_host").await;     // prints one key's value and details
/// ```
pub async fn config(ctx: CmdCtx) {
    let mut parts = ctx.raw.split_whitespace();
    match parts.next() {
        None => print_all(),
        Some("show") => print_all(),
        Some("keys") => print_keys(),
        Some(key) => {
            let value = parts.collect::<Vec<_>>().join(" ");
            if value.is_empty() {
                print_field(key);
            } else {
                change_config_value(key, &value);
            }
        }
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
    println!(
        "  {}",
        "Run `config:config keys` to list all keys, `config:config <key>` for one key's details, or `config:config <key> <value>` to update."
            .dimmed()
    );
    println!();
}

/// Prints every configuration key with its description and default, grouped
/// into sections.
pub fn print_keys() {
    println!();
    println!("{}", "  Configuration Keys".cyan().bold());
    println!("{}", RULE.dimmed());

    for section in config_sections() {
        println!();
        println!("  {}", section.white().bold());

        for field in CONFIG_SCHEMA.iter().filter(|f| f.section == section) {
            println!(
                "    {:<22} {}",
                field.name.green(),
                field.description.dimmed()
            );
        }
    }

    println!();
    println!(
        "  {}",
        "Run `config:config <key>` to see a key's current value, or `config:config <key> <value>` to update it."
            .dimmed()
    );
    println!();
}

/// Prints the current value, default, and description of a single key.
///
/// # Arguments
/// * `key` - The configuration key to look up (e.g., "server_host").
fn print_field(key: &str) {
    let cfg = Config::get();

    match CONFIG_SCHEMA.iter().find(|f| f.name == key) {
        None => println!(
            "{}",
            format!(
                "Unknown config key: {}. Run `config keys` to see all valid keys.",
                key
            )
            .red()
        ),
        Some(field) => {
            let value = match cfg.field_str(field.name) {
                Some(v) if field.sensitive => mask(&v),
                Some(v) => v.green().to_string(),
                None => "<missing>".red().to_string(),
            };

            println!();
            println!("  {}", field.name.cyan().bold());
            println!("{}", RULE.dimmed());
            println!(
                "  {:<14} {}",
                "Description:".white(),
                field.description.dimmed()
            );
            println!("  {:<14} {}", "Section:".white(), field.section.dimmed());
            println!("  {:<14} {}", "Current:".white(), value);
            println!(
                "  {:<14} {}",
                "Default:".white(),
                format_default(field.default_str).dimmed()
            );
            println!(
                "  {:<14} {}",
                "Sensitive:".white(),
                if field.sensitive {
                    "yes - masked in output".yellow().to_string()
                } else {
                    "no".dimmed().to_string()
                }
            );
            println!();
            println!(
                "  {}",
                format!("Update with: config {} <value>", field.name).dimmed()
            );
            println!();
        }
    }
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

/// Cleans up a schema default for display, e.g. `"127.0.0.1".to_string()`
/// becomes `127.0.0.1`.
///
/// # Arguments
/// * `default_str` - The raw stringified default expression from the schema.
fn format_default(default_str: &str) -> String {
    default_str
        .trim_end_matches(".to_string()")
        .trim_matches('"')
        .to_string()
}
