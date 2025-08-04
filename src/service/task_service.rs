
use crate::db::{member_query, role_query, task_query};
use crate::errors::AppError;
use crate::models::task::{AddRequiredRolePayload, AssignContributorPayload};
use crate::models::{CreateTaskPayload, Task};
use sqlx::PgPool;
use crate::utils::validation;
use crate::service::project_service;

/// The missing authorization function.
/// It checks if a user has permission to access a task by checking
/// their membership in the task's parent project.
pub async fn authorize_user_for_task(
    pool: &PgPool,
    user_id: i32,
    task_id: i32,
) -> Result<(), AppError> {
    // First, get the task to find out its project_id.
    // (This requires a new `get_by_id` function in `task_query.rs`)
    let task = task_query::get_by_id(pool, task_id).await?;

    // Now, use the project service to see if the user is a member of that project.
    project_service::authorize_user_for_project(pool, user_id, task.project_id).await
}

/// Service function for creating a new task.
pub async fn create_task(
    pool: &PgPool,
    project_id: i32,
    payload: CreateTaskPayload,
) -> Result<Task, AppError> {
    validation::validate_payload(&payload)?;
    task_query::create(pool, project_id, payload).await
}

/// Service to get all tasks for a project.
pub async fn get_tasks_for_project(pool: &PgPool, project_id: i32) -> Result<Vec<Task>, AppError> {
    task_query::get_by_project_id(pool, project_id).await
}

/// The service logic for adding a contributor to a task.
pub async fn add_contributor_to_task(
    pool: &PgPool,
    requester_id: i32,
    task_id: i32,
    payload: AssignContributorPayload,
) -> Result<(), AppError> {
    validation::validate_payload(&payload)?;

    // 1. Authorize: Can the requester even modify this task?
    authorize_user_for_task(pool, requester_id, task_id).await?;

    // 2. Business Logic: Ensure the member being added belongs to the same project as the task.
    let task = task_query::get_by_id(pool, task_id).await?;
    let member = member_query::get_by_id(pool, payload.member_id).await?; // This call now works.
    if task.project_id != member.project_id {
        return Err(AppError::BadRequest(
            "Cannot assign a member from a different project.".to_string(),
        ));
    }

    // 3. Action
    // THE FIX: Pass the `member_id` from the payload, not the whole payload struct.
    task_query::add_contributor(pool, task_id, payload.member_id).await
}

/// The business logic for adding a required role to a task.
pub async fn add_required_role_to_task(
    pool: &PgPool,
    requester_id: i32,
    task_id: i32,
    payload: AddRequiredRolePayload,
) -> Result<(), AppError> {
    validation::validate_payload(&payload)?;

    // 1. Authorize requester.
    authorize_user_for_task(pool, requester_id, task_id).await?;

    // 2. Business Logic: Ensure role and task are in the same project.
    let task = task_query::get_by_id(pool, task_id).await?;
    let role = role_query::get_by_id(pool, payload.role_id).await?; // This call now works.
    if task.project_id != role.project_id {
        return Err(AppError::BadRequest(
            "Cannot assign a role from a different project.".to_string(),
        ));
    }

    // 3. Action
    task_query::add_required_role(pool, task_id, payload.role_id).await
}