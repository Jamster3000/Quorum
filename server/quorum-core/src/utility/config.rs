//! Centralized configuration management.
//!
//! This module loads all environment variables once at application startup and provides
//! a singleton `Config` struct that can be accessed from anywhere in the application.
//! This eliminates the need to repeatedly read `.env` files and ensures consistent configuration
//! values across the entire application.
//!
//! Unlike a `OnceLock`, the config can be reloaded at runtime via `Config::reload()` without
//! restarting the server. All reads are atomic and lock-free via `arc-swap`.

use arc_swap::ArcSwap;
use std::sync::Arc;
use std::sync::OnceLock;
use crate::utility::std::{typewriter_println, press_enter_to_continue};
use colored::Colorize;
use crate::utility::secrets::{load_encrypted_config, run_setup, save_encrypted_config, prompt_passphrase, secrets_exist};

#[derive(Debug)]
pub struct Config {
    /// HTTP server port number.
    pub server_port: u16,

    /// Full server URL including protocol and port (e.g., `http://127.0.0.1:3000`).
    /// Used by clients and tests to connect to the server.
    pub server_url: String,

    /// Server host/IP address to bind to.
    pub server_host: String,

    /// SurrealDB connection URL.
    pub surreal_url: String,

    /// SurrealDB root username for authentication.
    pub surreal_user: String,

    /// SurrealDB root password for authentication.
    pub surreal_pass: String,

    /// SurrealDB namespace name.
    pub surreal_ns: String,

    /// SurrealDB database name.
    pub surreal_db: String,

    /// Secret key for signing and verifying JWT tokens.
    pub jwt_secret: String,

    /// Access token expiry time in minutes.
    pub jwt_access_minutes: i64,

    /// Refresh token expiry time in days.
    pub jwt_refresh_days: i64,

    /// Whether to run functional tests on server startup.
    pub enable_testing: bool,

    /// Rate limit: requests per second for default endpoints.
    pub default_per_second: u64,

    /// Rate limit: burst size for default endpoints.
    pub default_burst_size: u32,

    /// Rate limit: requests per second for testing endpoints.
    pub testing_per_second: u64,

    /// Rate limit: burst size for testing endpoints.
    pub testing_burst_size: u32,
}

static CONFIG: OnceLock<ArcSwap<Config>> = OnceLock::new();

impl Config {
    /// Parses all environment variables into a `Config` instance.
    fn build(jwt_secret: String, surreal_pass: String) -> Result<Config, Box<dyn std::error::Error>> {
        let server_port: u16 = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;

        let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        Ok(Config {
            server_port,
            server_host: server_host.clone(),
            server_url: std::env::var("SERVER_URL")
                .unwrap_or_else(|_| format!("http://{}:{}", server_host, server_port)),
            surreal_url: std::env::var("SURREAL_URL")?,
            surreal_user: std::env::var("SURREAL_USER")?,
            surreal_pass,
            surreal_ns: std::env::var("SURREAL_NS")?,
            surreal_db: std::env::var("SURREAL_DB")?,
            jwt_secret,
            jwt_access_minutes: std::env::var("JWT_ACCESS_MINUTES")?.parse()?,
            jwt_refresh_days: std::env::var("JWT_REFRESH_DAYS")?.parse()?,
            enable_testing: std::env::var("ENABLE_TESTING")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            default_per_second: std::env::var("DEFAULT_PER_SECOND")
                .unwrap_or_else(|_| "2".to_string())
                .parse()?,
            default_burst_size: std::env::var("DEFAULT_BURST_SIZE")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            testing_per_second: std::env::var("TESTING_PER_SECOND")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            testing_burst_size: std::env::var("TESTING_BURST_SIZE")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
        })
    }

    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        if secrets_exist() {
            println!();
            typewriter_println(&format!("{}", "Enter passphrase to unlock the server...".cyan().bold())).map_err(|e| e.to_string())?;

            let passphrase = prompt_passphrase()?;
            let serializable_config = load_encrypted_config(&passphrase)?;

            let config = Config {
                server_port: serializable_config.server_port,
                server_host: serializable_config.server_host.clone(),
                server_url: format!(
                    "http://{}:{}",
                    serializable_config.server_host,
                    serializable_config.server_port
                ),
                surreal_url: serializable_config.surreal_url,
                surreal_user: serializable_config.surreal_user,
                surreal_pass: serializable_config.surreal_pass,
                surreal_ns: serializable_config.surreal_ns,
                surreal_db: serializable_config.surreal_db,
                jwt_secret: serializable_config.jwt_secret,
                jwt_access_minutes: serializable_config.jwt_access_minutes,
                jwt_refresh_days: serializable_config.jwt_refresh_days,
                enable_testing: serializable_config.enable_testing,
                default_per_second: serializable_config.default_per_second,
                default_burst_size: serializable_config.default_burst_size,
                testing_per_second: serializable_config.testing_per_second,
                testing_burst_size: serializable_config.testing_burst_size,
            };
            CONFIG.set(ArcSwap::from_pointee(config))
                .map_err(|_| "Config already initialized".into())
        } else {
            let serializable_config = run_setup()?;
            let passphrase = prompt_passphrase()?;

            println!();
            typewriter_println(&format!("{}", "Passphrase setup successfully!".cyan().bold())).map_err(|e| e.to_string())?;

            press_enter_to_continue(true, true);

            save_encrypted_config(&serializable_config, &passphrase)?;
            Self::load()
        }
    }


    /// Reloads configuration from environment variables without restarting the server.
    /// All subsequent `Config::get()` calls will see the new values atomically.
    ///
    /// # Errors
    /// * Same as `Config::load()` — missing or invalid environment variables
    /// * Panics if called before `Config::load()`
    pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
        let current = Self::get();
        let jwt_secret = current.jwt_secret.clone();
        let surreal_pass = current.surreal_pass.clone();
        drop(current);

        let config = Self::build(jwt_secret, surreal_pass)?;
        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .store(Arc::new(config));
        Ok(())
    }

    /// Returns a snapshot of the current configuration.
    /// The snapshot is cheap to load and reflects any reloads since the last call.
    ///
    /// # Panics
    /// Panics if `Config::load()` has not been called yet.
    pub fn get() -> arc_swap::Guard<Arc<Config>> {
        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .load()
    }
}
