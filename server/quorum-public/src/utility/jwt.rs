//! JWT token generation and verification.
//!
//! This module provides functions to create access and refresh tokens with appropriate claims,
//! and to validate tokens against the configured JWT secret. All configuration values are loaded
//! from environment variables and cached in the `Config` singleton.

use quorum_core::utility::config::Config;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

/// Generates a new access token for a user.
///
/// Creates a JWT access token with the user's ID and username. The token is signed using
/// the configured `JWT_SECRET` and expires after `JWT_ACCESS_MINUTES` minutes.
/// Access tokens are short-lived and used for API request authentication.
///
/// # Arguments
/// * `user_id` - The unique identifier of the user.
/// * `username` - The username of the user.
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token string.
/// * `Err(Box<dyn Error>)` - If token encoding fails or config is not initialized.
///
/// # Errors
/// Returns an error if:
/// - JWT encoding fails (invalid secret or configuration)
/// - The configured JWT secret is missing or invalid
/// - Config has not been initialized via `Config::load()`
///
/// # Example
/// ```rust,no_run
/// use crate::utility::jwt;
///
/// let token = jwt::generate_access_token("user123", "john_doe")
///     .expect("Failed to generate token");
/// println!("Access token: {}", token);
/// ```
pub fn generate_access_token(user_id: &str, username: &str) -> Result<String, Box<dyn Error>> {
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

/// Generates a new refresh token for a user.
///
/// Creates a JWT refresh token with the user's ID and username. The token is signed using
/// the configured `JWT_SECRET` and expires after `JWT_REFRESH_DAYS` days.
/// Refresh tokens are long-lived and used to obtain new access tokens without re-authentication.
///
/// # Arguments
/// * `user_id` - The unique identifier of the user.
/// * `username` - The username of the user.
///
/// # Returns
/// * `Ok(String)` - The encoded JWT token string.
/// * `Err(Box<dyn Error>)` - If token encoding fails or config is not initialized.
///
/// # Errors
/// Returns an error if:
/// - JWT encoding fails (invalid secret or configuration)
/// - The configured JWT secret is missing or invalid
/// - Config has not been initialized via `Config::load()`
///
/// # Example
/// ```rust,no_run
/// use crate::utility::jwt;
///
/// let token = jwt::generate_refresh_token("user123", "john_doe")
///     .expect("Failed to generate token");
/// println!("Refresh token: {}", token);
/// ```
pub fn generate_refresh_token(user_id: &str, username: &str) -> Result<String, Box<dyn Error>> {
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

/// Verifies and decodes a JWT token.
///
/// Validates the token signature against the configured `JWT_SECRET` and checks expiration.
/// Returns the decoded claims if the token is valid, allowing the caller to access the user ID
/// and other claim data without re-parsing.
///
/// # Arguments
/// * `token` - The encoded JWT token string to verify.
///
/// # Returns
/// * `Ok(Claims)` - The decoded claims if the token is valid.
/// * `Err(Box<dyn Error>)` - If token verification fails.
///
/// # Errors
/// Returns an error if:
/// - The token is malformed or cannot be decoded
/// - The signature does not match the configured secret
/// - The token has expired
/// - Required claims are missing
/// - Config has not been initialized via `Config::load()`
///
/// # Example
/// ```rust,no_run
/// use crate::utility::jwt;
///
/// let token = "eyJ0eXAiOiJKV1QiLCJhbGc..."; // Valid JWT
/// match jwt::verify_token(token) {
///     Ok(claims) => println!("Authenticated user: {}", claims.sub),
///     Err(_) => println!("Invalid or expired token"),
/// }
/// ```
pub fn verify_token(token: &str) -> Result<Claims, Box<dyn Error + Send + Sync>> {
    let config = Config::get();
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
