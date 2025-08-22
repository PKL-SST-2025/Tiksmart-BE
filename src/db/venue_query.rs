use crate::{
    errors::AppError,
    models::{CreateVenuePayload, Venue},
};
use sqlx::{PgPool, Executor, Postgres};

/// Creates a new venue.
pub async fn create<'e, E>(executor: E, payload: &CreateVenuePayload) -> Result<Venue, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        Venue,
        "INSERT INTO venues (name, city, postal_code, country, address_line_1)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING *",
        payload.name,
        payload.city,
        payload.postal_code,
        payload.country,
        payload.address_line_1,
    )
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

/// Fetches a venue by its ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Venue, AppError> {
    sqlx::query_as!(Venue, "SELECT * FROM venues WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
}

/// Lists all active venues.
pub async fn list_all(pool: &PgPool) -> Result<Vec<Venue>, AppError> {
    sqlx::query_as!(Venue, "SELECT * FROM venues WHERE is_active = true ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}