// File: src/db/member_query.rs

use crate::errors::AppError;
use crate::models::{CreateMemberPayload, Member};
// Import the necessary traits
use sqlx::{Executor, PgPool, Postgres};

/// Adds a user to a project as a new member.
/// Generic over the executor to allow use in transactions.
pub async fn create<'c, E>(
    executor: E, // CHANGED: from `pool: &PgPool` to a generic executor
    project_id: i32,
    payload: CreateMemberPayload,
) -> Result<Member, AppError>
where
    E: Executor<'c, Database = Postgres>,
{
    let member = sqlx::query_as!(
        Member,
        "INSERT INTO members (user_id, project_id, role_id, full_name)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
        payload.user_id,
        project_id,
        payload.role_id,
        payload.full_name
    )
    .fetch_one(executor) // CHANGED: Use the generic executor here
    .await?;

    Ok(member)
}

// These functions likely don't need to be in transactions, so &PgPool is fine.
pub async fn get_by_project_id(pool: &PgPool, project_id: i32) -> Result<Vec<Member>, AppError> {
    let members = sqlx::query_as!(
        Member,
        "SELECT * FROM members WHERE project_id = $1",
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(members)
}

pub async fn get_by_user_and_project(
    pool: &PgPool,
    user_id: i32,
    project_id: i32,
) -> Result<Member, AppError> {
    let member = sqlx::query_as!(
        Member,
        "SELECT * FROM members WHERE user_id = $1 AND project_id = $2",
        user_id,
        project_id
    )
    .fetch_one(pool)
    .await?;

    Ok(member)
}

/// Fetches a single member from the database by their primary key.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Member, AppError> {
    let member = sqlx::query_as!(Member, "SELECT * FROM members WHERE id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(member)
}