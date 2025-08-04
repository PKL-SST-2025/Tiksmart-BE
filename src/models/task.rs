// File: src/models/task.rs

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::utils::validation;

// This enum maps directly to your PostgreSQL `task_status` ENUM.
// sqlx uses this information to correctly encode and decode the `status` field.
// NOTE: Ensure the variant names and the renames match your actual DB enum definition.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "task_status")] // The name of the ENUM type in PostgreSQL
pub enum TaskStatus {
    #[sqlx(rename = "To Do")] // Maps the Rust variant `ToDo` to the DB value 'To Do'
    ToDo,
    #[sqlx(rename = "In Progress")]
    InProgress,
    Done, // Assumes this variant is named 'Done' in the DB
}

// Represents a row in the 'tasks' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    // Use the new, strongly-typed enum instead of String
    pub status: TaskStatus,
    pub project_id: i32,
    // Assuming these columns exist in your 'tasks' table based on the trigger
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

// DTO for creating a new task.

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskPayload {
    #[validate(
        length(min = 3, max = 255, message = "Title must be between 3 and 255 characters."),
        custom(function = "validation::is_safe_text")
    )]
    pub title: String,

    #[validate(
        length(max = 10000, message = "Description is too long."),
        custom(function = "validation::is_safe_text")
    )]
    pub description: Option<String>,

    pub status: Option<TaskStatus>,
}

// These structs remain the same
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TasksRequiredRoles {
    pub id: i32,
    pub task_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TasksContributors {
    pub id: i32,
    pub task_id: i32,
    pub member_id: i32,
}


#[derive(Deserialize, Validate)]
pub struct AssignContributorPayload {
    #[validate(range(min = 1, message = "A valid member ID must be provided."))]
    pub member_id: i32,
}

#[derive(Deserialize, Validate)]
pub struct AddRequiredRolePayload {
    #[validate(range(min = 1, message = "A valid role ID must be provided."))]
    pub role_id: i32,
}