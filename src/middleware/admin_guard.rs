// File: src/middleware/admin_guard.rs

use crate::errors::AppError;
use axum::{
    extract::Request,
    http::Response,
    middleware::Next,
};
use axum::body::Body;

pub async fn admin_guard(request: Request, next: Next) -> Result<Response<Body>, AppError> {
    // The role is inserted into extensions by the preceding `auth_guard`.
    let role = request.extensions().get::<String>().cloned().ok_or_else(|| {
        // This indicates a server logic error (auth_guard didn't insert the role).
        AppError::InternalServerError("Role missing from request extensions.".to_string())
    })?;

    if role == "admin" {
        // If the role is "admin", allow the request to proceed.
        Ok(next.run(request).await)
    } else {
        // Otherwise, reject it with a 403 Forbidden error.
        Err(AppError::Forbidden(
            "Administrator access is required for this resource.".to_string(),
        ))
    }
}