// File: be-api/src/errors/mod.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Refactor the enum with `thiserror` attributes <--
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Not authenticated")]
    AuthFailTokenNotFound,

    #[error("Invalid token")]
    AuthFailInvalidToken,

    #[error("Failed to create token")]
    JwtCreationError,

    // This will automatically implement `From<sqlx::Error> for AppError`
    #[error("Database error")]
    Sqlx(#[from] sqlx::Error),

    // This will automatically implement `From<bcrypt::BcryptError> for AppError`
    #[error("Hashing error")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

// This is the magic: we implement `IntoResponse` for our `AppError`.
// This tells Axum how to convert our error type into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // This logs the error variant and its contents as a structured field.
        tracing::error!(error = ?self, "Error response generated");


        let (status, error_message) = match self {
            // 400 - Bad Request
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),

            // 401 - Unauthorized
            AppError::InvalidCredentials
            | AppError::AuthFailTokenNotFound
            | AppError::AuthFailInvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),

            // 404 - Not Found (from a specific database error)
            AppError::Sqlx(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }

            // 500 - Internal Server Error (for all other errors)
            AppError::JwtCreationError | AppError::Sqlx(_) | AppError::Bcrypt(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occurred".to_string())
            }
        };
        
        // Create a JSON response body.
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}