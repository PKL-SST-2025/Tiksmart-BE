use crate::errors::AppError;
use crate::models::auth::ResetPasswordWithOtpPayload;
use crate::models::user::{BulkCreateResponse, CreateUserPayload, ResetPasswordPayload, User};
use crate::service::{auth_service, user_service};
use crate::AppState; // Correctly imports AppState
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    Json,
};

/// Handler to get a single user by their ID.
#[tracing::instrument(skip(app_state))]
pub async fn get_user_by_id(
    State(app_state): State<AppState>, // CORRECT: Extracts the full AppState
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let user = user_service::get_by_id(&app_state.db_pool, id).await?;
    Ok(Json(user))
}

/// Handler to create a new user.
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_user(
    State(app_state): State<AppState>, // CORRECT: Extracts the full AppState
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, AppError> {
    let user = user_service::create(&app_state.db_pool, payload).await?;
    Ok(Json(user))
}

/// Maybe add later... Idk...
#[tracing::instrument(skip(app_state, payloads))]
pub async fn create_user_bulk(
    State(app_state): State<AppState>, // This handler is already correct
    Json(payloads): Json<Vec<CreateUserPayload>>,
) -> Result<Json<BulkCreateResponse>, AppError> {
    let users_created = user_service::create_bulk(&app_state.db_pool, payloads).await?;
    Ok(Json(BulkCreateResponse { users_created }))
}

#[tracing::instrument(skip(app_state, payload))]
pub async fn reset_password_handler(
    State(app_state): State<AppState>, // This handler is already correct
    Json(payload): Json<ResetPasswordPayload>,
) -> Result<StatusCode, AppError> {
    user_service::reset_password(&app_state.db_pool, payload).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handler to reset a user's password using an OTP.
#[tracing::instrument(skip(app_state, payload))]
pub async fn reset_password_with_otp_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ResetPasswordWithOtpPayload>,
) -> Result<StatusCode, AppError> {
    auth_service::reset_password_with_otp(&app_state.db_pool, payload).await?;
    Ok(StatusCode::NO_CONTENT) 
}