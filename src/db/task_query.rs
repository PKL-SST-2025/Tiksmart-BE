// File: src/db/task_query.rs

use crate::errors::AppError;
use crate::models::task::TaskStatus;
// Import the new enum along with the other models
use crate::models::{CreateTaskPayload, Task};
use sqlx::PgPool;

/// Creates a new task within a project.
pub async fn create(
    pool: &PgPool,
    project_id: i32,
    payload: CreateTaskPayload,
) -> Result<Task, AppError> {
    let task = sqlx::query_as!(
        Task,
        // We explicitly list the returned columns and provide a type hint for the enum.
        // The `status as "status: _"` syntax tells sqlx to infer the type from the `Task` struct's `status` field.
        r#"
        INSERT INTO tasks (title, description, status, project_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, description, status as "status: _", project_id, created_at, last_updated
        "#,
        payload.title,
        payload.description,
        // We now pass the enum variant directly, not a string.
        payload.status.unwrap_or(TaskStatus::ToDo) as TaskStatus,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(task)
}

/// Fetches all tasks for a given project.
pub async fn get_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as!(
        Task,
        // We use the same type hint (`status as "status: _"`) for selecting.
        r#"
        SELECT id, title, description, status as "status: _", project_id, created_at, last_updated
        FROM tasks
        WHERE project_id = $1
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(tasks)
}

/// Assigns a member (contributor) to a specific task.
pub async fn add_contributor(pool: &PgPool, task_id: i32, member_id: i32) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO tasks_contributors (task_id, member_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        task_id,
        member_id
    )
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Task, AppError> {
    let task = sqlx::query_as!(
        Task,
        r#"SELECT id, title, description, status as "status: _", project_id, created_at, last_updated FROM tasks WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(task)
}

// NEW: A query to add a required role to a task.
pub async fn add_required_role(pool: &PgPool, task_id: i32, role_id: i32) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO tasks_required_roles (task_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        task_id,
        role_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
