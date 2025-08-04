use crate::db::{member_query, project_query};
use crate::errors::AppError;
use crate::models::{CreateMemberPayload, CreateProjectPayload, Project};
use sqlx::PgPool;
use crate::utils::validation;
/// The service function for creating a new project.
///
/// Business Logic:
/// 1. Creates the project with the specified owner.
/// 2. Automatically adds the owner as the first member of the project.

pub async fn create_project(
    pool: &PgPool,
    owner_user_id: i32,
    owner_full_name: String,
    payload: CreateProjectPayload,
) -> Result<Project, AppError> {
    // This validation call is now correct because we will fix the model in the next step.
    validation::validate_payload(&payload)?;

    let mut tx = pool.begin().await?;

    // THE FIX: Use `&mut *tx` to pass the underlying connection to the query functions.
    let project = project_query::create(&mut *tx, owner_user_id, payload).await?;

    let member_payload = CreateMemberPayload {
        user_id: owner_user_id,
        role_id: None,
        full_name: owner_full_name,
    };

    // THE FIX: Apply the same change here.
    member_query::create(&mut *tx, project.id, member_payload).await?;

    tx.commit().await?;

    Ok(project)
}

/// This is the new, critical authorization function.
/// It checks if a user is a member of a specific project.
/// If they are not, it returns a `Forbidden` error. This will be used everywhere.
pub async fn authorize_user_for_project(
    pool: &PgPool,
    user_id: i32,
    project_id: i32,
) -> Result<(), AppError> {
    match member_query::get_by_user_and_project(pool, user_id, project_id).await {
        Ok(_) => Ok(()), // User is a member, authorization passed.
        Err(AppError::Sqlx(sqlx::Error::RowNotFound)) => {
            // User is NOT a member, return a clear forbidden error.
            Err(AppError::Forbidden(
                "You do not have permission to access this project.".to_string(),
            ))
        }
        Err(e) => Err(e), // Some other database error occurred.
    }
}

/// Service function to get a single project by its ID.
/// In a real app, this should also check if the user requesting it has access.
pub async fn get_project_by_id(pool: &PgPool, project_id: i32) -> Result<Project, AppError> {
    project_query::get_by_id(pool, project_id).await
}

/// Service function to get all projects a user is a member of.
/// This is more complex as it requires joining tables, a good candidate for a dedicated query.
/// (For now, we'll just get projects they own as an example).
pub async fn get_projects_for_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<Project>, AppError> {
    project_query::get_all_by_owner(pool, user_id).await
}

