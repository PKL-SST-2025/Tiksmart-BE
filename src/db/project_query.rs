// File: src/db/project_query.rs

use crate::errors::AppError;
use crate::models::{CreateProjectPayload, Project};
// Import the necessary traits
use sqlx::{Executor, PgPool, Postgres};

/// Creates a new project in the database.
/// This function is generic over the executor type, so it can be used with
/// a connection pool or a transaction.
pub async fn create<'c, E>(
    executor: E, // CHANGED: from `pool: &PgPool` to a generic executor
    owner_user_id: i32,
    payload: CreateProjectPayload,
) -> Result<Project, AppError>
where
    E: Executor<'c, Database = Postgres>, // The executor must be for a Postgres database
{
    let project = sqlx::query_as!(
        Project,
        "INSERT INTO projects (project_name, business_name, description, owner_user_id)
         VALUES ($1, $2, $3, $4)
         RETURNING id, project_name, business_name, description, owner_user_id, created_at",
        payload.project_name,
        payload.business_name,
        payload.description,
        owner_user_id
    )
    .fetch_one(executor) // CHANGED: Use the generic executor here
    .await?;

    Ok(project)
}

// Functions that DON'T need to be in a transaction can keep taking a &PgPool for simplicity.
pub async fn get_all_by_owner(pool: &PgPool, owner_user_id: i32) -> Result<Vec<Project>, AppError> {
    let projects = sqlx::query_as!(
        Project,
        "SELECT id, project_name, business_name, description, owner_user_id, created_at
         FROM projects WHERE owner_user_id = $1",
        owner_user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn get_all_for_member(pool: &PgPool, user_id: i32) -> Result<Vec<Project>, AppError> {
    let projects = sqlx::query_as!(
        Project,
        "SELECT p.id, p.project_name, p.business_name, p.description, p.owner_user_id, p.created_at
         FROM projects p
         INNER JOIN members m ON p.id = m.project_id
         WHERE m.user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Project, AppError> {
    let project = sqlx::query_as!(
        Project,
        "SELECT id, project_name, business_name, description, owner_user_id, created_at
         FROM projects WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(project)
}