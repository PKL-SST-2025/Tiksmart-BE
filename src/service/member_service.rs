use crate::db::{member_query, user_query};
use crate::errors::AppError;
use crate::models::{CreateMemberPayload, Member};
use crate::utils::validation; 
use sqlx::PgPool;

/// Service function to add a new member to a project.
///
/// Business Logic:
/// 1. Checks if the user being added actually exists.
/// 2. Adds the user to the project via the database query.
pub async fn add_member_to_project(
    pool: &PgPool,
    project_id: i32,
    payload: CreateMemberPayload,
) -> Result<Member, AppError> {
    // ADDED: Validate the incoming payload.
    validation::validate_payload(&payload)?;

    // This check is still valuable as it confirms the user exists in the DB.
    user_query::get_by_id(pool, payload.user_id).await?;

    member_query::create(pool, project_id, payload).await
}

/// Service function to list all members of a project.
pub async fn get_members_for_project(
    pool: &PgPool,
    project_id: i32,
) -> Result<Vec<Member>, AppError> {
    member_query::get_by_project_id(pool, project_id).await
}