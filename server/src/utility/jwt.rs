use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,
	pub username: String,
	pub exp: i64,
	pub iat: i64,
	pub token_type: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
	pub secret: String,
	pub access_expiry_minutes: i64,
	pub refresh_expiry_days: i64,
}

impl JwtConfig {
	pub fn from_env() -> Result<Self, Box<dyn Error>> {
		let secret = std::env::var("JWT_SECRET")?;
		let access_expiry_minutes = std::env::var("JWT_ACCESS_MINUTES")?.parse()?;
		let refresh_expiry_days = std::env::var("JWT_REFRESH_DAYS")?.parse()?;

		Ok(JwtConfig {
			secret, access_expiry_minutes,
			refresh_expiry_days,
		})
	}
}

pub fn generate_access_token(
	user_id: &str,
	username: &str,
	config: &JwtConfig,
) -> Result<String, Box<dyn Error>> {
	let now = Utc::now();
	let expiry = now + Duration::minutes(config.access_expiry_minutes);

	let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiry.timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )?;

	Ok(token)
}

pub fn generate_refresh_token(
	user_id: &str,
	username: &str,
	config: &JwtConfig,
) -> Result<String, Box<dyn Error>> {
	let now = Utc::now();
	let expiry = now + Duration::days(config.refresh_expiry_days);

	let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiry.timestamp(),
        iat: now.timestamp(),
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )?;

	Ok(token)
}

pub fn verify_token(token: &str, config: &JwtConfig) -> Result<Claims, Box<dyn Error>> {
	let data = decode::<Claims>(
		token,
		&DecodingKey::from_secret(config.secret.as_ref()),
		&Validation::default(),
	)?;

	Ok(data.claims)
}