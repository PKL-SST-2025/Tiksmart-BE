use crate::{errors::AppError, models::SeatMapInfo, service::seating_service, AppState};
use axum::{
    extract::{Path, State},
    Json,
};

/// Handler for the public to fetch the full seat map for an event.
/// GET /api/events/:event_id/seat-map
#[tracing::instrument(skip(app_state))]
pub async fn get_seat_map_for_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<Json<Vec<SeatMapInfo>>, AppError> {
    let seat_map = seating_service::get_seat_map_for_event(&app_state.db_pool, event_id).await?;
    Ok(Json(seat_map))
}