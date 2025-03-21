use crate::models::Claims;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| anyhow!("Failed to hash password: {}", e))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(|e| anyhow!("Failed to verify password: {}", e))
}

/// Generate a JWT token for a user
pub fn generate_token(user_id: Uuid, secret: &str, expiration_seconds: u64) -> Result<String> {
    let now = Utc::now();
    let exp = (now + Duration::seconds(expiration_seconds as i64))
        .timestamp() as usize;
    let iat = now.timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow!("Failed to generate token: {}", e))
}

/// Validate a JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims> {
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|e| anyhow!("Failed to validate token: {}", e))?;
    
    Ok(token_data.claims)
}
