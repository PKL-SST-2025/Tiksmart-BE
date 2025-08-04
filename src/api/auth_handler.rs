use crate::errors::AppError;
use crate::models::{auth::ForgotPasswordPayload, LoginPayload, LoginResponse, User};
use crate::service::{auth_service, user_service};
use axum::http::StatusCode;
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

/// POST /api/auth/login
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, AppError> {
    // The service layer now handles validation, so this is clean.
    let login_response = auth_service::login(&pool, payload).await?;
    Ok(Json(login_response))
}

/// GET /api/auth/me
pub async fn get_me(
    Extension(user_id): Extension<i32>, // Extract user ID from auth_guard middleware
    State(pool): State<PgPool>,
) -> Result<Json<User>, AppError> {
    let user = user_service::get_by_id(&pool, user_id).await?;
    Ok(Json(user))
}

/// POST /api/auth/forgot-password
pub async fn forgot_password(
    State(pool): State<PgPool>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<StatusCode, AppError> {
    // THE FIX: Pass the entire validated payload struct to the service, not just the email string.
    auth_service::forgot_password(&pool, payload).await?;

    // Return a 200 OK with no body. We don't want to reveal if the email exists.
    Ok(StatusCode::OK)
}