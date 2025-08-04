// File: src/api/user_handler.rs

use crate::errors::AppError;
use crate::models::{CreateUserPayload, User};
use crate::models::user::{BulkCreateResponse, ResetPasswordPayload};
use crate::service::{auth_service, user_service};
use crate::AppState; use axum::http::StatusCode;
// <-- Import the service
use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;

/// Handler to get a single user by their ID.
#[tracing::instrument(skip(pool))] // <-- Add this attribute
pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let user = user_service::get_by_id(&pool, id).await?;
    Ok(Json(user))
}

/// Handler to create a new user.
#[tracing::instrument(skip(pool, payload))] // <-- Add this attribute
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<Json<User>, AppError> {
    let user = user_service::create(&pool, payload).await?;
    Ok(Json(user))
}

// Maybe add later... Idk...
#[tracing::instrument(skip(app_state, payloads))]
pub async fn create_user_bulk(
    State(app_state): State<AppState>,
    Json(payloads): Json<Vec<CreateUserPayload>>,
) -> Result<Json<BulkCreateResponse>, AppError> {
    let users_created = user_service::create_bulk(&app_state.db_pool, payloads).await?;
    Ok(Json(BulkCreateResponse { users_created }))
}

#[tracing::instrument(skip(app_state, payload))]
pub async fn reset_password_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> Result<StatusCode, AppError> {
    user_service::reset_password(&app_state.db_pool, payload).await?;
    Ok(StatusCode::OK)
}


