use crate::errors::AppError;
use crate::models::task::{AddRequiredRolePayload, AssignContributorPayload};
use crate::models::{CreateTaskPayload, Task};
use crate::service::{project_service, task_service};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sqlx::PgPool;

/// POST /api/projects/:project_id/tasks
pub async fn create_task(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Task>, AppError> {
    // --- Authorization ---
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let task = task_service::create_task(&pool, project_id, payload).await?;
    Ok(Json(task))
}

/// GET /api/projects/:project_id/tasks
pub async fn get_tasks_for_project(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> Result<Json<Vec<Task>>, AppError> {
    // --- Authorization ---
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let tasks = task_service::get_tasks_for_project(&pool, project_id).await?;
    Ok(Json(tasks))
}


// Endpoint to assign a contributor.
#[tracing::instrument(skip(pool, payload))]
pub async fn assign_contributor_handler(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(payload): Json<AssignContributorPayload>,
) -> Result<StatusCode, AppError> {
    task_service::add_contributor_to_task(&pool, user_id, task_id, payload).await?;
    Ok(StatusCode::CREATED)
}

// Endpoint to add a required role.
#[tracing::instrument(skip(pool, payload))]
pub async fn add_required_role_handler(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(payload): Json<AddRequiredRolePayload>,
) -> Result<StatusCode, AppError> {
    task_service::add_required_role_to_task(&pool, user_id, task_id, payload).await?;
    Ok(StatusCode::CREATED)
}