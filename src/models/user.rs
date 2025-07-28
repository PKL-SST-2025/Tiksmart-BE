// File: be-api/src/models/user.rs

use serde::{Deserialize, Serialize};

// The primary User model, representing a row in the `users` table.
// This is what we'll return from the database and in API responses.
#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// The Data Transfer Object (DTO) for creating a new user.
// This is what we expect to receive in the request body.
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub email: String,
    // In a real app, you'd receive a plain password and hash it in the service/handler layer.
    pub password: String,
}

#[derive(Serialize)]
pub struct BulkCreateResponse {
    pub users_created: usize,
}