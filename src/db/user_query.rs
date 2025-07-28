// File: be-api/src/db/user_query.rs

use crate::errors::AppError;
use crate::models::User;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use itertools::Itertools; 

// A private struct used only for authentication.
// It includes the password hash, which should never be sent to the client.
pub struct UserAuthData {
    pub id: i32,
    pub password_hash: String,
}

/// Fetches a single user from the database by their ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<User, AppError> { // <-- Add `pub`
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, created_at FROM users WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Creates a new user in the database and returns the created user record.
pub async fn create(pool: &PgPool, email: String, password_hash: String) -> Result<User, AppError> { // <-- Add `pub`
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, created_at",
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Fetches essential authentication data for a user by their email.
pub async fn get_auth_data_by_email( // <-- Add `pub`
    pool: &PgPool,
    email: &str,
) -> Result<UserAuthData, AppError> {
    let user_auth_data = sqlx::query_as!(
        UserAuthData,
        "SELECT id, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user_auth_data)
}


/// Sets a password reset token and its expiration for a user identified by their email.
pub async fn set_password_reset_token(
    pool: &PgPool,
    email: &str,
    token: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE users SET password_reset_token = $1, password_reset_expires_at = $2 WHERE email = $3",
        token,
        expires_at,
        email
    )
    .execute(pool)
    .await?;

    Ok(())
}


/// Inserts a batch of new users into the database in a single, efficient query.
///
/// This function uses PostgreSQL's `UNNEST` feature to handle bulk data insertion,
/// which is significantly faster than inserting rows one by one.
///
/// # Arguments
///
/// * `pool` - The database connection pool.
/// * `emails` - A vector of email strings for the new users.
/// * `password_hashes` - A vector of pre-hashed passwords for the new users.
///
/// # Returns
///
/// * A `Result` containing the number of users successfully created (`usize`).
pub async fn create_bulk(
    pool: &PgPool,
    emails: Vec<String>,
    password_hashes: Vec<String>,
) -> Result<usize, AppError> {
    // The query takes two arrays as input: one for emails ($1) and one for hashes ($2).
    // UNNEST turns these arrays into a temporary two-column table.
    // We then insert all rows from this temporary table into the `users` table.
    let rows_affected = sqlx::query!(
        "INSERT INTO users (email, password_hash) SELECT * FROM UNNEST($1::varchar[], $2::varchar[])",
        &emails,
        &password_hashes
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected as usize)
}