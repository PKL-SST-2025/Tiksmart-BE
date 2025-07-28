// File: src/models/auth.rs

use serde::{Deserialize, Serialize};

// The payload for a login request.
#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

// The response after a successful login.
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// The data we'll encode into the JWT.
// `sub` is the standard claim for "subject" (the user ID).
// `exp` is the standard claim for "expiration time".
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct ForgotPasswordPayload {
    pub email: String,
}
