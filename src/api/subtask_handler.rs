use crate::errors::AppError;
use crate::models::{CreateSubtaskPayload, Subtask};
use crate::service::{subtask_service, task_service};
use axum::{
extract::{Path, State},
Extension, Json,
};
use sqlx::PgPool;

/// POST /api/tasks/:task_id/subtasks
pub async fn create_subtask(
Extension(user_id): Extension<i32>,
State(pool): State<PgPool>,
Path(task_id): Path<i32>,
Json(payload): Json<CreateSubtaskPayload>,
) -> Result<Json<Subtask>, AppError> {
    // --- Authorization ---
    // A user can only add a subtask if they are a member of the project the parent task belongs to.
    task_service::authorize_user_for_task(&pool, user_id, task_id).await?;

    let subtask = subtask_service::create_subtask(&pool, task_id, payload).await?;
    Ok(Json(subtask))
}

/// GET /api/tasks/:task_id/subtasks
pub async fn get_subtasks_for_task(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(task_id): Path<i32>,
    ) -> Result<Json<Vec<Subtask>>, AppError> {
    // --- Authorization ---
    task_service::authorize_user_for_task(&pool, user_id, task_id).await?;
    let subtasks = subtask_service::get_subtasks_for_task(&pool, task_id).await?;

    Ok(Json(subtasks))
}