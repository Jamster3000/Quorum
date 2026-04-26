//! JWT utility functions for generating and verifying access and refresh tokens.
//! This file provides functions to create JWTs with appropriate claims and to validate them against a secret key.
//! All its values are read from the environment variables (see .env.example).

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

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expiry_minutes: i64,
    pub refresh_expiry_days: i64,
}

impl JwtConfig {
    /// Load JWT configuration from environment variables.
    /// Expects the following environment variables to be set:
    /// - `JWT_SECRET`: The secret key used for signing the tokens.
    /// - `JWT_ACCESS_MINUTES`: The expiration time for access tokens in minutes.
    /// - `JWT_REFRESH_DAYS`: The expiration time for refresh tokens in days.
    ///
    /// # Example
    /// ```
    /// use std::env;
    /// env::set_var("JWT_SECRET", " mysecretkey");
    /// env::set_var("JWT_ACCESS_MINUTES", "15");
    /// env::set_var("JWT_REFRESH_DAYS", "7");
    /// let config = JwtConfig::from_env().unwrap();
    /// assert_eq!(config.secret, "mysecretkey");
    /// assert_eq!(config.access_expiry_minutes, 15);
    /// assert_eq!(config.refresh_expiry_days, 7);
    /// ```
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let secret = std::env::var("JWT_SECRET")?;
        let access_expiry_minutes = std::env::var("JWT_ACCESS_MINUTES")?.parse()?;
        let refresh_expiry_days = std::env::var("JWT_REFRESH_DAYS")?.parse()?;

        Ok(JwtConfig {
            secret,
            access_expiry_minutes,
            refresh_expiry_days,
        })
    }
}

/// Generate an access token for a user with the given user ID and username.
/// The token will include claims such as the subject (user ID), username, expiration time, issued at time, and token type.
///
/// # Arguments
/// * `user_id` - The unique identifier of the user for whom the token is being generated.
/// * `username` - The username of the user for whom the token is being generated.
/// * `config` - The JWT configuration containing the secret key and expiration settings.
///
/// # Returns
/// A `Result` containing the generated JWT as a `String` if successful, or an error if the token generation fails.
///
/// # Example
/// ```
/// let config = JwtConfig {
///     secret: " mysecretkey".to_string(),
///     access_expiry_minutes: 15,
///     refresh_expiry_days: 7,
/// };
/// let token = generate_access_token("user123", "john_doe", &config).unwrap();
/// assert!(!token.is_empty());
/// ```
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

/// Generate a refresh token for a user with the given user ID and username.
/// The token will include claims such as the subject (user ID), username, expiration time, issued at time, and token type.
///
/// # Arguments
/// * `user_id` - The unique identifier of the user for whom the token is being generated.
/// * `username` - The username of the user for whom the token is being generated.
/// * `config` - The JWT configuration containing the secret key and expiration settings.
///
/// # Returns
/// A `Result` containing the generated JWT as a `String` if successful, or an error if the token generation fails.
///
/// # Example
/// ```
/// let config = JwtConfig {
///     secret: " mysecretkey".to_string(),
///     access_expiry_minutes: 15,
///     refresh_expiry_days: 7,
/// };
/// let token = generate_refresh_token("user123", "john_doe", &config).unwrap();
/// assert!(!token.is_empty());
/// ```
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

/// Verify a JWT token and return the claims if the token is valid.
/// The function decodes the token using the secret key from the configuration and checks its validity.
///
/// # Arguments
/// * `token` - The JWT token to be verified.
/// * `config` - The JWT configuration containing the secret key for decoding the token.
///
/// # Returns
/// A `Result` containing the `Claims` if the token is valid, or an error if the token is invalid or expired.
///
/// # Example
/// ```
/// let config = JwtConfig {
///     secret: " mysecretkey".to_string(),
///     access_expiry_minutes: 15,
///     refresh_expiry_days: 7,
/// };
/// let token = generate_access_token("user123", "john_doe", &config).unwrap();
/// let claims = verify_token(&token, &config).unwrap();
/// assert_eq!(claims.sub, "user123");
/// assert_eq!(claims.username, "john_doe");
/// assert_eq!(claims.token_type, "access");
/// ```
pub fn verify_token(token: &str, config: &JwtConfig) -> Result<Claims, Box<dyn Error>> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
