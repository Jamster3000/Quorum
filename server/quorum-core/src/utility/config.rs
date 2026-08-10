// config.rs (REFACTORED)
//! Centralized configuration management.
//!
//! Configuration is loaded once at startup from an AES-256-GCM encrypted file (`secrets.enc`),
//! decrypted using a passphrase the admin enters at the terminal. No `.env` files, no plaintext
//! secrets on disk.
//!
//! The loaded config is stored in a global [`ArcSwap`] singleton, meaning:
//! - All reads are atomic and lock-free via [`Config::get()`]
//! - The config can be hot-reloaded at runtime via [`Config::reload()`] without restarting the server
//! - Any reload is immediately visible to all subsequent [`Config::get()`] calls across all threads
//!
//! **All configuration fields are defined in `config_schema.rs`** using the `define_config_schema!` macro.
//! This file now handles only the runtime logic (loading, reloading, parsing) — it never duplicates field definitions.

use crate::utility::secrets::{SECRETS_BACKUP_PATH, SECRETS_PATH};
use crate::utility::secrets::{
    load_encrypted_config, prompt_passphrase, prompt_passphrase_new, run_setup,
    save_encrypted_config, secrets_exist,
};
use crate::utility::std::{press_enter_to_continue, typewriter_println};
use arc_swap::ArcSwap;
use colored::Colorize;
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;
use zeroize::Zeroizing;

// Import the generated types and functions from the schema macro
use crate::utility::config_schema::{ConfigFields, parse_config_field};

/// The global configuration singleton.
static CONFIG: OnceLock<ArcSwap<ConfigFields>> = OnceLock::new();

// ===================================================================
// PUBLIC API (Config methods)
// ===================================================================

/// Server-wide configuration, loaded from the encrypted `secrets.enc` file at startup.
pub struct Config;

impl Config {
    /// Loads configuration from `secrets.enc` and initializes the global singleton.
    ///
    /// On first run (no `secrets.enc` exists), walks the admin through the interactive setup
    /// wizard, prompts for a passphrase, encrypts the config, saves it, then loads it.
    ///
    /// On subsequent runs, prompts for the passphrase and decrypts the existing file.
    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(SECRETS_PATH).exists() && Path::new(SECRETS_BACKUP_PATH).exists() {
            let backup_data = std::fs::read(SECRETS_BACKUP_PATH)
                .map_err(|e| format!("Failed to read backup: {}", e))?;
            std::fs::write(SECRETS_PATH, backup_data)
                .map_err(|e| format!("Failed to restore from backup: {}", e))?;
        }

        if secrets_exist() {
            if !cfg!(debug_assertions) {
                println!();
                typewriter_println(&format!(
                    "{}",
                    "Enter passphrase to unlock the server...".cyan().bold()
                ))
                .map_err(|e| e.to_string())?;

                let passphrase = Zeroizing::new(prompt_passphrase()?);
                Self::load_with_passphrase(&passphrase)
            } else {
                Self::load_with_passphrase("correct horse battery staple")
            }
        } else {
            let config_fields = run_setup()?;

            if !cfg!(debug_assertions) {
                let passphrase = Zeroizing::new(prompt_passphrase_new()?);

                println!();
                typewriter_println(&format!(
                    "{}",
                    "Passphrase setup successfully!".cyan().bold()
                ))
                .map_err(|e| e.to_string())?;

                press_enter_to_continue(true, true);
                save_encrypted_config(&config_fields, &passphrase)?;
            } else {
                save_encrypted_config(&config_fields, "correct horse battery staple")?;
            }

            Self::load()
        }
    }

    /// Reloads configuration from `secrets.enc` without restarting the server.
    pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
        typewriter_println(&format!(
            "{}",
            "Enter passphrase to reload config...".cyan().bold()
        ))
        .map_err(|e| e.to_string())?;

        let passphrase = prompt_passphrase()?;
        let config_fields = load_encrypted_config(&passphrase)?;

        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .store(Arc::new(config_fields));

        Ok(())
    }

    /// Loads config with a known passphrase (used during setup and testing).
    pub fn load_with_passphrase(passphrase: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config_fields = load_encrypted_config(passphrase)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

        if let Some(existing) = CONFIG.get() {
            existing.store(Arc::new(config_fields));
            Ok(())
        } else {
            CONFIG
                .set(ArcSwap::from_pointee(config_fields))
                .map_err(|_| "Config already initialized".into())
        }
    }

    /// Returns a snapshot of the current configuration.
    pub fn get() -> arc_swap::Guard<Arc<ConfigFields>> {
        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .load()
    }

    /// Updates a single config field by key.
    ///
    /// # Arguments
    /// * `config_key` - The name of the field to update (e.g., "server_port", "jwt_secret")
    /// * `config_value` - The new value as a string (will be parsed to the field's type)
    ///
    /// # Error
    /// Returns an error if:
    /// - The key is unknown
    /// - The value cannot be parsed to the field's type
    /// - Config is not initialized
    pub fn update(config_key: &str, config_value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let existing = CONFIG
            .get()
            .ok_or("Config not initialized. Call Config::load() first.")?;

        let current = existing.load();
        let mut new_fields = (**current).clone();

        // Try to parse the field
        match parse_config_field(&mut new_fields, config_key, config_value) {
            Ok(true) => {
                // Field was recognized and parsed successfully
                save_encrypted_config(&new_fields, "correct horse battery staple")?;
                existing.store(Arc::new(new_fields));
                Ok(())
            }
            Ok(false) => {
                // Field name not recognized
                Err(format!("Unknown config key: {}", config_key).into())
            }
            Err(e) => {
                // Parse failed
                Err(e.into())
            }
        }
    }
}

pub use crate::utility::config_schema::{CONFIG_SCHEMA, field_names};