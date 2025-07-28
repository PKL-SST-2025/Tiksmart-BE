use crate::errors::AppError;
use crate::utils::csrf;
use axum::{
    extract::{Form, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CsrfTokenResponse {
    pub csrf_token: String,
}

#[derive(Deserialize)]
struct SubmitRequest {
    message: String,
    csrf_token: String,
}

#[derive(Deserialize)]
pub struct Keys {
    pub authenticity_token: String,
}

/// GET /csrf-token — Returns a fresh CSRF token and sets the secure cookie.
pub async fn get_token_handler(jar: CookieJar) -> (CookieJar, Json<CsrfTokenResponse>) {
    let (jar, raw_token) = csrf::create_csrf_cookie(jar);
    (jar, Json(CsrfTokenResponse { csrf_token: raw_token }))
}

/// POST /csrf-check — Verifies the submitted token against the hashed cookie.
pub async fn protected_post_handler(jar: CookieJar, Form(payload): Form<Keys>) -> impl IntoResponse {
    if csrf::verify_csrf_token(&jar, &payload.authenticity_token) {
        "✅ Token is valid. You can proceed."
    } else {
        "❌ Token is invalid or missing."
    }
}