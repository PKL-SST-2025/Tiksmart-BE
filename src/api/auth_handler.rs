// File: be-api/src/api/auth_handler.rs

use crate::errors::AppError;
use crate::models::{LoginPayload, User}; 
use crate::service::{auth_service, user_service}; 
use crate::models::auth::ForgotPasswordPayload;

use axum::{
    extract::{State, Extension}, 
    Json,
};
use sqlx::PgPool;

/// The handler for the login endpoint.
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Call the auth service to perform the login.
    let login_response = auth_service::login(&pool, payload).await?;

    // Return the token as JSON.
    Ok(Json(serde_json::json!(login_response)))
}

pub async fn get_me(
    State(pool): State<PgPool>,
    Extension(user_id): Extension<i32>, // <-- Extract the user ID from the middleware
) -> Result<Json<User>, AppError> {
    // We can trust user_id because the auth_guard middleware has validated it.
    let user = user_service::get_by_id(&pool, user_id).await?;

    Ok(Json(user))
}

/// Handler to initiate the "forgot password" flow.
pub async fn forgot_password(
    State(pool): State<PgPool>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<(), AppError> {
    auth_service::forgot_password(&pool, payload.email).await?;
    // Return a 200 OK with no body. We don't want to reveal if the email exists or not.
    Ok(())
}