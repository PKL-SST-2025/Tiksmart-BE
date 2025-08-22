use serde::{Deserialize, Serialize};
use validator::Validate;

// Represents a row from the 'segments' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Segment {
    pub id: i32,
    pub name: String,
}

// Represents a row from the 'genres' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub segment_id: i32,
}

// Represents a row from the 'sub_genres' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubGenre {
    pub id: i32,
    pub name: String,
    pub genre_id: i32,
}

// A combined structure for displaying the full category hierarchy.
// This is not a direct DB model, but a useful DTO for API responses.
#[derive(Debug, Serialize)]
pub struct FullCategory {
    pub segment_id: i32,
    pub segment_name: String,
    pub genre_id: i32,
    pub genre_name: String,
    pub sub_genre_id: i32,
    pub sub_genre_name: String,
}

// Payload for creating any new category. Reusable for all three.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryPayload {
    #[validate(length(min = 2, message = "Name must be at least 2 characters."))]
    pub name: String,
    // The parent ID will be specified in the URL, e.g., POST /segments/{id}/genres
}