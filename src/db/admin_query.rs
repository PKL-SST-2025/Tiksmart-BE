// File: src/db/admin_query.rs

use crate::errors::AppError;
use sqlx::PgPool;

// A lightweight struct to fetch only the necessary data for admin authentication.
pub struct AdminAuthData {
    pub id: i32,
    pub password_hash: String,
}

/// Fetches essential authentication data for an admin by their email.
pub async fn get_auth_data_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<AdminAuthData, AppError> {
    sqlx::query_as!(
        AdminAuthData,
        "SELECT id, password_hash FROM admins WHERE email = $1 AND is_active = true",
        email
    )
    .fetch_one(pool)
    .await
    // Map RowNotFound to InvalidCredentials for security (don't reveal if email exists)
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InvalidCredentials,
        _ => AppError::Sqlx(e),
    })
}