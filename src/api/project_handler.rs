use crate::db::user_query;
use crate::errors::AppError;
use crate::models::{CreateProjectPayload, Project};
use crate::service::project_service;
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sqlx::PgPool;

/// POST /api/projects
/// Creates a new project, with the logged-in user as the owner.
pub async fn create_project(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<Json<Project>, AppError> {
    // We need the user's name to create their initial member entry.
    // This is a good example of a handler enriching data before calling a service.
    let user = user_query::get_by_id(&pool, user_id).await?;

    let project = project_service::create_project(&pool, user_id, user.email, payload).await?;

    Ok(Json(project))
}

/// GET /api/projects/:project_id
/// Fetches the details of a single project.
pub async fn get_project_by_id(
    Extension(user_id): Extension<i32>, // Auth check
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> Result<Json<Project>, AppError> {
    // --- Authorization ---
    // A user can only see a project if they are a member of it.
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let project = project_service::get_project_by_id(&pool, project_id).await?;
    Ok(Json(project))
}

/// GET /api/projects
/// Fetches all projects the logged-in user is a member of.
pub async fn get_my_projects(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Project>>, AppError> {
    let projects = project_service::get_projects_for_user(&pool, user_id).await?;
    Ok(Json(projects))
}