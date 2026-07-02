//! Functions for managing refresh tokens in the database.
//! This includes storing, validating, revoking, and deleting refresh tokens.

use quorum_core::db::DB;
use sha2::{Digest, Sha256};
use std::error::Error;

/// Stores a new refresh token for a user in the database.
/// The token is hashed before storage for security purpsoes.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `user_id` - The ID of the user the token belongs to.
/// * `refresh_token` - The refresh token to be stored.
/// * `expires_at` - The expiration time of the refresh token (Unix timestamp).
///
/// # Returns
/// * `Ok(())` if the token was successfully stored.
/// * `Err` if there was an error during the database operation.
///
/// # Exmaple
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::tokens::store_refresh_token;
/// let db = DB::new("localhost:8529", "my_database").await.unwrap();
/// store_refresh_token(&db, "user123", "some_refresh_token", 1700000000).await.unwrap();
/// ```
pub async fn store_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
    expires_at: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let token_hash = hash_token(refresh_token);

    let query = "CREATE refresh_token
         SET user_id = $user_id,
             token_hash = $token_hash,
             expires_at = $expires_at,
             is_revoked = false";

    db.query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .bind(("token_hash", token_hash))
        .bind(("expires_at", expires_at))
        .await?;

    Ok(())
}

/// Revokes a refresh token by setting its `is_revoked` flag to true in the database.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `refresh_token` - The refresh token to be revoked.
///
/// # Returns
/// * `Ok(())` if the token was successfully revoked.
/// * `Err` if there was an error during the database operation.
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::tokens::revoke_refresh_token;
/// let db = DB::new("localhost:8529", "my_database").await.unwrap();
/// revoke_refresh_token(&db, "some_refresh_token").await.unwrap();
/// ```
pub async fn revoke_refresh_token(
    db: &DB,
    refresh_token: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let token_hash = hash_token(refresh_token);
    let query = "UPDATE refresh_token SET is_revoked = true WHERE token_hash = $token_hash";

    db.query(query).bind(("token_hash", token_hash)).await?;

    Ok(())
}

/// Deletes all refresh tokens associated with a user from the database.
///
/// # Arguments
/// * `db` - A reference to the database connection.
/// * `user_id` - The ID of the user whose tokens should be deleted.
///
/// # Returns
/// * `Ok(())` if the tokens were successfully deleted.
/// * `Err` if there was an error during the database operation.
///
/// # Example
/// ```rust
/// use crate::db::DB;
/// use crate::db::queries::tokens::delete_all_user_tokens;
/// let db = DB::new("localhost:8529", "my_database").await.unwrap();
/// delete_all_user_tokens(&db, "user123").await.unwrap();
/// ```
pub async fn delete_all_user_tokens(
    db: &DB,
    user_id: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = "DELETE FROM refresh_token WHERE user_id = $user_id";

    db.query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .await?;

    Ok(())
}

/// Hashes a refresh token using SHA-256 for secure storage in the database.
///
/// # Arguments
/// * `token` - The refresh token to be hashed.
///
/// # Returns
/// * A `String` representing the SHA-256 hash of the token.
///
/// # Example
/// ```rust
/// let token = "some_refresh_token";
/// let hashed_token = hash_token(token);
/// ```
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn validate_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let token_hash = hash_token(refresh_token);

    let query = "SELECT VALUE count() > 0 FROM refresh_token
         WHERE user_id = $user_id
         AND token_hash = $token_hash
         AND is_revoked = false LIMIT 1";

    let mut response = db
        .query(query)
        .bind(("user_id", format!("users:{}", user_id)))
        .bind(("token_hash", token_hash))
        .await?;

    let is_valid = response.take::<Option<bool>>(0)?.unwrap_or(false);

    if is_valid {
        Ok(())
    } else {
        Err("Refresh token not found or revoked".into())
    }
}
