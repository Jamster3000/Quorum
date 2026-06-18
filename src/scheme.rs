use rand::Rng;
use crate::keys::{PublicKey, SecretKey};

pub struct Ciphertext {
	pub c0: u64,
	pub c1: u64,
}

pub fn encrypt(message: u64, pk: &PublicKey, q: u64, t: u64) -> Ciphertext {
	assert!(message < t, "Message {message} out of range [0, {t})");

	let mut rng = rand::rng();
	let delta = q / t;
	let r = rng.random_range(0u64..2);
	let noise = rng.random_range(0u64..8);

	Ciphertext {
		c0: (pk.b.wrapping_mul(r).wrapping_add(noise).wrapping_add(delta.wrapping_mul(message))) % q,
		c1: (pk.a.wrapping_mul(r)) % q,
	}
}

pub fn decrypt(ct: &Ciphertext, sk: &SecretKey, q: u64, t: u64) -> u64 {
	//raw = c0 + c1 * sk mod q
	// cancels the public key - leaving delta message + small noise
    let raw = (ct.c0.wrapping_add(ct.c1.wrapping_mul(sk.0))) % q;

    let scaled = (raw as u128 * t as u128) / q as u128;
    (scaled as u64) % t
}