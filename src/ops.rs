use crate::scheme::Ciphertext;

pub fn add(ct1: &Ciphertext, ct2: &Ciphertext, q: u64) -> Ciphertext {
    Ciphertext {
        c0: (ct1.c0.wrapping_add(ct2.c0)) % q,
        c1: (ct1.c1.wrapping_add(ct2.c1)) % q,
    }
}