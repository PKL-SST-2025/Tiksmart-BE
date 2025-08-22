use crate::{
    errors::AppError,
    models::{AssignAttractionPayload, Attraction, AttractionType},
    service::attraction_service,
    AppState,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};

/// Handler for an organizer to add an attraction to their event's lineup.
/// POST /api/events/:event_id/attractions
#[tracing::instrument(skip(app_state, payload))]
pub async fn add_attraction_to_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
    Extension(organizer_id): Extension<i32>,
    Json(payload): Json<AssignAttractionPayload>,
) -> Result<StatusCode, AppError> {
    attraction_service::add_to_event(&app_state.db_pool, event_id, organizer_id, &payload).await?;
    Ok(StatusCode::CREATED)
}

/// Handler for an organizer to remove an attraction from their event's lineup.
/// DELETE /api/events/:event_id/attractions/:attraction_id
#[tracing::instrument(skip(app_state))]
pub async fn remove_attraction_from_event(
    State(app_state): State<AppState>,
    Path((event_id, attraction_id)): Path<(i32, i32)>,
    Extension(organizer_id): Extension<i32>,
) -> Result<StatusCode, AppError> {
    attraction_service::remove_from_event(&app_state.db_pool, event_id, attraction_id, organizer_id).await?;
    Ok(StatusCode::NO_CONTENT)
}