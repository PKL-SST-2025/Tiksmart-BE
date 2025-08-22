// File: src/models/user.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;
use chrono::{DateTime, Utc};
// The primary User model, representing a row in the `users` table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String, 
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    
    #[serde(skip)]
    pub password_reset_token: Option<String>,
    #[serde(skip)]
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    
}

// Represents the organizer-specific data linked to a user.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OrganizerProfile {
    pub user_id: i32,
    pub company_name: Option<String>,
    pub contact_phone: Option<String>,
    pub website_url: Option<String>,
    
    // Add the new field to the model.
    pub stripe_account_id: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// The Data Transfer Object (DTO) for creating a new user.
// It now derives `Validate` and defines its own validation rules.
#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {

        #[validate(
        length(min = 3, max = 100, message = "Username must be between 3 and 100 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub username: String,

    // The `validate` attribute lets us apply various checks.
    // Here, we check the email format using our custom function.
    #[validate(
        length(min = 5, message = "Email must be at least 5 characters long."),
        custom(function = "validation::is_valid_email", message = "Invalid email format.")
    )]
    pub email: String,

    // We can also apply custom validation to the password.
    #[validate(custom(
        function = "validation::is_strong_password",
        message = "Password is not strong enough."
    ))]
    pub password: String,
}


// Payload for updating a user's password during a password reset.
#[derive(Deserialize)]
pub struct UpdateUserPasswordPayload {
    pub password: String,
}

#[derive(Serialize)]
pub struct BulkCreateResponse {
    pub users_created: usize,
}

/// This is the payload for the password reset *action*, after a user has a token.
#[derive(Deserialize, Validate)]
pub struct ResetPasswordPayload {
    #[validate(length(min = 32, message = "Reset token appears to be invalid."))]
    pub token: String,

    #[validate(custom(function = "validation::is_strong_password"))]
    pub new_password: String,
}



