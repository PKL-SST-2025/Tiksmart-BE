// File: be-api/src/service/user_service.rs

use crate::db::user_query;
use crate::errors::AppError;
use crate::models::{CreateUserPayload, User};
use crate::utils;
use itertools::Itertools; 
use sqlx::PgPool;

/// A simple pass-through service function to get a user by ID.
/// Some service functions might just orchestrate a single DB call.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<User, AppError> {
    user_query::get_by_id(pool, id).await
}

/// The service function for creating a user.
/// This contains the core business logic.
pub async fn create(pool: &PgPool, payload: CreateUserPayload) -> Result<User, AppError> {
    // Email validation 
    if !utils::validate_email(&payload.email) {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    if payload.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    let hashed_password =
        bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).map_err(AppError::from)?;

    let user = user_query::create(pool, payload.email, hashed_password).await?;

    Ok(user)
}


/// The service function for creating multiple users in bulk.
///
/// This function processes users in batches to ensure high performance and
/// avoid overwhelming the database with too many individual requests.
///
/// # Arguments
///
/// * `pool` - The database connection pool.
/// * `payloads` - A vector of `CreateUserPayload` structs.
///
/// # Returns
///
/// * A `Result` containing the total number of users created (`usize`).
pub async fn create_bulk(
    pool: &PgPool,
    payloads: Vec<CreateUserPayload>,
) -> Result<usize, AppError> {
    const CHUNK_SIZE: usize = 100; // Process 100 users at a time.
    let mut total_created = 0;

    // Here is the magic! `.chunks()` comes from `itertools`.
    // It breaks our large vector of payloads into an iterator of smaller slices.
    for chunk in &payloads.iter().chunks(CHUNK_SIZE) {
        // We need to transform this chunk of payloads into two separate vectors
        // for our database function: one for emails and one for hashes.
        let mut emails = Vec::with_capacity(CHUNK_SIZE);
        let mut password_hashes = Vec::with_capacity(CHUNK_SIZE);

        for payload in chunk {
            // Perform validation for each user in the chunk.
            if !utils::validate_email(&payload.email) {
                return Err(AppError::BadRequest(format!(
                    "Invalid email format for: {}",
                    payload.email
                )));
            }
            if payload.password.len() < 8 {
                return Err(AppError::BadRequest(format!(
                    "Password for {} is too short",
                    payload.email
                )));
            }

            // Hash the password and collect the data.
            let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;
            emails.push(payload.email.clone());
            password_hashes.push(hashed_password);
        }

        // If we have data to insert, call our bulk DB function.
        if !emails.is_empty() {
            let created_in_chunk = user_query::create_bulk(pool, emails, password_hashes).await?;
            total_created += created_in_chunk;
        }
    }

    Ok(total_created)
}