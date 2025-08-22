use crate::{
    db::{attraction_query, event_query},
    errors::AppError,
    models::{AssignAttractionPayload, Attraction, AttractionType},
};
use sqlx::PgPool;

/// Service to create a new attraction.
pub async fn create(pool: &PgPool, name: &str, kind: AttractionType) -> Result<Attraction, AppError> {
    // Basic validation could be added here if needed (e.g., check name length)
    attraction_query::create(pool, name, kind).await
}

/// Service to add an attraction to an event's lineup.
pub async fn add_to_event(
    pool: &PgPool,
    event_id: i32,
    organizer_id: i32, // ID of the user making the request
    payload: &AssignAttractionPayload,
) -> Result<(), AppError> {
    // Authorization: Check if the user is the organizer of the event.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to modify this event's lineup.".to_string(),
        ));
    }

    attraction_query::add_to_event(pool, event_id, payload).await
}

/// Service to remove an attraction from an event's lineup.
pub async fn remove_from_event(
    pool: &PgPool,
    event_id: i32,
    attraction_id: i32,
    organizer_id: i32, // ID of the user making the request
) -> Result<(), AppError> {
    // Authorization: Check if the user is the organizer of the event.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to modify this event's lineup.".to_string(),
        ));
    }

    attraction_query::remove_from_event(pool, event_id, attraction_id).await
}