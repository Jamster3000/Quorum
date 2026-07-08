//! This file includes everything for server configuration setup and hashing.
//!
//! It includes functions for encrypting and decrypting the server configuration
//! using AES-256-GCM, as well as functions for saving and loading the encrypted configuration
//! from a file. It also provides a setup process that prompts the user
//! for necessary configurations and generates secure defaults.

use crate::utility::std::{press_enter_to_continue, typewriter_println};
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit},
};
use colored::Colorize;
use dialoguer::Password;
use rand_core::OsRng;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use zeroize::Zeroizing;
use zxcvbn::{zxcvbn, Score};
use crate::startup;

fn generate_random_bytes() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

const SECRETS_PATH: &str = "secrets.enc";

#[derive(Serialize, Deserialize)]
pub struct SerializableConfig {
    pub server_port: u16,
    pub server_host: String,
    pub surreal_data_path: String,
    pub surreal_ns: String,
    pub surreal_db: String,
    pub jwt_secret: String,
    pub jwt_access_minutes: i64,
    pub jwt_refresh_days: i64,
    pub enable_testing: bool,
    pub default_per_second: u64,
    pub default_burst_size: u32,
    pub testing_per_second: u64,
    pub testing_burst_size: u32,
}

/// Derives a key from the given passphrase and salt using Argon2id.
///
/// # Arguments
/// * `passphrase` - The passphrase to derive the key from.
/// * `salt` - A 32-byte salt used in the key derivation process.
///
/// # Returns
/// A `Zeroizing<[u8; 32]>` containing the derived key. The
fn derive_key(passphrase: &str, salt: &[u8; 32]) -> Zeroizing<[u8; 32]> {
    use argon2::{Algorithm, Params, Version};
    let argon2 = argon2::Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(65536, 3, 1, Some(32)).unwrap(),
    );
    let mut key = Zeroizing::new([0u8; 32]);
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut *key)
        .expect("Failed to derive key");
    key
}

/// Encrypts the given data using AES-256-GCM with a key derived from the provided passphrase.
///
/// # Arguments
/// * `data` - The plaintext data to encrypt.
/// * `passphrase` - The passphrase used to derive the encryption key.
///
/// # Returns
/// A `Result` containing the encrypted data as a `Vec<u8>` on success, or an error message as a `String` on failure.
fn encrypt(data: &[u8], passphrase: &str) -> Result<Vec<u8>, String> {
    let salt = generate_random_bytes();
    let key = derive_key(passphrase, &salt);
    let cipher = Aes256Gcm::new_from_slice(&*key).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    cipher
        .encrypt(&nonce, data)
        .map(|mut ciphertext| {
            let mut result = Vec::with_capacity(32 + 12 + ciphertext.len());
            result.extend_from_slice(&salt);
            result.extend_from_slice(nonce.as_slice());
            result.append(&mut ciphertext);
            result
        })
        .map_err(|e| format!("Encryption failed: {}", e))
}

/// Decrypts the given encrypted data using AES-256-GCM with a key derived from the provided passphrase.
///
/// # Arguments
/// * `encrypted_data` - The encrypted data to decrypt, which should include the salt and nonce.
/// * `passphrase` - The passphrase used to derive the decryption key.
///
/// # Returns
/// A `Result` containing the decrypted data as a `Vec<u8>` on success, or an error message as a `String` on failure.
fn decrypt(encrypted_data: &[u8], passphrase: &str) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < 44 {
        return Err("Encrypted data too short".to_string());
    }
    let (salt_bytes, rest) = encrypted_data.split_at(32);
    let (nonce_bytes, ciphertext) = rest.split_at(12);
    let salt: [u8; 32] = salt_bytes.try_into().map_err(|_| "Invalid salt length")?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let key = derive_key(passphrase, &salt);
    let cipher = Aes256Gcm::new_from_slice(&*key).unwrap();
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}

/// Saves the given configuration to an encrypted file using the provided passphrase.
///
/// # Arguments
/// * `config` - The configuration to save.
/// * `passphrase` - The passphrase used to encrypt the configuration.
///
/// # Returns
/// A `Result` indicating success or failure, with an error message as a `String` on failure.
///
/// # Example
/// ```rust
/// let config = SerializableConfig { ... };
/// save_encrypted_config(&config, "my_secure_passphrase").expect("Failed to save encrypted config");
/// ```
pub fn save_encrypted_config(config: &SerializableConfig, passphrase: &str) -> Result<(), String> {
    let json = serde_json::to_vec(config).map_err(|e| format!("Serialization failed: {}", e))?;
    let encrypted = encrypt(&json, passphrase)?;
    let mut file =
        File::create(SECRETS_PATH).map_err(|e| format!("Failed to create secrets.enc: {}", e))?;
    file.write_all(&encrypted)
        .map_err(|e| format!("Failed to write secrets.enc: {}", e))?;
    Ok(())
}

/// Loads and decrypts the configuration from the encrypted file using the provided passphrase.
///
/// # Arguments
/// * `passphrase` - The passphrase used to decrypt the configuration.
///
/// # Returns
/// A `Result` containing the decrypted configuration as a `SerializableConfig` on success, or an error message as a `String` on failure.
///
/// # Example
/// ```rust
/// let config = load_encrypted_config("my_secure_passphrase").expect("Failed to load encrypted config");
/// ```
pub fn load_encrypted_config(passphrase: &str) -> Result<SerializableConfig, String> {
    let encrypted =
        fs::read(SECRETS_PATH).map_err(|e| format!("Failed to read secrets.enc: {}", e))?;
    let decrypted = decrypt(&encrypted, passphrase)?;
    serde_json::from_slice(&decrypted).map_err(|e| format!("Failed to deserialize config: {}", e))
}

/// Checks if the encrypted secrets file exists.
///
/// # Returns
/// `true` if the secrets file exists, `false` otherwise.
///
/// # Example
/// ```rust
/// if secrets_exist() {
///     println!("Secrets file exists.");
/// } else {
///     println!("Secrets file does not exist.");
/// }
/// ```
pub fn secrets_exist() -> bool {
    Path::new(SECRETS_PATH).exists()
}

/// Helper function that maps a `dialoguer::Error` to a `String` for easier error handling.
///
/// # Arguments
/// * `e` - The `dialoguer::Error` to map.
///
/// # Returns
/// A `String` containing the error message.
fn map_dialoguer_error(e: dialoguer::Error) -> String {
    format!("Error: {:?}", e)
}

/// Runs setup process
///
/// This runs the setup process for the Quorum server,
/// prompting the user for necessary configurations and returning
/// a `SerializableConfig` object.
///
/// # Returns
/// A `Result` containing the `SerializableConfig` on success, or an error message as a `String` on failure.
///
/// # Example
/// ```rust
/// let config = run_setup().expect("Failed to run setup");
/// ```
pub fn run_setup() -> Result<SerializableConfig, String> {
    if !cfg!(debug_assertions) {
        typewriter_println(&format!(
            "{}",
            "\nWelcome to Quorum Server Setup!".cyan().bold()
        ))
        .map_err(|e| e.to_string())?;
        println!();

        typewriter_println(&format!(
            "{}",
            "Your server configuration will be encrypted with a passphrase.".dimmed()
        ))
        .map_err(|e| e.to_string())?;
        typewriter_println(&format!(
            "{}",
            "You'll need this passphrase every time you start the server.".dimmed()
        ))
        .map_err(|e| e.to_string())?;
        println!();

        press_enter_to_continue(false, false);
        print!("\x1B[2J\x1B[1;1H");
        startup::print_banner();
    }

    Ok(SerializableConfig {
        server_port: 3000,
        server_host: "127.0.0.1".to_string(),
        surreal_data_path: "./data/db".to_string(),
        surreal_ns: "quorum".to_string(),
        surreal_db: "quorum".to_string(),
        jwt_secret: hex::encode(generate_random_bytes()),
        jwt_access_minutes: 15,
        jwt_refresh_days: 7,
        enable_testing: false,
        default_per_second: 100,
        default_burst_size: 200,
        testing_per_second: 1000,
        testing_burst_size: 2000,
    })
}

/// Prompts the user for a passphrase to encrypt/decrypt the server configuration.
///
/// # Returns
/// A `Result` containing the entered passphrase as a `String` on success, or an error message as a `String` on failure.
///
/// # Example
/// ```rust
/// let passphrase = prompt_passphrase().expect("Failed to get passphrase");
/// ```
pub fn prompt_passphrase() -> Result<String, String> {
    Password::new()
        .with_prompt("Enter server passphrase")
        .interact()
        .map_err(map_dialoguer_error)
}

pub fn prompt_passphrase_new() -> Result<String, String> {
    loop {
        let passphrase = Password::new()
            .with_prompt("Enter server passphrase")
            .with_confirmation("Confirm passphrase", "Passphrases do not match")
            .interact()
            .map_err(map_dialoguer_error)?;

        print!("\x1B[2J\x1B[1;1H");
        startup::print_banner();

        match validate_passphrase(&passphrase) {
            Ok(()) => return Ok(passphrase),
            Err(err) => eprintln!("{}", err.red()),
        }
    }
}

pub fn verify_admin_credentials(
    username: &str,
    password: &str,
    stored_username: &str,
    stored_hash: &str,
) -> Result<(), String> {
    use argon2::{
        Algorithm, Argon2, Params, Version,
        password_hash::{PasswordHash, PasswordVerifier},
    };

    if username != stored_username {
        return Err("Invalid credentials".to_string());
    }

    let params = Params::new(65536, 3, 1, Some(32)).map_err(|e| e.to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| format!("Failed to parse stored hash: {}", e))?;

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid credentials".to_string())
}

pub fn validate_passphrase(passphrase: &str) -> Result<(), String> {
    let len = passphrase.chars().count();

    if len < 12 {
        return Err("Use at least 12 characters.".into());
    }

    if len > 64 {
        return Err("Passphrase is too long for this field.".into());
    }

    let estimate = zxcvbn(passphrase, &[]);

    if estimate.score() < Score::Three {
        if let Some(feedback) = estimate.feedback() {
            let warning = feedback
                .warning()
                .map(|w| w.to_string())
                .unwrap_or_default();

            let suggestions = feedback
                .suggestions()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            let message = format!("{} {}", warning, suggestions).trim().to_string();

            if message.is_empty() {
                return Err("Passphrase is too weak.".into());
            }

            return Err(format!("Passphrase is too weak. {message}"));
        }

        return Err("Passphrase is too weak.".into());
    }

    Ok(())
}
