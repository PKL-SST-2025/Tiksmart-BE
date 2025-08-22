// File: src/db/user_query.rs

use crate::{errors::AppError, models::user::User};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres};


// A private struct used only for authentication.
pub struct UserAuthData {
    pub id: i32,
    pub password_hash: String,
}

/// This is more efficient than fetching the entire User object.
pub struct UserByResetToken {
    pub id: i32,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
}




/// Fetches a single user from the database by their ID.
pub async fn get_by_id<'e, E>(executor: E, id: i32) -> Result<User, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    let user = sqlx::query_as!(
        User,
        // The problem is likely in THIS SELECT statement's columns
        "SELECT id, email, username, password_hash, created_at, password_reset_token, password_reset_expires_at FROM users WHERE id = $1",
        id
    )
    .fetch_one(executor)
    .await?; // If fetch_one finds no row, it returns RowNotFound
    Ok(user)
}


/// Creates a new user in the database and returns the created user record.
// This function should already be generic over Executor if it's called within a transaction elsewhere.
// If not, it can remain `&PgPool`. But it's good practice for atomic operations.
// For now, let's assume it *might* be called in transaction, so make it generic.
pub async fn create<'e, E>(executor: E, email: String, username: String, password_hash: String) -> Result<User, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3)
         RETURNING id, email, username, password_hash, created_at, password_reset_token, password_reset_expires_at",
        email,
        username,
        password_hash
    )
    .fetch_one(executor) // Use the generic executor
    .await?;

    Ok(user)
}


/// Fetches essential authentication data for a user by their email.
pub async fn get_auth_data_by_email(
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


/// Inserts a batch of new users into the database in a single, efficient query.
pub async fn create_bulk(
    pool: &PgPool,
    emails: Vec<String>,
    password_hashes: Vec<String>,
) -> Result<usize, AppError> {
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

/// This will return a `RowNotFound` error if the token is invalid, which is handled
/// by our `AppError` enum to become a 404 Not Found response.
pub async fn get_user_by_reset_token(
    pool: &PgPool,
    token: &str,
) -> Result<UserByResetToken, AppError> {
    sqlx::query_as!(
        UserByResetToken,
        "SELECT id, password_reset_expires_at FROM users WHERE password_reset_token = $1",
        token
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// It is critical to nullify the token fields to prevent the same token from being reused.
pub async fn update_password_and_clear_token(
    pool: &PgPool,
    user_id: i32,
    new_hash: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE users
         SET password_hash = $1, password_reset_token = NULL, password_reset_expires_at = NULL
         WHERE id = $2",
        new_hash,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}




/// Sets a password reset OTP code and its expiration for a user identified by their email.
pub async fn set_password_reset_token(
    pool: &PgPool,
    email: &str,
    token: &str, // The 6-digit code
    expires_at: DateTime<Utc>,
) -> Result<(), AppError> {
    let result = sqlx::query!(
        "UPDATE users SET password_reset_token = $1, password_reset_expires_at = $2 WHERE email = $3",
        token,
        expires_at,
        email
    )
    .execute(pool)
    .await?;

    // Check if any row was actually updated.
    // This is a security measure to prevent leaking whether an email exists in the system.
    // The request will succeed from the user's perspective, but no email will be sent.
    if result.rows_affected() == 0 {
        tracing::warn!("Password reset requested for non-existent email: {}", email);
        // We return RowNotFound, which our error handler will treat as a 404, but
        // the service layer will handle this gracefully.
        return Err(AppError::Sqlx(sqlx::Error::RowNotFound));
    }

    Ok(())
}

/// A lightweight struct to fetch only the data needed for OTP verification.
pub struct UserForOtpVerification {
    pub id: i32,
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
}

/// Fetches a user's ID and reset token info by their email.
pub async fn get_user_for_otp_verification(
    pool: &PgPool,
    email: &str,
) -> Result<UserForOtpVerification, AppError> {
    sqlx::query_as!(
        UserForOtpVerification,
        "SELECT id, password_reset_token, password_reset_expires_at FROM users WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Updates a user's password hash and, crucially, clears the OTP fields to prevent reuse.
pub async fn update_password_and_clear_otp(
    pool: &PgPool,
    user_id: i32,
    new_hash: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE users
         SET password_hash = $1, password_reset_token = NULL, password_reset_expires_at = NULL
         WHERE id = $2",
        new_hash,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}