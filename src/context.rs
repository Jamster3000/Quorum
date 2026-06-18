use crate::keys::{self, PublicKey, SecretKey};
use crate::scheme::{self, Ciphertext};
use crate::ops;
use crate::text;
use crate::bfv::{self, BfvParams, BfvPublicKey, BfvSecretKey, BfvCiphertext};
use crate::moderation;

const Q: u64 = 1 << 32;
const T: u64 = 256;

pub struct Context {
    pub bfv: BfvParams,
}

impl Context {
    pub fn new() -> Self {
        Context { bfv: BfvParams::default() }
    }

    // ------Integer scheme------

    pub fn generate_keys(&self) -> (SecretKey, PublicKey) {
        keys::generate(Q)
    }

    pub fn encrypt(&self, value: u64, pk: &PublicKey) -> Ciphertext {
        scheme::encrypt(value, pk, Q, T)
    }

    pub fn decrypt(&self, ct: &Ciphertext, sk: &SecretKey) -> u64 {
        scheme::decrypt(ct, sk, Q, T)
    }

    pub fn add(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext {
        ops::add(ct1, ct2, Q)
    }

    pub fn encrypt_bytes(&self, bytes: &[u8], pk: &PublicKey) -> Vec<Ciphertext> {
        text::encrypt_bytes(bytes, pk, Q, T)
    }

    pub fn decrypt_bytes(&self, cts: &[Ciphertext], sk: &SecretKey) -> Vec<u8> {
        text::decrypt_bytes(cts, sk, Q, T)
    }

    pub fn encrypt_str(&self, s: &str, pk: &PublicKey) -> Vec<Ciphertext> {
        text::encrypt_str(s, pk, Q, T)
    }

    pub fn decrypt_str(&self, cts: &[Ciphertext], sk: &SecretKey) -> Result<String, std::string::FromUtf8Error> {
        text::decrypt_str(cts, sk, Q, T)
    }

    //------BFV scheme ------

    pub fn generate_bfv_keys(&self) -> (BfvSecretKey, BfvPublicKey) {
        bfv::generate_keys(&self.bfv)
    }

    /// Encrypt a message as a single batched ciphertext using one polynomial multiplication
    pub fn encrypt_message(&self, msg: &str, pk: &BfvPublicKey) -> BfvCiphertext {
        bfv::encrypt_bytes_batched(msg.as_bytes(), pk, &self.bfv)
    }

    pub fn decrypt_message(&self, ct: &BfvCiphertext, sk: &BfvSecretKey, len: usize) -> String {
        String::from_utf8_lossy(&bfv::decrypt_bytes_batched(ct, sk, &self.bfv, len)).into_owned()
    }

    /// Prepare a pattern for scanning.
    pub fn encrypt_pattern(&self, pattern: &[u8], _pk: &BfvPublicKey) -> Vec<u8> {
        moderation::encrypt_pattern(pattern, _pk, &self.bfv)
    }

    /// Scan an encrypted message against patterns - here, the server only learns true/false, not the actual message.
    pub fn scan(&self, message: &BfvCiphertext, patterns: &[Vec<u8>], sk: &BfvSecretKey, message_len: usize) -> bool {
        moderation::scan(message, patterns, sk, &self.bfv, message_len)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}