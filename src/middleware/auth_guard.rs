use crate::config::CONFIG;
use crate::errors::AppError;
use crate::models::TokenClaims; // Assumes TokenClaims now has a `role: String` field
use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn auth_guard(mut req: Request, next: Next) -> Result<Response, AppError> {
    // This part remains the same: get the token from header or cookie.
    let token = get_token_from_header(req.headers().get(header::AUTHORIZATION))
        .or_else(|_| {
            let headers = req.headers().clone();
            let jar = CookieJar::from_headers(&headers);
            get_token_from_cookie(&jar)
        })?;

    // This part also remains the same: decode the token.
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| {
        tracing::warn!("Auth guard: Invalid token provided: {}", e);
        AppError::AuthFailInvalidToken
    })?
    .claims;

    // The previous version only inserted the subject (ID). Now we insert both
    // the ID and the role into the request extensions for downstream use.

    // 1. Parse the subject (sub) claim into an i32 for type safety.
    let principal_id: i32 = claims.sub.parse().map_err(|_| {
        tracing::error!("Auth guard: 'sub' claim is not a valid integer in token.");
        AppError::AuthFailInvalidToken
    })?;

    // 2. Insert the parsed ID into the request extensions.
    // Handlers will extract this with `Extension(user_id): Extension<i32>`.
    req.extensions_mut().insert(principal_id);

    // 3. Insert the role string into the request extensions.
    // The `admin_guard` middleware will extract this.
    req.extensions_mut().insert(claims.role);

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

/// Helper function to extract token from the auth cookie
fn get_token_from_cookie(jar: &CookieJar) -> Result<String, AppError> {
    jar.get("auth-token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(AppError::AuthFailTokenNotFound)
}