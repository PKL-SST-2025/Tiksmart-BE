use crate::{
    db::category_query,
    errors::AppError,
    models::{CreateCategoryPayload, Genre, Segment, SubGenre},
    utils::validation,
};
use sqlx::PgPool;

// --- Segment Services ---

/// Service to create a new top-level segment.
pub async fn create_segment(pool: &PgPool, payload: &CreateCategoryPayload) -> Result<Segment, AppError> {
    validation::validate_payload(payload)?;
    category_query::create_segment(pool, &payload.name).await
}

/// Service to list all available segments.
pub async fn list_segments(pool: &PgPool) -> Result<Vec<Segment>, AppError> {
    category_query::list_segments(pool).await
}

// --- Genre Services ---

/// Service to create a new genre within a specific segment.
pub async fn create_genre(
    pool: &PgPool,
    segment_id: i32,
    payload: &CreateCategoryPayload,
) -> Result<Genre, AppError> {
    validation::validate_payload(payload)?;
    // The foreign key constraint in the DB will return an error if segment_id is invalid,
    // which is caught and converted to an AppError automatically.
    category_query::create_genre(pool, &payload.name, segment_id).await
}

/// Service to list all genres belonging to a segment.
pub async fn list_genres_by_segment(pool: &PgPool, segment_id: i32) -> Result<Vec<Genre>, AppError> {
    category_query::list_genres_by_segment(pool, segment_id).await
}

// --- Sub-Genre Services ---

/// Service to create a new sub-genre within a specific genre.
pub async fn create_sub_genre(
    pool: &PgPool,
    genre_id: i32,
    payload: &CreateCategoryPayload,
) -> Result<SubGenre, AppError> {
    validation::validate_payload(payload)?;
    category_query::create_sub_genre(pool, &payload.name, genre_id).await
}

/// Service to list all sub-genres belonging to a genre.
pub async fn list_sub_genres_by_genre(pool: &PgPool, genre_id: i32) -> Result<Vec<SubGenre>, AppError> {
    category_query::list_sub_genres_by_genre(pool, genre_id).await
}