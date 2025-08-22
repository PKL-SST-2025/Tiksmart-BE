use crate::{
    db::event_query,
    errors::AppError,
    models::{CreateEventPayload, Event, UpdateEventPayload},
    utils::validation,
};
use sqlx::PgPool;

/// Service to create a new event.
/// The `organizer_id` is passed in from the authenticated user's token claims.
pub async fn create(
    pool: &PgPool,
    payload: &CreateEventPayload,
    organizer_id: i32,
) -> Result<Event, AppError> {
    // 1. Validate the incoming payload.
    validation::validate_payload(payload)?;

    // 2. Call the database query to create the event.
    event_query::create(pool, payload, organizer_id).await
}

/// Service to fetch a single event by its ID. (Simple pass-through)
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Event, AppError> {
    event_query::get_by_id(pool, id).await
}

/// Service to list all published, upcoming events. (Simple pass-through)
pub async fn list_published(pool: &PgPool) -> Result<Vec<Event>, AppError> {
    event_query::list_published(pool).await
}

/// Service to update an event.
/// Contains critical business logic for authorization.
pub async fn update(
    pool: &PgPool,
    event_id: i32,
    organizer_id: i32, // The ID of the user attempting the update.
    payload: &UpdateEventPayload,
) -> Result<Event, AppError> {
    // 1. Validate the payload.
    validation::validate_payload(payload)?;

    // 2. Authorization: Fetch the event first to ensure the user is the owner.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to modify this event.".to_string(),
        ));
    }

    // 3. If authorized, proceed with the update.
    event_query::update(pool, event_id, payload).await
}

/// Service to delete an event.
/// Also contains the same critical authorization logic.
pub async fn delete(pool: &PgPool, event_id: i32, organizer_id: i32) -> Result<(), AppError> {
    // 1. Authorization: Fetch the event to check ownership.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to delete this event.".to_string(),
        ));
    }

    // 2. If authorized, proceed with the deletion.
    event_query::delete(pool, event_id).await
}