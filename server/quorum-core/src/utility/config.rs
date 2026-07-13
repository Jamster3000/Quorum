// config.rs
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

/// The global configuration singleton.
///
/// Populated once by [`Config::load()`] and never replaced — only the inner [`Arc<Config>`]
/// is swapped on reload, which is what [`ArcSwap`] is for.
static CONFIG: OnceLock<ArcSwap<Config>> = OnceLock::new();

/// Server-wide configuration, loaded from the encrypted `secrets.enc` file at startup.
///
/// All fields are read-only after initialization. To update values at runtime,
/// edit `secrets.enc` and call [`Config::reload()`].
#[derive(Debug)]
pub struct Config {
    /// HTTP port the server binds to (e.g. `3000`).
    pub server_port: u16,

    /// Full server URL including protocol and port (e.g. `http://127.0.0.1:3000`).
    /// Used by the test suite and any internal self-referencing requests.
    pub server_url: String,

    /// IP address the server binds to (e.g. `127.0.0.1` or `0.0.0.0`).
    pub server_host: String,

    /// Path to the directory where SurrealDB stores its on-disk data.
    ///
    /// SurrealDB runs embedded (in-process) using RocksDB as the storage engine.
    /// This directory is created automatically on first startup if it doesn't exist.
    /// Defaults to `./data/db` if not set during setup.
    pub surreal_data_path: String,

    /// SurrealDB namespace to use (e.g. `quorum`).
    pub surreal_ns: String,

    /// SurrealDB database name within the namespace (e.g. `quorum`).
    pub surreal_db: String,

    /// Secret key used to sign and verify JWT access and refresh tokens.
    ///
    /// Auto-generated as a random 32-byte hex string during first-run setup.
    /// Changing this value invalidates all currently issued tokens.
    pub jwt_secret: String,

    /// How long an access token remains valid, in minutes.
    /// Access tokens are short-lived by design — default is 15 minutes.
    pub jwt_access_minutes: i64,

    /// How long a refresh token remains valid, in days.
    /// Refresh tokens are long-lived — default is 7 days.
    pub jwt_refresh_days: i64,

    /// Whether to run the functional and robustness test suite on server startup.
    ///
    /// Should be `false` in production. When `true`, the test suite fires real HTTP
    /// requests against the running server immediately after it becomes ready.
    pub enable_testing: bool,

    /// Rate limit: sustained request rate for standard endpoints (requests per second).
    pub default_per_second: u64,

    /// Rate limit: maximum burst size for standard endpoints.
    ///
    /// Allows short spikes above `default_per_second` up to this many requests
    /// before the limiter kicks in.
    pub default_burst_size: u32,

    /// Rate limit: sustained request rate for test/dev endpoints (requests per second).
    ///
    /// Intentionally higher than `default_per_second` so the test suite doesn't
    /// rate-limit itself during a full run.
    pub testing_per_second: u64,

    /// Rate limit: maximum burst size for test/dev endpoints.
    pub testing_burst_size: u32,
}

impl Config {
    /// Builds a [`Config`] directly from a decrypted [`SerializableConfig`].
    ///
    /// This is the only construction path — there are no environment variable fallbacks.
    /// All values come from the encrypted secrets file, which was populated during first-run setup.
    ///
    /// [`SerializableConfig`]: crate::utility::secrets::SerializableConfig
    fn from_serializable(s: crate::utility::secrets::SerializableConfig) -> Config {
        Config {
            server_url: format!("http://{}:{}", s.server_host, s.server_port),
            server_port: s.server_port,
            server_host: s.server_host,
            surreal_data_path: s.surreal_data_path,
            surreal_ns: s.surreal_ns,
            surreal_db: s.surreal_db,
            jwt_secret: s.jwt_secret,
            jwt_access_minutes: s.jwt_access_minutes,
            jwt_refresh_days: s.jwt_refresh_days,
            enable_testing: s.enable_testing,
            default_per_second: s.default_per_second,
            default_burst_size: s.default_burst_size,
            testing_per_second: s.testing_per_second,
            testing_burst_size: s.testing_burst_size,
        }
    }

    /// Loads configuration from `secrets.enc` and initializes the global [`CONFIG`] singleton.
    ///
    /// On first run (no `secrets.enc` exists), walks the admin through the interactive setup
    /// wizard, prompts for a passphrase, encrypts the config, saves it, then loads it.
    ///
    /// On subsequent runs, prompts for the passphrase and decrypts the existing file.
    ///
    /// This must be called exactly once, before any call to [`Config::get()`].
    /// Calling it a second time returns an error (`"Config already initialized"`).
    ///
    /// # Errors
    /// * Wrong passphrase — AES-GCM auth tag mismatch, decryption fails
    /// * Corrupt or missing `secrets.enc` — file read or deserialization fails
    /// * Setup wizard aborted — terminal I/O error during first-run prompts
    ///
    /// # Example
    /// ```rust
    /// Config::load().expect("Failed to load configuration");
    /// ```
    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        // Check if primary is missing but backup exists
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
                //936
                Self::load_with_passphrase("correct horse battery staple")
            }
        } else {
            let serializable = run_setup()?;

            if !cfg!(debug_assertions) {
                let passphrase = Zeroizing::new(prompt_passphrase_new()?);

                println!();
                typewriter_println(&format!(
                    "{}",
                    "Passphrase setup successfully!".cyan().bold()
                ))
                .map_err(|e| e.to_string())?;

                press_enter_to_continue(true, true);
                save_encrypted_config(&serializable, &passphrase)?;
            } else {
                //936
                save_encrypted_config(&serializable, "correct horse battery staple")?;
            }

            Self::load()
        }
    }

    /// Reloads configuration from `secrets.enc` without restarting the server.
    ///
    /// Prompts the admin for their passphrase, decrypts the file, and atomically swaps
    /// the global config. All subsequent [`Config::get()`] calls will see the new values
    /// immediately, with no downtime and no impact on in-flight requests.
    ///
    /// Useful when you've updated `secrets.enc` (e.g. changed rate limits or JWT expiry)
    /// and want the changes applied without a full server restart.
    ///
    /// # Errors
    /// * Wrong passphrase — decryption fails
    /// * Corrupt `secrets.enc` — deserialization fails
    ///
    /// # Panics
    /// Panics if called before [`Config::load()`].
    pub fn reload() -> Result<(), Box<dyn std::error::Error>> {
        typewriter_println(&format!(
            "{}",
            "Enter passphrase to reload config...".cyan().bold()
        ))
        .map_err(|e| e.to_string())?;

        let passphrase = prompt_passphrase()?;
        let serializable = load_encrypted_config(&passphrase)?;
        let config = Self::from_serializable(serializable);

        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .store(Arc::new(config));

        Ok(())
    }

    pub fn load_with_passphrase(passphrase: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serializable = load_encrypted_config(passphrase)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
        let config = Self::from_serializable(serializable);

        if let Some(existing) = CONFIG.get() {
            existing.store(Arc::new(config));
            Ok(())
        } else {
            CONFIG
                .set(ArcSwap::from_pointee(config))
                .map_err(|_| "Config already initialized".into())
        }
    }

    /// Returns a snapshot of the current configuration.
    ///
    /// The returned guard is cheap to acquire (atomic load, no locking) and reflects
    /// the latest [`Config::reload()`] if one has been called. Treat it like an `Arc<Config>` —
    /// hold it for the duration of a request or operation, then drop it.
    ///
    /// # Panics
    /// Panics if [`Config::load()`] has not been called yet.
    ///
    /// # Example
    /// ```rust
    /// let config = Config::get();
    /// println!("Listening on port {}", config.server_port);
    /// ```
    pub fn get() -> arc_swap::Guard<Arc<Config>> {
        CONFIG
            .get()
            .expect("Config not initialized. Call Config::load() first.")
            .load()
    }

    pub fn update(config_key: &str, config_value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let existing = CONFIG
            .get()
            .ok_or("Config not initialized. Call Config::load() first.")?;
        let current = existing.load();
        let mut new_config = Config {
            server_port: current.server_port,
            server_url: current.server_url.clone(),
            server_host: current.server_host.clone(),
            surreal_data_path: current.surreal_data_path.clone(),
            surreal_ns: current.surreal_ns.clone(),
            surreal_db: current.surreal_db.clone(),
            jwt_secret: current.jwt_secret.clone(),
            jwt_access_minutes: current.jwt_access_minutes,
            jwt_refresh_days: current.jwt_refresh_days,
            enable_testing: current.enable_testing,
            default_per_second: current.default_per_second,
            default_burst_size: current.default_burst_size,
            testing_per_second: current.testing_per_second,
            testing_burst_size: current.testing_burst_size,
        };
        match config_key {
            "server_port" => {
                new_config.server_port = config_value
                    .parse()
                    .map_err(|e| format!("Invalid port format: {}", e))?
            }
            "server_url" => new_config.server_url = config_value.to_string(),
            "server_host" => new_config.server_host = config_value.to_string(),
            "surreal_data_path" => new_config.surreal_data_path = config_value.to_string(),
            "surreal_ns" => new_config.surreal_ns = config_value.to_string(),
            "surreal_db" => new_config.surreal_db = config_value.to_string(),
            "jwt_secret" => new_config.jwt_secret = config_value.to_string(),
            "jwt_access_minutes" => {
                new_config.jwt_access_minutes = config_value
                    .parse()
                    .map_err(|e| format!("Invalid minutes format: {}", e))?
            }
            "jwt_refresh_days" => {
                new_config.jwt_refresh_days = config_value
                    .parse()
                    .map_err(|e| format!("Invalid days format: {}", e))?
            }
            "enable_testing" => {
                new_config.enable_testing = config_value
                    .parse()
                    .map_err(|e| format!("Invalid boolean format: {}", e))?
            }
            "default_per_second" => {
                new_config.default_per_second = config_value
                    .parse()
                    .map_err(|e| format!("Invalid integer format: {}", e))?
            }
            "default_burst_size" => {
                new_config.default_burst_size = config_value
                    .parse()
                    .map_err(|e| format!("Invalid integer format: {}", e))?
            }
            "testing_per_second" => {
                new_config.testing_per_second = config_value
                    .parse()
                    .map_err(|e| format!("Invalid integer format: {}", e))?
            }
            "testing_burst_size" => {
                new_config.testing_burst_size = config_value
                    .parse()
                    .map_err(|e| format!("Invalid integer format: {}", e))?
            }
            _ => return Err(format!("Unknown config key: {}", config_key).into()),
        }
        save_encrypted_config(
            &crate::utility::secrets::SerializableConfig {
                server_port: new_config.server_port,
                server_host: new_config.server_host.clone(),
                surreal_data_path: new_config.surreal_data_path.clone(),
                surreal_ns: new_config.surreal_ns.clone(),
                surreal_db: new_config.surreal_db.clone(),
                jwt_secret: new_config.jwt_secret.clone(),
                jwt_access_minutes: new_config.jwt_access_minutes,
                jwt_refresh_days: new_config.jwt_refresh_days,
                enable_testing: new_config.enable_testing,
                default_per_second: new_config.default_per_second,
                default_burst_size: new_config.default_burst_size,
                testing_per_second: new_config.testing_per_second,
                testing_burst_size: new_config.testing_burst_size,
            },
            "correct horse battery staple",
        )?;

        existing.store(Arc::new(new_config));
        Ok(())
    }
}
