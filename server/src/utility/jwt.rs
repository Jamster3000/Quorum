//! JWT utility functions for generating and verifying access and refresh tokens.
//! This file provides functions to create JWTs with appropriate claims and to validate them against a secret key.
//! All its values are read from the environment variables (see .env.example).

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::utility::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

pub fn generate_access_token(
    user_id: &str,
    username: &str,
) -> Result<String, Box<dyn Error>> {
    let config = Config::get();
    let now = Utc::now();
    let expiry = now + Duration::minutes(config.jwt_access_minutes);

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
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn generate_refresh_token(
    user_id: &str,
    username: &str,
) -> Result<String, Box<dyn Error>> {
    let config = Config::get();
    let now = Utc::now();
    let expiry = now + Duration::days(config.jwt_refresh_days);

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
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims, Box<dyn Error>> {
    let config = Config::get();
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
