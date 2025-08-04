use crate::db::subtask_query;
use crate::errors::AppError;
use crate::models::{CreateSubtaskPayload, Subtask};
use sqlx::PgPool;
use crate::utils::validation;

/// Service for creating a new subtask for a parent task.
pub async fn create_subtask(
    pool: &PgPool,
    task_id: i32,
    payload: CreateSubtaskPayload,
) -> Result<Subtask, AppError> {
    // ADDED: Validate the incoming payload.
    validation::validate_payload(&payload)?;

    subtask_query::create(pool, task_id, payload).await
}

/// Service to get all subtasks for a parent task.
pub async fn get_subtasks_for_task(pool: &PgPool, task_id: i32) -> Result<Vec<Subtask>, AppError> {
    subtask_query::get_by_task_id(pool, task_id).await
}