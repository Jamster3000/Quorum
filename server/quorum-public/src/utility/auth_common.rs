//! This file contains common authentication utilities and functions used across the Quorum server.
//! Specifically functions and code that is/might be commonly used throughout the server, where it doesn't fit in `/db/queries` or `/route`
//!
//! All password hashing and verification runs on blocking worker threads via
//! `tokio::task::spawn_blocking`, so the async runtime is never stalled by the
//! CPU- and memory-heavy Argon2 work.

use crate::models::user::EmailBackupCode;
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use email_address::*;
use rand::RngExt;
use rand_core::OsRng;
use tokio::task::spawn_blocking;

/// Argon2 memory cost, in KiB. 65_536  KiB = 64 MiB per hash operation.
const MEMORY_COST_KIB: u32 = 65_536;

/// Argon2 time cost (number of iterations).
const TIME_COST: u32 = 3;

/// Argon2 parallelism (number of lanes).
const PARALLELISM: u32 = 2;

/// Generates the plain text backup codes.
///
/// Uses a `CHARSET` list excluding confusing characters like `0`, `O`, `1`, `l`, etc. to generate a list of backup codes.
///
/// # Arguments
/// * `length` - The length of each backup code.
///
/// # Returns
/// A `Vec<String>` containing the generated backup codes.
///
/// # Example
/// ```rust,ignore
/// let backup_codes = generate_backup_codes(8);
/// ```
pub fn generate_backup_code(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789";
    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Main entry function to generate backup codes, salts and hashing.
///
/// Generates a total of 10 backup codes with the appropriate salts and hashes for secure storage in the database.
/// Each backup code generated is 24 characters long.
///
/// # Returns
/// A `Vec<EmailBackupCode>` containing the generated backup codes, salts, and hashes
///
/// # Example
/// ```rust,ignore
/// let backup_codes = generate_backup_codes().await;
/// ```
pub async fn generate_backup_codes() -> Vec<EmailBackupCode> {
    let mut backup_code_array = Vec::with_capacity(10);
    for _ in 0..10 {
        let code = generate_backup_code(24);
        let hashed_code = hash(&code).await.expect("Failed to hash backup code");
        backup_code_array.push(EmailBackupCode {
            plain: Some(code),
            hash: hashed_code,
        });
    }
    backup_code_array
}

pub fn get_argon2() -> Argon2<'static> {
    let params = Params::new(MEMORY_COST_KIB, TIME_COST, PARALLELISM, Some(32))
        .expect("valid Argon2 params");

    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hashes `plaintext` on a blocking worker thread.
///
/// Argon2 at these parameters allocates hundreds of MiB and burns significant CPU,
/// which would otherwise stall the tokio runtime's async workers.
pub async fn hash(plaintext: &str) -> Result<String, String> {
    let plaintext = plaintext.to_string();

    spawn_blocking(move || {
        let argon2 = get_argon2();
        let salt = SaltString::generate(&mut OsRng);
        argon2
            .hash_password(plaintext.as_bytes(), &salt)
            .map(|phc| phc.to_string())
            .map_err(|e| format!("Failed to hash: {}", e))
    })
    .await
    .map_err(|e| format!("Hashing task failed: {}", e))?
}

/// Verifies `plaintext` against `stored_hash` on a blocking worker thread.
///
/// The memory and time parameters are taken from the stored PHC hash string, not
/// from `get_argon2()`, so hashes created with older parameters still verify.
pub async fn verify(plaintext: &str, stored_hash: &str) -> Result<bool, String> {
    let plaintext = plaintext.to_string();
    let stored_hash = stored_hash.to_string();

    spawn_blocking(move || {
        let argon2 = get_argon2();
        let parsed_hash =
            PasswordHash::new(&stored_hash).map_err(|e| format!("Failed to parse hash: {}", e))?;
        argon2
            .verify_password(plaintext.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| format!("Verification failed: {}", e))
    })
    .await
    .map_err(|e| format!("Verification task failed: {}", e))?
}

pub fn check_email_address(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
