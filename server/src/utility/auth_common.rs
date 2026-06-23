use crate::models::user::EmailBackupCode;
use rand::rngs::SysRng;
use rand::{RngExt, TryRng};
use sha2::{Digest, Sha256};

pub fn generate_salt() -> String {
    let mut salt_bytes = [0u8; 16];
    SysRng
        .try_fill_bytes(&mut salt_bytes)
        .expect("OS RNG unavailable");
    hex::encode(salt_bytes)
}

fn hash_backup_code(code: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

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
