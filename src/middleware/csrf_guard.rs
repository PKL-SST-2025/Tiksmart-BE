// File: src/middleware/csrf_guard.rs (or wherever you place your middleware)

use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use crate::utils::csrf;
use crate::{api::csrf_handler::CsrfPayload, errors::AppError};
use std::mem;

/// An Axum middleware for verifying a CSRF token from a JSON body.
pub async fn csrf_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    // --- 1. Exclude certain methods from CSRF protection ---
    // CSRF protection is primarily for state-changing requests.
    // It's common to skip checks for GET, HEAD, OPTIONS.
    let method = req.method().clone();
    if matches!(method, axum::http::Method::GET | axum::http::Method::HEAD | axum::http::Method::OPTIONS) {
        return Ok(next.run(req).await);
    }
    
    // --- 2. Extract cookies and buffer the body ---
    // We need the headers to construct the CookieJar.
    let headers = req.headers().clone();
    let jar = CookieJar::from_headers(&headers);

    // Buffer the body so we can read it and then "put it back".
    // This is crucial because a request body is a stream and can only be read once.
    // Take ownership of the body, replacing it with an empty one in the request.
    let original_body = mem::replace(req.body_mut(), Body::empty());
    
    // Buffer the owned body. This now compiles because `original_body` is `Body`, not `&mut Body`.
    let body_bytes = to_bytes(original_body, usize::MAX)
        .await
        .map_err(|_| AppError::BadRequest("Failed to read request body".into()))?;

    // --- 3. Deserialize and verify the token ---
    // Now we can deserialize the buffered bytes into our payload.
    // We use `serde_json::from_slice` because we have bytes, not a string.
    let payload: CsrfPayload = serde_json::from_slice(&body_bytes)
        .map_err(|_| AppError::BadRequest("Invalid JSON body or missing csrf_token field.".into()))?;

    // Use your existing verification logic.
    if !csrf::verify_csrf_token(&jar, &payload.csrf_token) {
        // If the token is invalid, we reject the request.
        tracing::warn!("Invalid CSRF token received.");
        return Err(AppError::InvalidCsrfToken);
    }

    // --- 4. Reconstruct the request and pass it to the next handler ---
    // The token is valid, so we put the buffered body back into the request.
    // This allows the downstream handler (e.g., your `protected_endpoint`)
    // to extract the JSON payload as if this middleware never touched it.
    *req.body_mut() = axum::body::Body::from(body_bytes);

    Ok(next.run(req).await)
}