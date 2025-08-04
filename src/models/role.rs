// File: src/models/role.rs

use serde::{Deserialize, Serialize};
use crate::utils::validation;
use validator::Validate;

// Represents a row in the 'roles' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Role {
    pub id: i32,
    pub role: String,
    pub description: Option<String>,
    pub project_id: i32,
}

// DTO for creating a new role.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateRolePayload {
    #[validate(
    length(min = 2, max = 100, message = "Role name must be between 2 and 100 characters."),
    custom(function = "validation::is_safe_text")
    )]
    pub role: String,
        
    #[validate(
        length(max = 500, message = "Description must not exceed 500 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub description: Option<String>,
}