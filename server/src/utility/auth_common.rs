use crate::models::user::EmailBackupCode;
use sha2::{Digest, Sha256};
use rand::distr::Alphanumeric;
use rand::RngExt;

pub fn generate_salt() -> String {
    let salt_bytes: [u8; 16] = rand::rng().random();
    hex::encode(salt_bytes)
}

fn hash_backup_code(code: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hasher.update(salt.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn generate_backup_code(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn generate_backup_codes() -> Vec<EmailBackupCode> {
    let mut backup_code_array = Vec::with_capacity(10);

    for _ in 0..10 {
        let code = generate_backup_code(12);
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