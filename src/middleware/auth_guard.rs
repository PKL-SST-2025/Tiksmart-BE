use crate::errors::AppError;
use crate::models::TokenClaims;
use crate::config::CONFIG; 
use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    let token = get_token_from_header(req.headers().get(header::AUTHORIZATION))?;

    // Use the secret from the config.
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::AuthFailInvalidToken)?
    .claims;

    // Add the user ID from the claims to the request extensions.
    // This makes the user ID available to the handlers.
    req.extensions_mut().insert(claims.sub);

    // If all is well, pass the request to the next layer.
    Ok(next.run(req).await)
}

/// Helper function to extract a token from an optional HeaderValue.
fn get_token_from_header(header: Option<&HeaderValue>) -> Result<String, AppError> {
    let header = header.ok_or(AppError::AuthFailTokenNotFound)?;
    let header_str = header.to_str().map_err(|_| AppError::AuthFailInvalidToken)?;

    let parts: Vec<&str> = header_str.split_whitespace().collect();
    if parts.len() == 2 && parts[0] == "Bearer" {
        Ok(parts[1].to_string())
    } else {
        Err(AppError::AuthFailInvalidToken)
    }
}