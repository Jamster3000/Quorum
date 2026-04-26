//! Password hashing and verification using Argon2id.
//! Uses a semaphore to limit concurrent hashing operations to 8, preventing CPU overload.

use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

static HASH_SEMAPHORE: std::sync::LazyLock<Arc<Semaphore>> =
    std::sync::LazyLock::new(|| Arc::new(Semaphore::new(8)));

/// Returns a configured Argon2 instance with specific parameters for hashing.
/// The parameters used are:
/// - Memory: 19456 KB
/// - Iterations: 2
/// - Parallelism: 1
///
/// These parameters are chosen to balance security and performance, making it suitable for password hashing.
///
/// # Returns
/// An `Argon2` instance configured with the specified parameters.
fn get_argon2() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(19456, 2, 1, None).unwrap(),
    )
}

/// Synchronously hashes a password using Argon2id with a randomly generated salt.
/// The function generates a unique salt for each password and returns the resulting hash as a string.
///
/// # Arguments
/// * `password` - The plaintext password to be hashed.
///
/// # Returns
/// A `Result` containing the hashed password as a string on success, or an `argon2::password_hash::Error` on failure.
fn hash_password_sync(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(get_argon2()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

/// Synchronously verifies a plaintext password against a given hash using Argon2id.
/// The function parses the provided hash and checks if the password matches it.
///
/// # Arguments
/// * `password` - The plaintext password to be verified.
/// * `hash` - The hashed password to compare against.
///
/// # Returns
/// A `Result` containing `true` if the password matches the hash, `false` if it does not, or an `argon2::password_hash::Error` on failure.
fn verify_password_sync(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(get_argon2()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Asynchronously hashes a password using a blocking task to avoid blocking the async runtime.
/// The function acquires a permit from the semaphore to limit concurrent hashing operations, ensuring that no more than 8 hashing tasks run simultaneously.
///
/// # Arguments
/// * `password` - The plaintext password to be hashed.
///
/// # Returns
/// A `Result` containing the hashed password as a string on success, or an `argon2::password_hash::Error` on failure.
///
/// # Example
/// ```
/// let hashed = hash_password("my_secure_password").await.unwrap();
/// ```
pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password.to_owned();
    let _permit = HASH_SEMAPHORE.acquire().await.unwrap();
    task::spawn_blocking(move || hash_password_sync(&password))
        .await
        .map_err(|_| argon2::password_hash::Error::Password)?
}

/// Asynchronously verifies a plaintext password against a given hash using a blocking task to avoid blocking the async runtime.
/// The function acquires a permit from the semaphore to limit concurrent verification operations, ensuring that no more than 8 verification tasks run simultaneously.
///
/// # Arguments
/// * `password` - The plaintext password to be verified.
/// * `hash` - The hashed password to compare against.
///
/// # Returns
/// A `Result` containing `true` if the password matches the hash, `false` if it does not, or an `argon2::password_hash::Error` on failure.
///
/// # Example
/// ```
/// let is_valid = verify_password("my_secure_password", &hashed).await.unwrap();
/// ```
pub async fn verify_password(
    password: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let password = password.to_owned();
    let hash = hash.to_owned();
    let _permit = HASH_SEMAPHORE.acquire().await.unwrap();
    task::spawn_blocking(move || verify_password_sync(&password, &hash))
        .await
        .map_err(|_| argon2::password_hash::Error::Password)?
}

/// Performs a password hashing warmup.
/// This function spawns multiple blocking tasks to hash a dummy password, allowing the Argon2 algorithm to warm up 
/// and have minor improvments on performance for subsequent hashing operations.
///
/// # Example
/// ```
/// warmup().await;
/// ```
pub async fn warmup() {
    let handles: Vec<_> = (0..16)
        .map(|_| {
            tokio::task::spawn_blocking(|| {
                let _ = hash_password_sync("warmup");
            })
        })
        .collect();
    for h in handles {
        let _ = h.await;
    }
}
