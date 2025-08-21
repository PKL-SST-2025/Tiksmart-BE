use crate::errors::AppError;
use crate::models::TokenClaims;
use crate::config::CONFIG; 
use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};


pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    // FIX: Get the token from either the header OR the cookie
    let token = get_token_from_header(req.headers().get(header::AUTHORIZATION))
        .or_else(|_| {
            // If header fails, try to get it from the cookie.
            // This is how we authenticate the WebSocket handshake.
            let headers = req.headers().clone();
            let jar = CookieJar::from_headers(&headers);
            get_token_from_cookie(&jar)
        })?;

    // Use the secret from the config.
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::warn!("Auth guard: Invalid token provided: {}", e); // Log the JWT error
        AppError::AuthFailInvalidToken
    })?
    .claims;

    // Add the user ID from the claims to the request extensions.
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

// NEW: Helper function to extract token from the auth cookie
fn get_token_from_cookie(jar: &CookieJar) -> Result<String, AppError> {
    jar.get("auth-token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(AppError::AuthFailTokenNotFound)
}