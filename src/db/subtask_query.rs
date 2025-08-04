// File: src/db/subtask_query.rs

use crate::errors::AppError;
use crate::models::{CreateSubtaskPayload, Subtask};
use sqlx::PgPool;

/// Creates a new subtask for a parent task.
pub async fn create(
    pool: &PgPool,
    task_id: i32,
    payload: CreateSubtaskPayload,
) -> Result<Subtask, AppError> {
    let subtask = sqlx::query_as!(
        Subtask,
        "INSERT INTO subtasks (description, is_completed, task_id)
         VALUES ($1, $2, $3)
         RETURNING *",
        payload.description,
        payload.is_completed.unwrap_or(false),
        task_id
    )
    .fetch_one(pool)
    .await?;

    Ok(subtask)
}

/// Fetches all subtasks for a given parent task.
pub async fn get_by_task_id(pool: &PgPool, task_id: i32) -> Result<Vec<Subtask>, AppError> {
    let subtasks = sqlx::query_as!(
        Subtask,
        "SELECT * FROM subtasks WHERE task_id = $1",
        task_id
    )
    .fetch_all(pool)
    .await?;

    Ok(subtasks)
}
