use crate::{
    errors::AppError,
    models::{CreateVenuePayload, Venue},
    service::venue_service,
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// Handler to create a new venue.
/// This would likely be an admin-only endpoint.
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_venue(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateVenuePayload>,
) -> Result<(StatusCode, Json<Venue>), AppError> {
    let venue = venue_service::create(&app_state.db_pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(venue)))
}

/// Handler to get a single venue by its ID.
#[tracing::instrument(skip(app_state))]
pub async fn get_venue_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Venue>, AppError> {
    let venue = venue_service::get_by_id(&app_state.db_pool, id).await?;
    Ok(Json(venue))
}

/// Handler to list all active venues.
#[tracing::instrument(skip(app_state))]
pub async fn list_venues(State(app_state): State<AppState>) -> Result<Json<Vec<Venue>>, AppError> {
    let venues = venue_service::list_all(&app_state.db_pool).await?;
    Ok(Json(venues))
}