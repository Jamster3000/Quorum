use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

static HASH_SEMAPHORE: std::sync::LazyLock<Arc<Semaphore>> =
    std::sync::LazyLock::new(|| Arc::new(Semaphore::new(8)));

fn get_argon2() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            19456,
            2,
            1,
            None,
        )
        .unwrap(),
    )
}

fn hash_password_sync(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(get_argon2()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

fn verify_password_sync(
    password: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(get_argon2()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password.to_owned();
    let _permit = HASH_SEMAPHORE.acquire().await.unwrap();
    task::spawn_blocking(move || hash_password_sync(&password))
        .await
        .map_err(|_| argon2::password_hash::Error::Password)?
}

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