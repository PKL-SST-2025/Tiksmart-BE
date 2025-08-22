// File: src/service/auth_service.rs (or wherever you have this code)

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use cookie::{time::Duration, Cookie, SameSite};
use rand::Rng;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    config::CONFIG,
    db::user_query,
    errors::AppError,
    models::{auth::{ForgotPasswordPayload, LoginPayload, ResetPasswordWithOtpPayload}, user::CreateUserPayload, TokenClaims},
    service::user_service,
    utils::{jwt, validation},
};
use crate::db::admin_query;



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
    let token = jwt::create_jwt(user_id, "user")?;

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

/// Service to handle the login process for an administrator.
/// It authenticates against the `admins` table and issues a token with an "admin" role.
pub async fn admin_login(pool: &PgPool, payload: LoginPayload) -> Result<Response, AppError> {
    // 1. Validate the payload.
    validation::validate_payload(&payload)?;

    // 2. Fetch admin auth data from the database.
    let admin_data = admin_query::get_auth_data_by_email(pool, &payload.email).await?;

    // 3. Verify the password.
    let is_valid_password = bcrypt::verify(&payload.password, &admin_data.password_hash)?;
    if !is_valid_password {
        return Err(AppError::InvalidCredentials);
    }

    // 4. Create the JWT claims with the "admin" role.
    let claims = TokenClaims {
        sub: admin_data.id.to_string(),
        role: "admin".to_string(), // Crucial part for the admin guard
        exp: (Utc::now() + chrono::Duration::hours(CONFIG.jwt_expiration_hours)).timestamp()
            as usize,
    };

    // 5. Generate the JWT and create the response with the cookie.
    build_auth_response(admin_data.id, claims)
}

/// Generates a password reset OTP, saves it to the DB, and returns it.
pub async fn request_password_reset(
    pool: &PgPool,
    payload: ForgotPasswordPayload,
) -> Result<String, AppError> {
    // 1. Validate the payload.
    validation::validate_payload(&payload)?;

    // 2. Generate a 6-digit random code.
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();
    let reset_code: String = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    // 3. Define the expiration time (e.g., 10 minutes from now).
    let expires_at = Utc::now() + chrono::Duration::minutes(10);

    // 4. Save the token and expiration to the user's record.
    // We handle the RowNotFound case gracefully. From the user's perspective, the
    // request is always "successful" to prevent email enumeration attacks.
    if let Err(e) = user_query::set_password_reset_token(
        pool,
        &payload.email,
        &reset_code,
        expires_at,
    )
    .await
    {
        // Only fail on actual database errors, not if the user wasn't found.
        if !matches!(e, AppError::Sqlx(sqlx::Error::RowNotFound)) {
            return Err(e);
        }
    }

    // 5. Return the generated code for the handler to send back.
    Ok(reset_code)
}



/// Verifies a password reset OTP and updates the user's password if the OTP is valid.
pub async fn reset_password_with_otp(
    pool: &PgPool,
    payload: ResetPasswordWithOtpPayload,
) -> Result<(), AppError> {
    // 1. Validate the payload format (strong password, email format, etc.).
    validation::validate_payload(&payload)?;

    // 2. Fetch the user's current reset token information from the database.
    // If the email doesn't exist, this will return a RowNotFound error, which becomes a 404.
    // This is acceptable here, as the user must prove they know the OTP.
    let user_data = user_query::get_user_for_otp_verification(pool, &payload.email).await?;

    // 3. Perform the core verification checks.
    let stored_token = user_data.password_reset_token.ok_or_else(|| {
        AppError::BadRequest("No password reset is pending for this account.".to_string())
    })?;

    if stored_token != payload.otp {
        return Err(AppError::BadRequest("Invalid OTP code.".to_string()));
    }

    let expires_at = user_data.password_reset_expires_at.ok_or_else(|| {
        // This case should not happen if a token exists, but we handle it for safety.
        AppError::InternalServerError("Password reset token is missing an expiration date.".to_string())
    })?;

    if Utc::now() > expires_at {
        return Err(AppError::BadRequest("The OTP code has expired.".to_string()));
    }

    // 4. If all checks pass, hash the new password.
    let new_password_hash = hash_password(&payload.new_password).await?;

    // 5. Update the user's password in the database and clear the OTP fields.
    user_query::update_password_and_clear_otp(pool, user_data.id, &new_password_hash).await?;

    Ok(())
}