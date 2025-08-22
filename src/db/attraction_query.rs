use crate::{
    errors::AppError,
    models::{AssignAttractionPayload, Attraction, AttractionType},
};
use chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Postgres};

/// Creates a new attraction (e.g., a band, a speaker).
pub async fn create<'e, E>(
    executor: E,
    name: &str,
    kind: AttractionType,
) -> Result<Attraction, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        Attraction,
        r#"
        INSERT INTO attractions (name, "type") VALUES ($1, $2)
        RETURNING id, name, description, image_url, "type" AS "kind: _", created_at, last_updated
        "#,
        name,
        kind as _
    )
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

/// Fetches an attraction by its ID.
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Attraction, AppError> {
    sqlx::query_as!(
        Attraction,
        r#"SELECT id, name, description, image_url, "type" AS "kind: _", created_at, last_updated FROM attractions WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Assigns an attraction to a specific event in the join table.
pub async fn add_to_event(
    pool: &PgPool,
    event_id: i32,
    payload: &AssignAttractionPayload,
) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO event_attractions (event_id, attraction_id, performance_time, stage_name)
         VALUES ($1, $2, $3, $4)",
        event_id,
        payload.attraction_id,
        payload.performance_time,
        payload.stage_name
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Removes an attraction from an event.
pub async fn remove_from_event(pool: &PgPool, event_id: i32, attraction_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        "DELETE FROM event_attractions WHERE event_id = $1 AND attraction_id = $2",
        event_id,
        attraction_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(AppError::Sqlx(sqlx::Error::RowNotFound))
    } else {
        Ok(())
    }
}