use rand::Rng;

pub struct SecretKey(pub u64);

pub struct PublicKey {
	pub a: u64,
	pub b: u64,
}

pub fn generate(q: u64) -> (SecretKey, PublicKey) {
	let mut rng = rand::rng();

	let sk = rng.random_range(1..q);
	let a = rng.random_range(1..q);
	let noise = rng.random_range(1u64..8);

	// b = -(a * sk) + noise mod q
	let b = a.wrapping_mul(sk).wrapping_neg().wrapping_add(noise) % q;

	(SecretKey(sk), PublicKey { a, b})
}