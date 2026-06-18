use crate::keys::{PublicKey, SecretKey};
use crate::scheme::{Ciphertext, encrypt, decrypt};

pub fn encrypt_bytes(bytes: &[u8], pk: &PublicKey, q: u64, t: u64) -> Vec<Ciphertext> {
    bytes.iter().map(|&b| encrypt(b as u64, pk, q, t)).collect()
}

pub fn decrypt_bytes(cts: &[Ciphertext], sk: &SecretKey, q: u64, t: u64) -> Vec<u8> {
    cts.iter().map(|ct| decrypt(ct, sk, q, t) as u8).collect()
}

pub fn encrypt_str(s: &str, pk: &PublicKey, q: u64, t: u64) -> Vec<Ciphertext> {
    encrypt_bytes(s.as_bytes(), pk, q, t)
}

pub fn decrypt_str(cts: &[Ciphertext], sk: &SecretKey, q: u64, t: u64) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(decrypt_bytes(cts, sk, q, t))
}