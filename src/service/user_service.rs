// File: src/service/user_service.rs

use crate::db::user_query;
use crate::errors::AppError;
use crate::models::user::ResetPasswordPayload;
use crate::models::{CreateUserPayload, User};
use crate::utils::validation; // Import the validation module
use itertools::Itertools;
use sqlx::PgPool;

/// A simple pass-through service function to get a user by ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<User, AppError> {
    user_query::get_by_id(pool, id).await
}

/// The service function for creating a user.
/// It now uses the centralized validation logic.
pub async fn create(pool: &PgPool, payload: CreateUserPayload) -> Result<User, AppError> {
    // --- Validation Logic ---
    // A single, clean call to validate the entire payload based on the
    // rules we defined in the `CreateUserPayload` struct.
    validation::validate_payload(&payload)?;

    // --- Core Logic ---
    let hashed_password =
        bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).map_err(AppError::from)?;

    // --- Database Interaction ---
    let user = user_query::create(pool, payload.email, hashed_password).await?;

    Ok(user)
}


/// The service function for creating multiple users in bulk.
/// It also benefits from the streamlined validation.
pub async fn create_bulk(
    pool: &PgPool,
    payloads: Vec<CreateUserPayload>,
) -> Result<usize, AppError> {
    const CHUNK_SIZE: usize = 100;
    let mut total_created = 0;

    for chunk in &payloads.iter().chunks(CHUNK_SIZE) {
        let mut emails = Vec::with_capacity(CHUNK_SIZE);
        let mut password_hashes = Vec::with_capacity(CHUNK_SIZE);

        for payload in chunk {
            // --- Validation Logic ---
            // Validate each payload in the chunk. If any fails, the entire
            // operation is aborted before hitting the database.
            validation::validate_payload(payload)?;

            // --- Core Logic ---
            let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;
            emails.push(payload.email.clone());
            password_hashes.push(hashed_password);
        }

        if !emails.is_empty() {
            let created_in_chunk = user_query::create_bulk(pool, emails, password_hashes).await?;
            total_created += created_in_chunk;
        }
    }

    Ok(total_created)
}


// Service function for the password reset logic.
pub async fn reset_password(pool: &PgPool, payload: ResetPasswordPayload) -> Result<(), AppError> {
    // 1. Validate the payload (e.g., strong password, token format).
    validation::validate_payload(&payload)?;

    // 2. Find the user by the token. Returns 404 if token is invalid.
    let user = user_query::get_user_by_reset_token(pool, &payload.token).await?;

    // 3. Check if the token has expired.
    if let Some(expires_at) = user.password_reset_expires_at {
        if chrono::Utc::now() > expires_at {
            return Err(AppError::BadRequest("Password reset token has expired.".to_string()));
        }
    } else {
        return Err(AppError::BadRequest("Invalid password reset token.".to_string()));
    }

    // 4. Hash the new password.
    let new_hashed_password = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST)?;

    // 5. Update the user's password and clear the token in the database.
    user_query::update_password_and_clear_token(pool, user.id, &new_hashed_password).await?;

    Ok(())
}