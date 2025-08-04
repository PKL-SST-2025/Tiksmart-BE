// File: src/db/role_query.rs

use crate::errors::AppError;
use crate::models::{CreateRolePayload, Role};
use sqlx::PgPool;

/// Creates a new role within a project.
pub async fn create(
    pool: &PgPool,
    project_id: i32,
    payload: CreateRolePayload,
) -> Result<Role, AppError> {
    let role = sqlx::query_as!(
        Role,
        "INSERT INTO roles (role, description, project_id)
         VALUES ($1, $2, $3)
         RETURNING *",
        payload.role,
        payload.description,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(role)
}

/// Fetches all roles for a given project.
pub async fn get_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<Role>, AppError> {
    let roles = sqlx::query_as!(
        Role,
        "SELECT * FROM roles WHERE project_id = $1",
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(roles)
}

/// Fetches a single role from the database by its primary key.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Role, AppError> {
    let role = sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(role)
}