//! This file contains common authentication utilities and functions used across the Quorum server.
//! Specifically functions and code that is/might be commonly used throughout the server, where it doesn't fit in `/db/queries` or `/route`
//!
//! Hashing is split by threat model:
//! * Human passwords use Argon2id (memory-hard, on blocking threads via `spawn_blocking`).
//! * Server-generated secrets (backup codes) use plain SHA-256 — they carry ~140 bits of
//!   entropy, so brute force is infeasible regardless of hash speed.

use crate::models::user::EmailBackupCode;
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use email_address::*;
use rand::RngExt;
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use tokio::task::spawn_blocking;

/// Argon2 memory cost, in KiB. 65_536 KiB = 64 MiB per hash operation.
/// Note: verification always uses the parameters embedded in the stored PHC
/// hash string, so lowering this only affects newly created hashes.
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

/// Main entry function to generate backup codes and hash them for storage.
///
/// Generates a total of 10 backup codes with SHA-256 hashes for secure storage in the database.
/// Each backup code generated is 24 characters long.
///
/// Backup codes are random server-generated secrets (~140 bits of entropy), so a fast
/// SHA-256 hash is sufficient — the same reasoning as refresh tokens in `tokens.rs`.
///
/// # Returns
/// A `Vec<EmailBackupCode>` containing the generated backup codes and their hashes
///
/// # Example
/// ```rust,ignore
/// let backup_codes = generate_backup_codes();
/// ```
pub fn generate_backup_codes() -> Vec<EmailBackupCode> {
    let mut backup_code_array = Vec::with_capacity(10);
    for _ in 0..10 {
        let code = generate_backup_code(24);
        let hashed_code = hash_backup_code(&code);
        backup_code_array.push(EmailBackupCode {
            plain: Some(code),
            hash: hashed_code,
        });
    }
    backup_code_array
}

/// Hashes a server-generated secret with SHA-256, hex-encoded.
///
/// Mirrors `tokens.rs::hash_token`. Backup codes carry ~140 bits of entropy, so
/// brute-forcing the hash is infeasible and a memory-hard KDF would add nothing.
fn hash_backup_code(code: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn get_argon2() -> Argon2<'static> {
    let params = Params::new(MEMORY_COST_KIB, TIME_COST, PARALLELISM, Some(32))
        .expect("valid Argon2 params");

    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hashes a human password on a blocking worker thread.
///
/// Argon2 allocates tens of MiB and burns significant CPU, which would otherwise
/// stall the tokio runtime's async workers.
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

/// Verifies a human password against its stored hash on a blocking worker thread.
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

/// Verifies a backup code against a stored SHA-256 hash.
///
/// Hashes the presented code and compares in constant time, so response timing
/// does not reveal how much of the stored hash matches.
#[allow(dead_code)]
pub fn verify_backup_code(code: &str, stored_hash: &str) -> bool {
    let computed = hash_backup_code(code);
    if computed.len() != stored_hash.len() {
        return false;
    }
    computed
        .bytes()
        .zip(stored_hash.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

pub fn check_email_address(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
