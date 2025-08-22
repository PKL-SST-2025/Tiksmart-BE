use crate::{
    errors::AppError,
    models::{CreateCategoryPayload, Genre, Segment, SubGenre},
    service::category_service,
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

// --- Segment Handlers ---
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_segment(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateCategoryPayload>,
) -> Result<(StatusCode, Json<Segment>), AppError> {
    let segment = category_service::create_segment(&app_state.db_pool, &payload).await?;
    Ok((StatusCode::CREATED, Json(segment)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_segments(State(app_state): State<AppState>) -> Result<Json<Vec<Segment>>, AppError> {
    let segments = category_service::list_segments(&app_state.db_pool).await?;
    Ok(Json(segments))
}

// --- Genre Handlers ---
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_genre(
    State(app_state): State<AppState>,
    Path(segment_id): Path<i32>,
    Json(payload): Json<CreateCategoryPayload>,
) -> Result<(StatusCode, Json<Genre>), AppError> {
    let genre = category_service::create_genre(&app_state.db_pool, segment_id, &payload).await?;
    Ok((StatusCode::CREATED, Json(genre)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_genres_by_segment(
    State(app_state): State<AppState>,
    Path(segment_id): Path<i32>,
) -> Result<Json<Vec<Genre>>, AppError> {
    let genres = category_service::list_genres_by_segment(&app_state.db_pool, segment_id).await?;
    Ok(Json(genres))
}

// --- Sub-Genre Handlers ---
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_sub_genre(
    State(app_state): State<AppState>,
    Path(genre_id): Path<i32>,
    Json(payload): Json<CreateCategoryPayload>,
) -> Result<(StatusCode, Json<SubGenre>), AppError> {
    let sub_genre = category_service::create_sub_genre(&app_state.db_pool, genre_id, &payload).await?;
    Ok((StatusCode::CREATED, Json(sub_genre)))
}

#[tracing::instrument(skip(app_state))]
pub async fn list_sub_genres_by_genre(
    State(app_state): State<AppState>,
    Path(genre_id): Path<i32>,
) -> Result<Json<Vec<SubGenre>>, AppError> {
    let sub_genres = category_service::list_sub_genres_by_genre(&app_state.db_pool, genre_id).await?;
    Ok(Json(sub_genres))
}