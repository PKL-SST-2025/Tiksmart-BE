use crate::errors::AppError;
use crate::utils::csrf;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CsrfTokenResponse {
    pub csrf_token: String,
}

// THE FIX: Renamed this to be more generic for a JSON payload.
#[derive(Deserialize)]
pub struct CsrfPayload {
    pub csrf_token: String,
}

/// GET /api/csrf/token
/// Handler to provide a fresh, raw CSRF token to the client.
/// It also sets a secure, HttpOnly cookie containing the *hashed* version of the token.
pub async fn get_csrf_token(jar: CookieJar) -> (CookieJar, Json<CsrfTokenResponse>) {
    let (jar, raw_token) = csrf::create_csrf_cookie(jar);
    (jar, Json(CsrfTokenResponse { csrf_token: raw_token }))
}

/// POST /api/csrf/protected-endpoint
/// An example of a handler that verifies a CSRF token from a JSON payload.
pub async fn protected_endpoint(
    jar: CookieJar,
    // THE FIX: Expect a JSON payload, not a Form, for API consistency.
    Json(payload): Json<CsrfPayload>,
) -> Result<impl IntoResponse, AppError> {
    if csrf::verify_csrf_token(&jar, &payload.csrf_token) {
        // The token is valid, you can proceed with the real business logic.
        Ok((StatusCode::OK, "CSRF token is valid. Action processed."))
    } else {
        // The token is invalid or the cookie is missing.
        // We return a 403 Forbidden error.
        Err(AppError::InvalidCsrfToken)
    }
}