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
    pub sub: String,    // Subject (user_id or admin_id)
    pub role: String,   // The role, e.g., "user" or "admin"
    pub exp: usize,     // Expiration time
}
#[derive(Deserialize, Validate)]
pub struct ForgotPasswordPayload {
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}


// Add this new struct for the response.
#[derive(Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
    pub reset_code: String, // We return the code for "mail js" to use
}

// Add this new struct for the OTP-based password reset action.
#[derive(Deserialize, Validate)]
pub struct ResetPasswordWithOtpPayload {
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,

    #[validate(length(equal = 6, message = "OTP must be 6 digits."))]
    pub otp: String,

    #[validate(custom(function = "validation::is_strong_password"))]
    pub new_password: String,
}