// File: src/utils/jwt.rs

use crate::config::CONFIG;
use crate::errors::AppError;
use crate::models::TokenClaims;
use jsonwebtoken::{encode, EncodingKey, Header};

/// Creates a JSON Web Token (JWT) for a given principal ID and role.
// We've added the `role` parameter here.
pub fn create_jwt(principal_id: i32, role: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(CONFIG.jwt_expiration_hours))
        .expect("Failed to create expiration time")
        .timestamp();

    // Now we correctly provide the `role` field when creating the claims.
    let claims = TokenClaims {
        sub: principal_id.to_string(), // Convert the ID to a string for the 'sub' claim
        role: role.to_string(),        // Add the role
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
    .map_err(|_| AppError::JwtCreationError)
}