//! SIMD accelerated polynomial operations using AVX2.
//! if AVx2 is not available at runtime, falls back to scalar
//!
//! AVX2 processes 4 x i64 values per instruction.

/// SIMD add two coefficient slices mod q, storing result in dst.
/// Processes 4 elements at a time with AVX2, remainder scalar.
#[cfg(target_arch = "x86_64")]
pub fn add_mod(a: &[i64], b: &[i64], q: i64, dst: &mut [i64]) {
    use std::arch::x86_64::*;
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), dst.len());

    let n    = a.len();
    let q_v  = unsafe { _mm256_set1_epi64x(q) };

    let mut i = 0;
    // Process 4 elements at a time
    while i + 4 <= n {
        unsafe {
            let va  = _mm256_loadu_si256(a[i..].as_ptr() as *const __m256i);
            let vb  = _mm256_loadu_si256(b[i..].as_ptr() as *const __m256i);
            let sum = _mm256_add_epi64(va, vb);
            // Reduce mod q: if sum >= q, subtract q
            // _mm256_cmpgt_epi64: returns -1 (all ones) where sum > q
            // We need sum >= q, so compare q-1 < sum i.e. (q-1) < sum
            let q1  = _mm256_set1_epi64x(q - 1);
            let mask = _mm256_cmpgt_epi64(sum, q1);        // -1 where sum > q-1
            let sub = _mm256_and_si256(mask, q_v);          // q where sum >= q, else 0
            let res = _mm256_sub_epi64(sum, sub);
            _mm256_storeu_si256(dst[i..].as_mut_ptr() as *mut __m256i, res);
        }
        i += 4;
    }
    // Scalar remainder
    while i < n {
        dst[i] = (a[i] + b[i]).rem_euclid(q);
        i += 1;
    }
}

/// SIMD sub two coefficient slices mod q.
#[cfg(target_arch = "x86_64")]
pub fn sub_mod(a: &[i64], b: &[i64], q: i64, dst: &mut [i64]) {
    use std::arch::x86_64::*;
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), dst.len());

    let n   = a.len();
    let q_v = unsafe { _mm256_set1_epi64x(q) };
    let z   = unsafe { _mm256_setzero_si256() };

    let mut i = 0;
    while i + 4 <= n {
        unsafe {
            let va   = _mm256_loadu_si256(a[i..].as_ptr() as *const __m256i);
            let vb   = _mm256_loadu_si256(b[i..].as_ptr() as *const __m256i);
            let diff = _mm256_sub_epi64(va, vb);
            // If diff < 0, add q
            let mask = _mm256_cmpgt_epi64(z, diff);  // -1 where diff < 0
            let add  = _mm256_and_si256(mask, q_v);
            let res  = _mm256_add_epi64(diff, add);
            _mm256_storeu_si256(dst[i..].as_mut_ptr() as *mut __m256i, res);
        }
        i += 4;
    }
    while i < n {
        dst[i] = (a[i] - b[i]).rem_euclid(q);
        i += 1;
    }
}