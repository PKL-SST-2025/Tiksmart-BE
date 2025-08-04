// File: src/models/auth.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;
// The payload for a login request.
#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,

    // For login, we just check for presence, not strength.
    #[validate(length(min = 1, message = "Password must not be empty."))]
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

#[derive(Deserialize, Validate)]
pub struct ForgotPasswordPayload {
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}