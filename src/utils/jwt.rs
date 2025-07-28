// File: be-api/src/utils/jwt.rs

use crate::config::CONFIG; 
use crate::errors::AppError;
use crate::models::TokenClaims;
use jsonwebtoken::{encode, EncodingKey, Header};

/// Creates a JSON Web Token (JWT) for a given user ID.
pub fn create_jwt(user_id: i32) -> Result<String, AppError> {
    // We get the expiration from the config now, not hardcoded.
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(CONFIG.jwt_expiration_hours))
        .expect("Failed to create expiration time")
        .timestamp();

    let claims = TokenClaims {
        sub: user_id,
        exp: expiration as usize,
    };

    // Use the secret from the config.
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
    .map_err(|_| AppError::JwtCreationError)
}