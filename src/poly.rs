use crate::simd;

#[derive(Clone, Debug)]
pub struct Poly {
    pub coeffs: Vec<i64>,
    pub n: usize,
}

/// Computes (a * b) mod m, ensuring that the result is non-negative and fits within the range of i64.
#[inline(always)]
fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    let a = a.rem_euclid(m) as u64;
    let b = b.rem_euclid(m) as u64;
    ((a as u128 * b as u128) % m as u64 as u128) as i64
}

/// Computes (base ^ exp) mod m using exponentiation by squaring, ensuring that the result is non-negative and fits within the range of i64.
fn pow_mod(base: i64, exp: i64, m: i64) -> i64 {
    let mut result = 1u64;
    let mut base = base.rem_euclid(m) as u64;
    let mut exp = exp as u64;
    let m64 = m as u64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % m64 as u128) as u64;
        }
        base = ((base as u128 * base as u128) % m64 as u128) as u64;
        exp >>= 1;
    }
    result as i64
}

/// Performs the Number Theoretic Transform (NTT) on the input vector `a` in place, using the modulus `q` and the primitive root of unity `omega`.
fn ntt_forward(a: &mut Vec<i64>, q: i64, omega: i64) {
    let n = a.len();
    let qu = q as u64;

    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { a.swap(i, j); }
    }

    let mut len = 2;
    while len <= n {
        let w = pow_mod(omega, (n / len) as i64, q);
        let wu = w.rem_euclid(q) as u64;
        for i in (0..n).step_by(len) {
            let mut wn = 1u64;
            for k in 0..len / 2 {
                let u = a[i + k].rem_euclid(q) as u64;
                let v = ((a[i + k + len/2].rem_euclid(q) as u128 * wn as u128) % qu as u128) as u64;
                a[i + k] = ((u + v) % qu) as i64;
                a[i + k + len / 2] = ((u + qu - v) % qu) as i64;
                wn = ((wn as u128 * wu as u128) % qu as u128) as u64;
            }
        }
        len <<= 1;
    }
}

/// Performs the inverse Number Theoretic Transform (NTT) on the input vector `a` in place, using the modulus `q` and the primitive root of unity `omega`.
fn ntt_inverse(a: &mut Vec<i64>, q: i64, omega: i64) {
    let omega_inv = pow_mod(omega, q - 2, q);
    ntt_forward(a, q, omega_inv);
    let n = a.len();
    let n_inv = pow_mod(n as i64, q - 2, q);
    for x in a.iter_mut() {
        *x = mul_mod(*x, n_inv, q);
    }
}

impl Poly {
    pub fn zero(n: usize) -> Self {
        Poly { coeffs: vec![0; n], n }
    }

    pub fn new(coeffs: Vec<i64>, n: usize) -> Self {
        assert_eq!(coeffs.len(), n);
        Poly { coeffs, n }
    }

    pub fn modq(&self, q: i64) -> Self {
        Poly {
            coeffs: self.coeffs.iter().map(|&c| c.rem_euclid(q)).collect(),
            n: self.n,
        }
    }

    pub fn add(&self, other: &Poly, q: i64) -> Poly {
        assert_eq!(self.n, other.n);
        let mut dst = vec![0i64; self.n];
        simd::add_mod(&self.coeffs, &other.coeffs, q, &mut dst);
        Poly { coeffs: dst, n: self.n }
    }

    pub fn sub(&self, other: &Poly, q: i64) -> Poly {
        assert_eq!(self.n, other.n);
        let mut dst = vec![0i64; self.n];
        simd::sub_mod(&self.coeffs, &other.coeffs, q, &mut dst);
        Poly { coeffs: dst, n: self.n }
    }

    pub fn mul(&self, other: &Poly, q: i64) -> Poly {
        assert_eq!(self.n, other.n);
        let n       = self.n;
        let psi     = PSI;
        let omega   = mul_mod(psi, psi, q);
        let psi_inv = pow_mod(psi, q - 2, q);

        let mut a  = self.coeffs.clone();
        let mut b  = other.coeffs.clone();
        let mut pw = 1i64;
        for i in 0..n {
            a[i] = mul_mod(a[i], pw, q);
            b[i] = mul_mod(b[i], pw, q);
            pw   = mul_mod(pw, psi, q);
        }

        ntt_forward(&mut a, q, omega);
        ntt_forward(&mut b, q, omega);

        for i in 0..n {
            a[i] = mul_mod(a[i], b[i], q);
        }

        ntt_inverse(&mut a, q, omega);

        pw = 1i64;
        for x in a.iter_mut() {
            *x = mul_mod(*x, pw, q);
            pw = mul_mod(pw, psi_inv, q);
        }

        Poly { coeffs: a, n }
    }

    pub fn scale(&self, scalar: i64, q: i64) -> Poly {
        Poly {
            coeffs: self.coeffs.iter()
                .map(|&c| mul_mod(c, scalar, q))
                .collect(),
            n: self.n,
        }
    }

    /// Prepare a polynomial for use as a reusable NTT operand.
    /// Applies the psi-twist and forward NTT so both steps are done once
    /// instead of once per mul() call. Use with mul_ntt_precomputed().
    pub fn ntt_prepared(&self, q: i64) -> NttPrepared {
        let n       = self.n;
        let psi     = PSI;
        let omega   = mul_mod(psi, psi, q);
        let mut a   = self.coeffs.clone();
        let mut pw  = 1i64;
        for i in 0..n {
            a[i] = mul_mod(a[i], pw, q);
            pw   = mul_mod(pw, psi, q);
        }
        ntt_forward(&mut a, q, omega);
        NttPrepared { coeffs: a, n, q }
    }

    /// Multiply self by a precomputed NTT operand, skipping the twist+NTT for that operand.
    /// self still gets its own twist+NTT applied - only the `prepared` side is reused.
    /// This is the win when the same polynomial (e.g. `u` in BFV encrypt) is multiplied
    /// against multiple targets: prepare it once, call this for each target.
    pub fn mul_ntt_precomputed(&self, prepared: &NttPrepared, q: i64) -> Poly {
        assert_eq!(self.n, prepared.n);
        assert_eq!(q, prepared.q);

        let n       = self.n;
        let psi     = PSI;
        let omega   = mul_mod(psi, psi, q);
        let psi_inv = pow_mod(psi, q - 2, q);

        // Twist + NTT only for self - `prepared` already has this done
        let mut a  = self.coeffs.clone();
        let mut pw = 1i64;
        for i in 0..n {
            a[i] = mul_mod(a[i], pw, q);
            pw   = mul_mod(pw, psi, q);
        }
        ntt_forward(&mut a, q, omega);

        // Pointwise multiply against the precomputed NTT form
        for i in 0..n {
            a[i] = mul_mod(a[i], prepared.coeffs[i], q);
        }

        ntt_inverse(&mut a, q, omega);

        // Untwist
        pw = 1i64;
        for x in a.iter_mut() {
            *x = mul_mod(*x, pw, q);
            pw = mul_mod(pw, psi_inv, q);
        }

        Poly { coeffs: a, n }
    }
}

pub struct NttPrepared {
    pub coeffs: Vec<i64>,
    pub n: usize,
    pub q: i64,
}

// Constants for NTT
pub const NTT_PRIME: i64 = 1152921504606904321;
pub const PSI: i64 = 78279720091597621;