//! CLI commands for inspecting and updating the server configuration.

use super::registry::CmdCtx;
use crate::utility::config::Config;
use crate::utility::config_schema::CONFIG_SCHEMA;
use colored::Colorize;
use serde_json::Value;

/// Keys whose values are masked in `config:show` output.
const SENSITIVE_KEYS: &[&str] = &["jwt_secret"];

/// Renders every config field with its current value.
pub async fn show(ctx: &mut CmdCtx<'_>) {
    let v: Value = serde_json::to_value(&*Config::get()).expect("config is serializable");

    for field in CONFIG_SCHEMA {
        let raw = &v[field.name].to_string().trim_matches('"').to_string();
        let displayed = if SENSITIVE_KEYS.contains(&field.name) {
            "▪▪▪▪▪▪▪▪ (hidden)".dimmed().to_string() //TODO: this is weird and likely needs changing
        } else {
            raw.white().to_string()
        };
        println!("  {:<24} : {:<}{:>8}", field.name.yellow(), displayed, "");
    }
}

/// Updates one config field: `config:set <key> <value>`.
pub async fn set(ctx: &mut CmdCtx<'_>) {
    let mut parts = ctx.raw.split_whitespace();
    let (Some(key), Some(value)) = (parts.next(), parts.collect::<Vec<_>>().join(" ")) else {
        println!("{}", "Usage: config:set <key> <value>".red());
        return;
    };

    if !CONFIG_SCHEMA.iter().any(|f| f.name == key) {
        println!(
            "{} {} — unknown key. Run `config:show` to list keys.",
            "Error:".red(),
            key.white()
        );
        return;
    }

    match Config::update(key, &value) {
        Ok(_) => println!("{}", format!("  {key} updated successfully.").green()),
        Err(e) => println!("{}", format!("  Failed to update config: {e}").red()),
    }
}
