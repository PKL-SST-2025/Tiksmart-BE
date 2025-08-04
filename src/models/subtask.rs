// File: src/models/subtask.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;

// Represents a row in the 'subtasks' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Subtask {
    pub id: i32,
    pub description: String,
    pub is_completed: bool,
    pub task_id: i32,
}

// DTO for creating a new subtask.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSubtaskPayload {
    #[validate(
        length(min = 1, max = 1000, message = "Description must be between 1 and 1000 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub description: String,

    pub is_completed: Option<bool>,
}