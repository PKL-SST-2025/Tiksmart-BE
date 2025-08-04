// File: src/models/project.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;


// Represents a row in the 'projects' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: i32,
    pub project_name: String,
    pub business_name: Option<String>,
    pub description: Option<String>,
    pub owner_user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// DTO for creating a new project.

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectPayload {
    #[validate(
        length(min = 3, max = 100, message = "Project name must be between 3 and 100 characters.")
    )]
#[validate(
        length(min = 3, max = 100, message = "Project name must be between 3 and 100 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub project_name: String,

    // Apply the same correct pattern to the optional fields.
    #[validate(
        length(min = 3, max = 100, message = "Business name must be between 3 and 100 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub business_name: Option<String>,

    #[validate(
        length(min = 10, max = 5000, message = "Description must be between 10 and 5000 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub description: Option<String>,
}