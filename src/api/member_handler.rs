use crate::errors::AppError;
use crate::models::{CreateMemberPayload, Member};
use crate::service::{member_service, project_service};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sqlx::PgPool;

/// POST /api/projects/:project_id/members
/// Adds a new member to a project.
pub async fn add_member_to_project(
    Extension(requester_id): Extension<i32>, // The user making the request
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<CreateMemberPayload>,
) -> Result<Json<Member>, AppError> {
    // --- Authorization ---
    // For now, we'll just check if the requester is a member.
    // A real app would check if they are an admin or owner.
    project_service::authorize_user_for_project(&pool, requester_id, project_id).await?;

    let member = member_service::add_member_to_project(&pool, project_id, payload).await?;
    Ok(Json(member))
}

/// GET /api/projects/:project_id/members
/// Gets the list of all members for a project.
pub async fn get_project_members(
    Extension(user_id): Extension<i32>,
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> Result<Json<Vec<Member>>, AppError> {
    // --- Authorization ---
    project_service::authorize_user_for_project(&pool, user_id, project_id).await?;

    let members = member_service::get_members_for_project(&pool, project_id).await?;
    Ok(Json(members))
}