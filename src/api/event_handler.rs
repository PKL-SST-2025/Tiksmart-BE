use crate::{
    errors::AppError,
    models::{CreateEventPayload, Event, UpdateEventPayload},
    service::event_service,
    AppState,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};

/// Handler to create a new event.
/// The `organizer_id` is extracted from the JWT token by the auth middleware.
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_event(
    State(app_state): State<AppState>,
    Extension(organizer_id): Extension<i32>, // Comes from auth_guard
    Json(payload): Json<CreateEventPayload>,
) -> Result<(StatusCode, Json<Event>), AppError> {
    let event = event_service::create(&app_state.db_pool, &payload, organizer_id).await?;
    Ok((StatusCode::CREATED, Json(event)))
}

/// Handler to get a single, publicly available event by its ID.
#[tracing::instrument(skip(app_state))]
pub async fn get_event_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Event>, AppError> {
    let event = event_service::get_by_id(&app_state.db_pool, id).await?;
    Ok(Json(event))
}

/// Handler to list all published, upcoming events.
#[tracing::instrument(skip(app_state))]
pub async fn list_published_events(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Event>>, AppError> {
    let events = event_service::list_published(&app_state.db_pool).await?;
    Ok(Json(events))
}

/// Handler for an organizer to update their own event.
#[tracing::instrument(skip(app_state, payload))]
pub async fn update_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
    Extension(organizer_id): Extension<i32>, // Authenticated user's ID
    Json(payload): Json<UpdateEventPayload>,
) -> Result<Json<Event>, AppError> {
    let updated_event =
        event_service::update(&app_state.db_pool, event_id, organizer_id, &payload).await?;
    Ok(Json(updated_event))
}

/// Handler for an organizer to delete their own event.
#[tracing::instrument(skip(app_state))]
pub async fn delete_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
    Extension(organizer_id): Extension<i32>, // Authenticated user's ID
) -> Result<StatusCode, AppError> {
    event_service::delete(&app_state.db_pool, event_id, organizer_id).await?;
    Ok(StatusCode::NO_CONTENT) // 204 No Content is standard for successful deletions
}