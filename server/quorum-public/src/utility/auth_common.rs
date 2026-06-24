//! This file contains common authentication utilities and functions used across the Quorum server.
//! Specifically functions and code that is/might be commonly used throughout the server, where it doesn't fit in `/db/queries` or `/route`

use crate::models::user::EmailBackupCode;
use rand::rngs::SysRng;
use rand::{RngExt, TryRng};
use sha2::{Digest, Sha256};

/// Generates a random 16 byte salt code to use with hashing backup codes
///
/// Uses the system's random for a more safe and secure random number generator
///
/// # Returns
/// A 16 byte array of random bytes
///
/// # Example
/// ```
/// let salt = generate_salt();
/// ```
pub fn generate_salt() -> String {
    let mut salt_bytes = [0u8; 16];
    SysRng
        .try_fill_bytes(&mut salt_bytes)
        .expect("OS RNG unavailable");
    hex::encode(salt_bytes)
}

/// Hashes a backup code using SHA-256 with a provided salt for secure storage in the database.
///
/// # Arguments
/// * `code` - The backup code to be hashed.
/// * `salt` - The salt to be used in the hashing process.
///
/// # Returns
/// A `String` representing the hexadecimal representation of the hashed backup code.
fn hash_backup_code(code: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

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
/// ```
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
/// ```
/// let backup_codes = generate_backup_codes();
/// ```
pub fn generate_backup_codes() -> Vec<EmailBackupCode> {
    let mut backup_code_array = Vec::with_capacity(10);

    for _ in 0..10 {
        let code = generate_backup_code(24);
        let salt = generate_salt();
        let hashed_code = hash_backup_code(&code, &salt);

        backup_code_array.push(EmailBackupCode {
            plain: Some(code),
            hash: hashed_code,
            salt,
        });
    }

    backup_code_array
}
