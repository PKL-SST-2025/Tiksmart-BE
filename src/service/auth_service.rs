// File: src/service/auth_service.rs (or wherever you have this code)

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use cookie::{time::Duration, Cookie, SameSite};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    config::CONFIG,
    db::user_query,
    errors::AppError,
    models::{auth::LoginPayload, user::CreateUserPayload},
    service::user_service,
    utils::{jwt, validation},
};


use bcrypt::{hash, verify, DEFAULT_COST};
// --- Response Body Structs ---
// It's good practice to define the exact shape of your JSON responses.

#[derive(Serialize)]
pub struct AuthSuccessResponse<T: Serialize> {
    status: &'static str,
    data: T,
}

#[derive(Serialize)]
pub struct GenericMessageResponse {
    status: &'static str,
    message: String,
}

// --- Public Service Functions ---
// These are the functions your Axum handlers will call.

/// Validates user credentials, and on success, returns a `Response`
/// containing the auth cookie.
pub async fn login(pool: &PgPool, payload: LoginPayload) -> Result<Response, AppError> {
    validation::validate_payload(&payload)?;

    let user_auth_data = user_query::get_auth_data_by_email(pool, &payload.email).await?;

    let is_valid = bcrypt::verify(&payload.password, &user_auth_data.password_hash)
        .map_err(|_| AppError::InternalServerError("Password hashing error".to_string()))?; // More specific error

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // The JSON body for the response
    let response_body = GenericMessageResponse {
        status: "success",
        message: "Login successful.".to_string(),
    };
    
    // Use the helper to build the final response with the cookie
    build_auth_response(user_auth_data.id, response_body)
}

/// Creates a new user and immediately logs them in by returning a `Response`
/// with an auth cookie.
pub async fn register(pool: &PgPool, payload: CreateUserPayload) -> Result<Response, AppError> {
    // This handles validation, hashing, and database insertion.
    let new_user = user_service::create(pool, payload).await?;
    let user_id = new_user.id;

    // The JSON body will be the newly created user object.
    let response_body = AuthSuccessResponse {
        status: "success",
        data: new_user,
    };

    // Use the helper to build the response with the cookie.
    build_auth_response(user_id, response_body)
}

/// Logs a user out by returning a `Response` that clears the auth cookie.
pub async fn logout() -> Result<Response, AppError> {
    let response_body = GenericMessageResponse {
        status: "success",
        message: "Logged out successfully.".to_string(),
    };

    // Use the new logout helper to build the response.
    build_logout_response(response_body)
}


// --- Private Helper Functions ---
// These helpers handle the repetitive task of creating responses with cookies.

/// Builds a response with a secure, HttpOnly auth cookie.
fn build_auth_response<T: Serialize>(user_id: i32, response_body: T) -> Result<Response, AppError> {
    // 1. Create the JWT
    let token = jwt::create_jwt(user_id)?;

    // 2. Create the cookie
    let auth_cookie = Cookie::build(("auth-token", token))
        .path("/")
        .max_age(Duration::seconds((CONFIG.jwt_expiration_hours * 3600) as i64))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(CONFIG.env != "development")
        .finish();

    // 3. Build the final response
    let mut response = (StatusCode::OK, Json(response_body)).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, auth_cookie.to_string().parse()?);

    Ok(response)
}

/// Builds a response that clears the auth cookie from the browser.
fn build_logout_response<T: Serialize>(response_body: T) -> Result<Response, AppError> {
    // Create an "expired" cookie to overwrite the existing one in the browser.
    let clearing_cookie = Cookie::build(("auth-token", "")) // Set value to empty
        .path("/")
        .max_age(Duration::ZERO) // Set max_age to 0 to expire immediately
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(CONFIG.env != "development")
        .finish();

    let mut response = (StatusCode::OK, Json(response_body)).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, clearing_cookie.to_string().parse()?);

    Ok(response)
}

/// Hashes a plain-text password using bcrypt.
pub async fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::Bcrypt(e)) // Map bcrypt error to AppError
}


/// Verifies a plain-text password against a bcrypt hash.
pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| AppError::Bcrypt(e)) // Map bcrypt error to AppError
}