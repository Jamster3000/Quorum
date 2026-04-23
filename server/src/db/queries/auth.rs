use crate::db::DB;
use crate::models::user::User;
use std::error::Error;

pub async fn signup_user(
    db: &DB,
    username: &str,
    email: Option<&str>,
    password_hash: &str,
) -> Result<User, Box<dyn Error>> {
    let mut response = db
        .query(
            "CREATE users SET username = $username, email = $email, password_hash = $password_hash",
        )
        .bind(("username", username.to_string()))
        .bind(("email", email.map(|e| e.to_string())))
        .bind(("password_hash", password_hash.to_string()))
        .await?;

    let user: Vec<User> = response.take(0)?;
    user.into_iter()
        .next()
        .ok_or("Failed to create user".into())
}

pub async fn get_user_by_username_or_email(
    db: &DB,
    username_or_email: &str,
) -> Result<User, Box<dyn Error>> {
    let mut response = db
		.query("SELECT id, username, email, password_hash, created_at, last_login, is_banned, is_deleted FROM users WHERE username = $value OR email = $value LIMIT 1")
		.bind(("value", username_or_email.to_string()))
		.await?;

    let user: Vec<User> = response.take(0)?;
    user.into_iter().next().ok_or("User not found".into())
}

pub async fn delete_user_by_id(db: &DB, user_id: &str) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
    tokens::delete_all_user_tokens(db, user_id).await?;

    let query = format!("DELETE FROM users:{}", user_id);
    db.query(&query).await?;
    Ok(())
}

pub async fn store_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
    expires_at: i64,
) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
    tokens::store_refresh_token(db, user_id, refresh_token, expires_at).await
}

pub async fn validate_refresh_token(
    db: &DB,
    user_id: &str,
    refresh_token: &str,
) -> Result<bool, Box<dyn Error>> {
    use crate::db::queries::tokens;
    tokens::validate_refresh_token(db, user_id, refresh_token).await
}

pub async fn revoke_refresh_token(db: &DB, refresh_token: &str) -> Result<(), Box<dyn Error>> {
    use crate::db::queries::tokens;
    tokens::revoke_refresh_token(db, refresh_token).await
}
