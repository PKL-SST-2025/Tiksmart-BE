use crate::db::role_query;
use crate::errors::AppError;
use crate::models::{CreateRolePayload, Role};
use sqlx::PgPool;
use crate::utils::validation;

/// Service function for creating a new role within a project.

pub async fn create_role_for_project(
    pool: &PgPool,
    project_id: i32,
    payload: CreateRolePayload,
) -> Result<Role, AppError> {
    // ADDED: Validate the incoming payload.
    validation::validate_payload(&payload)?;

    role_query::create(pool, project_id, payload).await
}

pub async fn get_roles_for_project(pool: &PgPool, project_id: i32) -> Result<Vec<Role>, AppError> {
    role_query::get_by_project_id(pool, project_id).await
}