// File: src/models/member.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;
// Represents a row in the 'members' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub role_id: Option<i32>,
    pub full_name: String,
    pub is_banned: bool,
}

// DTO for adding a new member to a project.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateMemberPayload {
    #[validate(range(min = 1, message = "A valid user ID must be provided."))]
    pub user_id: i32,

    // Also validate optional fields. It's skipped if None.
    #[validate(range(min = 1, message = "A valid role ID must be provided."))]
    pub role_id: Option<i32>,

    #[validate(
        length(min = 2, max = 100, message = "Full name must be between 2 and 100 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub full_name: String,
}