use crate::{
    db::venue_query,
    errors::AppError,
    models::{CreateVenuePayload, Venue},
    utils::validation,
};
use sqlx::PgPool;

/// Service to create a new venue.
pub async fn create(pool: &PgPool, payload: &CreateVenuePayload) -> Result<Venue, AppError> {
    validation::validate_payload(payload)?;
    venue_query::create(pool, payload).await
}

/// Service to fetch a venue by its ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Venue, AppError> {
    venue_query::get_by_id(pool, id).await
}

/// Service to list all active venues.
pub async fn list_all(pool: &PgPool) -> Result<Vec<Venue>, AppError> {
    venue_query::list_all(pool).await
}