use crate::{
    errors::AppError,
    models::{CreateEventPayload, Event, EventStatus, UpdateEventPayload},
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres};

/// Creates a new event for a given organizer.
pub async fn create<'e, E>(
    executor: E,
    payload: &CreateEventPayload,
    organizer_id: i32,
) -> Result<Event, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        Event,
        r#"
        INSERT INTO events 
            (title, description, start_time, end_time, venue_id, segment_id, genre_id, sub_genre_id, organizer_id)
        VALUES 
            ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING 
            id, organizer_id, venue_id, segment_id, genre_id, sub_genre_id, title, 
            description, status AS "status: _", start_time, end_time, price_min, price_max, 
            created_at, last_updated
        "#,
        payload.title,
        payload.description,
        payload.start_time,
        payload.end_time,
        payload.venue_id,
        payload.segment_id,
        payload.genre_id,
        payload.sub_genre_id,
        organizer_id
    )
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

/// Fetches a single event by its ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Event, AppError> {
    sqlx::query_as!(
        Event,
        r#"
        SELECT 
            id, organizer_id, venue_id, segment_id, genre_id, sub_genre_id, title, 
            description, status AS "status: _", start_time, end_time, price_min, price_max, 
            created_at, last_updated
        FROM events WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Lists all events that are currently published and upcoming.
pub async fn list_published(pool: &PgPool) -> Result<Vec<Event>, AppError> {
    sqlx::query_as!(
        Event,
        r#"
        SELECT 
            id, organizer_id, venue_id, segment_id, genre_id, sub_genre_id, title, 
            description, status AS "status: _", start_time, end_time, price_min, price_max, 
            created_at, last_updated
        FROM events 
        WHERE status = 'published' AND start_time > NOW()
        ORDER BY start_time ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

/// Updates an event's details. Uses COALESCE to only update non-None fields.
pub async fn update(pool: &PgPool, id: i32, payload: &UpdateEventPayload) -> Result<Event, AppError> {
    sqlx::query_as!(
        Event,
        r#"
        UPDATE events
        SET 
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            start_time = COALESCE($4, start_time),
            end_time = COALESCE($5, end_time),
            last_updated = NOW()
        WHERE id = $6
        RETURNING 
            id, organizer_id, venue_id, segment_id, genre_id, sub_genre_id, title, 
            description, status AS "status: _", start_time, end_time, price_min, price_max, 
            created_at, last_updated
        "#,
        payload.title,
        payload.description,
        payload.status as _,
        payload.start_time,
        payload.end_time,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Deletes an event by its ID. Returns a `RowNotFound` error if the ID does not exist.
pub async fn delete(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM events WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(AppError::Sqlx(sqlx::Error::RowNotFound))
    } else {
        Ok(())
    }
}