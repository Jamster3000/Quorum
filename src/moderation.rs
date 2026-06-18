use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rayon::prelude::*;
use crate::bfv::{self, BfvCiphertext, BfvPublicKey, BfvSecretKey, BfvParams};
use crate::poly::Poly;

pub fn encrypt_pattern(pattern: &[u8], _pk: &BfvPublicKey, _params: &BfvParams) -> Vec<u8> {
    pattern.to_vec()
}

/// Read `len` decoded bytes from a poly starting at `offset`, without allocating a rotated copy.
/// Rotation by k = read from index (offset + i) % n, with a sign flip
#[inline]
fn read_decoded_at(c0: &Poly, c1_sk: &Poly, offset: usize, len: usize, params: &BfvParams) -> Vec<u8> {
    let n = params.n;
    let q = params.q;
    let t = params.t;

    (0..len).map(|i| {
        let src = offset + i;
        let idx = src % n;
        // rotation by `offset` means coeff i of rotated = sign * coeff[(i+offset)%n] of original
        let sign: i64 = if src >= n { -1 } else { 1 }; // sign is -1 only for wrapped-around indices

        let raw_c0 = (sign * c0.coeffs[idx]).rem_euclid(q);
        let raw_c1 = (sign * c1_sk.coeffs[idx]).rem_euclid(q);
        let raw = (raw_c0 + raw_c1).rem_euclid(q);
        let scaled = (raw as i128 * t as i128 + (q as i128 / 2)) / q as i128;
        (scaled as i64).rem_euclid(t) as u8
    }).collect()
}

pub fn scan(
    message_ct: &BfvCiphertext,
    patterns: &[Vec<u8>],
    sk: &BfvSecretKey,
    params: &BfvParams,
    message_len: usize,
) -> bool {
    let c1_sk = Arc::new(bfv::precompute_c1_sk(message_ct, sk, params));
    let c0    = Arc::new(message_ct.c0.clone());

    let mut sorted_patterns: Vec<&Vec<u8>> = patterns.iter().collect();
    sorted_patterns.sort_by_key(|p| p.len());

    for pattern in sorted_patterns {
        if pattern.is_empty() || pattern.len() > message_len {
            continue;
        }

        let offsets: Vec<usize> = (0..=(message_len - pattern.len())).collect();
        let found = Arc::new(AtomicBool::new(false));

        let c0_ref    = Arc::clone(&c0);
        let c1_sk_ref = Arc::clone(&c1_sk);
        let found_ref = Arc::clone(&found);

        offsets.par_iter().for_each(|&offset| {
            if found_ref.load(Ordering::Relaxed) {
                return;
            }

            let bytes = read_decoded_at(&c0_ref, &c1_sk_ref, offset, pattern.len(), params);

            if bytes == pattern.as_slice() {
                found_ref.store(true, Ordering::Relaxed);
            }
        });

        if found.load(Ordering::Relaxed) {
            return true;
        }
    }

    false
}