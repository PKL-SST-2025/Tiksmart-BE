// File: be-api/src/service/auth_service.rs

use crate::utils;
use crate::db::user_query;
use crate::errors::AppError;
use crate::models::{LoginPayload, LoginResponse};
use sqlx::PgPool;

/// The main login service function.
pub async fn login(pool: &PgPool, payload: LoginPayload) -> Result<LoginResponse, AppError> {
    // 1. Fetch user authentication data from the database.
    let user_auth_data = user_query::get_auth_data_by_email(pool, &payload.email).await?;

    // 2. Verify the provided password against the stored hash.
    let is_valid = bcrypt::verify(&payload.password, &user_auth_data.password_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // 3. If the password is valid, call our new utility function to create a JWT.
    let token = utils::create_jwt(user_auth_data.id)?; // <-- The clean new way

    // 4. Return the token in the response.
    Ok(LoginResponse { token })
}


pub async fn forgot_password(pool: &PgPool, email: String) -> Result<(), AppError> {
    // Generate a secure random token using our new utility.
    let reset_token = utils::generate_random_token(32);

    // Set an expiration time for the token (e.g., 15 minutes from now).
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);

    // Call the database layer to store the token and expiration.
    user_query::set_password_reset_token(pool, &email, &reset_token, expires_at).await?;

    // In a real app, you would now send an email to the user.
    tracing::info!(
        "Password reset token generated for {}. Token: {}",
        email,
        reset_token
    );

    Ok(())
}