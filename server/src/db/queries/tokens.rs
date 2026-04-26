use crate::db::DB;
use sha2::{Digest, Sha256};
use std::error::Error;

pub async fn store_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
    expires_at: i64,
) -> Result<(), Box<dyn Error>> {
    let token_hash = hash_token(refresh_token);

    let query = "CREATE refresh_token SET user_id = $user_id, token_hash = $token_hash, expires_at = $expires_at";

    db.query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .bind(("token_hash", token_hash))
        .bind(("expires_at", expires_at))
        .await?;

    Ok(())
}

pub async fn validate_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
) -> Result<bool, Box<dyn Error>> {
    let token_hash = hash_token(refresh_token);
    let query = "SELECT id FROM refresh_token WHERE user_id = $user_id AND token_hash = $token_hash AND is_revoked = false LIMIT 1";

    let mut response = db
        .query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .bind(("token_hash", token_hash))
        .await?;

    let results: Vec<serde_json::Value> = response.take(0)?;
    Ok(!results.is_empty())
}

pub async fn revoke_refresh_token(db: &DB, refresh_token: &str) -> Result<(), Box<dyn Error>> {
    let token_hash = hash_token(refresh_token);
    let query = "UPDATE refresh_token SET is_revoked = true WHERE token_hash = $token_hash";

    db.query(query).bind(("token_hash", token_hash)).await?;

    Ok(())
}

pub async fn delete_all_user_tokens(db: &DB, user_id: &str) -> Result<(), Box<dyn Error>> {
    let query = "DELETE FROM refresh_token WHERE user_id = $user_id";

    db.query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .await?;

    Ok(())
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
