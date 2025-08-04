use crate::errors::AppError;
use crate::models::{CreateRolePayload, Role};
use crate::service::{project_service, role_service};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sqlx::PgPool;

/// POST /api/projects/:project_id/roles
pub async fn create_role_for_project(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateRolePayload>,
) -> Result<Json<Role>, AppError> {
    // --- Authorization ---
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let role = role_service::create_role_for_project(&pool, project_id, payload).await?;
    Ok(Json(role))
}

/// GET /api/projects/:project_id/roles
pub async fn get_roles_for_project(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> Result<Json<Vec<Role>>, AppError> {
    // --- Authorization ---
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let roles = role_service::get_roles_for_project(&pool, project_id).await?;
    Ok(Json(roles))
}