// File: src/middleware/csrf_guard.rs (or wherever you place your middleware)

use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use serde_json::Value;
use crate::utils::csrf;
use crate::{api::csrf_handler::CsrfPayload, errors::AppError};
use std::mem;
use tracing::info; 

/// An Axum middleware for verifying a CSRF token from either a JSON body or a header.
pub async fn csrf_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    let method = req.method().clone();
    if matches!(method, axum::http::Method::GET | axum::http::Method::HEAD | axum::http::Method::OPTIONS) {
        return Ok(next.run(req).await);
    }
    
    let headers = req.headers().clone();
    let jar = CookieJar::from_headers(&headers);

    let original_body = mem::replace(req.body_mut(), Body::empty());
    let body_bytes = to_bytes(original_body, usize::MAX)
        .await
        .map_err(|e| {
            info!("CSRF Guard: Failed to read request body: {:?}", e);
            AppError::BadRequest("Failed to read request body".into())
        })?;

    let mut csrf_token_from_request: Option<String> = None;
    let mut debug_token_source = "None";

    // Try to parse body as JSON first
    if !body_bytes.is_empty() {
        match serde_json::from_slice::<Value>(&body_bytes) {
            Ok(json_value) => {
                if let Some(obj) = json_value.as_object() {
                    if let Some(token_val) = obj.get("csrf_token") {
                        if let Some(token_str) = token_val.as_str() {
                            csrf_token_from_request = Some(token_str.to_string());
                            debug_token_source = "JSON body field 'csrf_token'";
                        }
                    }
                }
            },
            Err(e) => {
                info!("CSRF Guard: Body is not valid JSON (or empty): {:?}", e);
            }
        }
    }
    
    // If token not found in body, check the `X-CSRF-Token` header.
    if csrf_token_from_request.is_none() {
        if let Some(header_token) = req.headers().get("X-CSRF-Token") {
            if let Ok(token_str) = header_token.to_str() {
                csrf_token_from_request = Some(token_str.to_string());
                debug_token_source = "X-CSRF-Token header";
            }
        }
    }

    // --- FIX: Get the received token first, then verify it. ---
    let received_token = csrf_token_from_request.ok_or_else(|| {
        info!("CSRF Guard: No token found in JSON body or X-CSRF-Token header. Source: {}", debug_token_source);
        AppError::BadRequest("Missing CSRF token. Expected in JSON body or X-CSRF-Token header.".into())
    })?;

    info!("CSRF Guard: Received token from {}: '{}'", debug_token_source, received_token);

    // FIX: Pass both the jar and the received_token to verify_csrf_token
    if !csrf::verify_csrf_token(&jar, &received_token) {
        // The verify_csrf_token function itself will log the hash mismatch.
        info!("CSRF Guard: VERIFICATION FAILED for received token '{}'", received_token);
        return Err(AppError::InvalidCsrfToken);
    }
    info!("CSRF Guard: Token verified successfully for method: {:?}, path: {}", method, req.uri().path());


    *req.body_mut() = axum::body::Body::from(body_bytes);

    Ok(next.run(req).await)
}