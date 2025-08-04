use crate::db::user_query;
use crate::errors::AppError;
use crate::models::{ForgotPasswordPayload, LoginPayload, LoginResponse};
use crate::utils;
use sqlx::PgPool;
use crate::utils::validation; 

/// The main login service function.
/// It orchestrates fetching user data, verifying the password, and generating a JWT.
pub async fn login(pool: &PgPool, payload: LoginPayload) -> Result<LoginResponse, AppError> {
    // ADDED: Validate the incoming payload.
    validation::validate_payload(&payload)?;

    let user_auth_data = user_query::get_auth_data_by_email(pool, &payload.email).await?;
    let is_valid = bcrypt::verify(&payload.password, &user_auth_data.password_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = crate::utils::jwt::create_jwt(user_auth_data.id)?;
    Ok(LoginResponse { token })
}

/// Handles the business logic for a password reset request.
pub async fn forgot_password(
    pool: &PgPool,
    payload: ForgotPasswordPayload,
) -> Result<(), AppError> {
    // ADDED: Validate the incoming payload.
    validation::validate_payload(&payload)?;

    let reset_token = crate::utils::random::generate_random_token(32);
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(15);

    user_query::set_password_reset_token(pool, &payload.email, &reset_token, expires_at).await?;

    tracing::info!(
        "Password reset token generated for {}. Token: {}",
        payload.email,
        reset_token
    );

    Ok(())
}