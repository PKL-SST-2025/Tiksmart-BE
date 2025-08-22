use crate::{
    errors::AppError,
    models::{Genre, Segment, SubGenre},
};
use sqlx::{PgPool};

// --- Segment Queries ---

pub async fn create_segment(pool: &PgPool, name: &str) -> Result<Segment, AppError> {
    sqlx::query_as!(Segment, "INSERT INTO segments (name) VALUES ($1) RETURNING *", name)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
}

pub async fn list_segments(pool: &PgPool) -> Result<Vec<Segment>, AppError> {
    sqlx::query_as!(Segment, "SELECT * FROM segments ORDER BY name")
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}

// --- Genre Queries ---

pub async fn create_genre(pool: &PgPool, name: &str, segment_id: i32) -> Result<Genre, AppError> {
    sqlx::query_as!(
        Genre,
        "INSERT INTO genres (name, segment_id) VALUES ($1, $2) RETURNING *",
        name,
        segment_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

pub async fn list_genres_by_segment(
    pool: &PgPool,
    segment_id: i32,
) -> Result<Vec<Genre>, AppError> {
    sqlx::query_as!(
        Genre,
        "SELECT * FROM genres WHERE segment_id = $1 ORDER BY name",
        segment_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

// --- Sub-Genre Queries ---

pub async fn create_sub_genre(
    pool: &PgPool,
    name: &str,
    genre_id: i32,
) -> Result<SubGenre, AppError> {
    sqlx::query_as!(
        SubGenre,
        "INSERT INTO sub_genres (name, genre_id) VALUES ($1, $2) RETURNING *",
        name,
        genre_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

pub async fn list_sub_genres_by_genre(
    pool: &PgPool,
    genre_id: i32,
) -> Result<Vec<SubGenre>, AppError> {
    sqlx::query_as!(
        SubGenre,
        "SELECT * FROM sub_genres WHERE genre_id = $1 ORDER BY name",
        genre_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}