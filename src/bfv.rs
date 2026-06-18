use rand::Rng;
use crate::poly::{Poly, NttPrepared, NTT_PRIME};

pub struct BfvParams {
    //polynomial degree - Number of coefficients in the polynomial
    pub n: usize,
    //ciphertext modulus - "clock size" for arithmetic inside ciphertexts
    pub q: i64,
    //plaintext modulus - "clock size" for arithmetic inside plaintexts
    pub t: i64,
}

impl BfvParams {
    pub fn default() -> Self {
        BfvParams { n: 4096, q: NTT_PRIME, t: 256 }
    }

    pub fn delta(&self) -> i64 { self.q / self.t }
}

pub struct BfvSecretKey(pub Poly);
pub struct BfvPublicKey { pub p0: Poly, pub p1: Poly }

pub struct BfvCiphertext {
    // c0 + c1 * sk = m * delta + e
    pub c0: Poly,
    pub c1: Poly,
}

/// Ciphertext with precomputed NTT of c1*sk baked in.
/// Avoids recomputing the polynomial multiply on every decrypt call.
pub struct BfvCiphertextNtt {
    pub c0: Poly,
    pub c1_sk: Poly, // already c1 * sk in NTT domain, reduced
}

fn sample_noise(n: usize, bound: i64) -> Poly {
    let mut rng = rand::rng();
    Poly::new((0..n).map(|_| rng.random_range(-bound..=bound)).collect(), n)
}

fn sample_uniform(n: usize, q: i64) -> Poly {
    let mut rng = rand::rng();
    Poly::new((0..n).map(|_| rng.random_range(0..q)).collect(), n)
}

fn sample_binary(n: usize) -> Poly {
    let mut rng = rand::rng();
    Poly::new((0..n).map(|_| rng.random_range(0i64..2)).collect(), n)
}

fn scale_down(c: i64, t: i64, q: i64) -> i64 {
    let num = c as i128 * t as i128 + (q as i128 / 2);
    (num / q as i128) as i64
}

// / Generate a secret key and public key for the BFV scheme.
pub fn generate_keys(params: &BfvParams) -> (BfvSecretKey, BfvPublicKey) {
    let n = params.n;
    let q = params.q;
    let sk = sample_binary(n);
    let p1 = sample_uniform(n, q);
    let e  = sample_noise(n, 4);
    let p0 = p1.mul(&sk, q).scale(-1, q).add(&e, q).modq(q);
    (BfvSecretKey(sk), BfvPublicKey { p0, p1 })
}

/// Encrypt a plaintext polynomial using the BFV scheme.
pub fn encrypt(pt: &Poly, pk: &BfvPublicKey, params: &BfvParams) -> BfvCiphertext {
    let n = params.n;
    let q = params.q;
    let delta = params.delta();
    let u  = sample_binary(n);
    let e0 = sample_noise(n, 4);
    let e1 = sample_noise(n, 4);
    let scaled_pt = pt.scale(delta, q);

    // Prepare u once - both pk.p0 and pk.p1 need to multiply by the same u,
    // so we NTT it a single time and reuse it for both instead of doing it twice.
    let u_prepared: NttPrepared = u.ntt_prepared(q);

    let c0 = pk.p0.mul_ntt_precomputed(&u_prepared, q).add(&e0, q).add(&scaled_pt, q).modq(q);
    let c1 = pk.p1.mul_ntt_precomputed(&u_prepared, q).add(&e1, q).modq(q);
    BfvCiphertext { c0, c1 }
}

/// Decrypt a ciphertext polynomial using the BFV scheme.
pub fn decrypt(ct: &BfvCiphertext, sk: &BfvSecretKey, params: &BfvParams) -> Poly {
    let q   = params.q;
    let t   = params.t;
    let raw = ct.c0.add(&ct.c1.mul(&sk.0, q), q).modq(q);
    Poly::new(
        raw.coeffs.iter().map(|&c| scale_down(c, t, q).rem_euclid(t)).collect(),
        params.n,
    )
}

/// Decrypt a rotated ciphertext using a precomputed c1*sk product.
/// This avoids recomputating c1*sk for every offset (which is an expensive poly mul)
/// Only the cheap rotation and addition are done per offset.
pub fn decrypt_with_precomputed(c0: &Poly, c1_sk: &Poly, params: &BfvParams) -> Poly {
    let q = params.q;
    let t = params.t;
    let raw = c0.add(c1_sk, q).modq(q);
    Poly::new(
        raw.coeffs.iter().map(|&c| scale_down(c, t, q).rem_euclid(t)).collect(),
        params.n,
    )
}

pub fn add(ct1: &BfvCiphertext, ct2: &BfvCiphertext, params: &BfvParams) -> BfvCiphertext {
    BfvCiphertext {
        c0: ct1.c0.add(&ct2.c0, params.q),
        c1: ct1.c1.add(&ct2.c1, params.q),
    }
}

pub fn sub(ct1: &BfvCiphertext, ct2: &BfvCiphertext, params: &BfvParams) -> BfvCiphertext {
    BfvCiphertext {
        c0: ct1.c0.sub(&ct2.c0, params.q),
        c1: ct1.c1.sub(&ct2.c1, params.q),
    }
}

/// Encryption but for an array of byes instead of a polynomial.
///The bytes are packed into the coefficients of a polynomial, and then encrypted.
pub fn encrypt_bytes_batched(bytes: &[u8], pk: &BfvPublicKey, params: &BfvParams) -> BfvCiphertext {
    assert!(bytes.len() <= params.n, "Message too long (max {} bytes)", params.n);
    let mut pt = Poly::zero(params.n);
    for (i, &b) in bytes.iter().enumerate() {
        pt.coeffs[i] = b as i64;
    }
    encrypt(&pt, pk, params)
}

pub fn decrypt_bytes_batched(ct: &BfvCiphertext, sk: &BfvSecretKey, params: &BfvParams, len: usize) -> Vec<u8> {
    let pt = decrypt(ct, sk, params);
    pt.coeffs[..len].iter().map(|&c| c.rem_euclid(256) as u8).collect()
}

/// Rotate ciphertext left by k which brings coefficient at index k to index 0.
pub fn rotate_left(ct: &BfvCiphertext, k: usize, params: &BfvParams) -> BfvCiphertext {
    let n = params.n;
    let q = params.q;
    if k == 0 {
        return BfvCiphertext { c0: ct.c0.clone(), c1: ct.c1.clone() };
    }
    let k = k % n;
    let rotate_poly = |p: &Poly| {
        let mut out = vec![0i64; n];
        for (i, &c) in p.coeffs.iter().enumerate() {
            let new_pos = (i + n - k) % n;
            let sign    = if i < k { -1i64 } else { 1i64 };
            out[new_pos] = (out[new_pos] + sign * c).rem_euclid(q);
        }
        Poly::new(out, n)
    };
    BfvCiphertext {
        c0: rotate_poly(&ct.c0),
        c1: rotate_poly(&ct.c1),
    }
}

/// Precompute c1 * sk once for ciphertext
pub fn precompute_c1_sk(ct: &BfvCiphertext, sk: &BfvSecretKey, params: &BfvParams) -> Poly {
    ct.c1.mul(&sk.0, params.q)
}

pub fn encrypt_byte(byte: u8, pk: &BfvPublicKey, params: &BfvParams) -> BfvCiphertext {
    let mut pt = Poly::zero(params.n);
    pt.coeffs[0] = byte as i64;
    encrypt(&pt, pk, params)
}

pub fn decrypt_byte(ct: &BfvCiphertext, sk: &BfvSecretKey, params: &BfvParams) -> u8 {
    decrypt(ct, sk, params).coeffs[0].rem_euclid(256) as u8
}

pub fn bytes_equal(ct_a: &BfvCiphertext, ct_b: &BfvCiphertext, sk: &BfvSecretKey, params: &BfvParams) -> bool {
    let diff = sub(ct_a, ct_b, params);
    decrypt(&diff, sk, params).coeffs[0].rem_euclid(256) == 0
}