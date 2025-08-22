use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "attraction_type", rename_all = "snake_case")]
pub enum AttractionType {
    Music,
    Speaker,
    Comedy,
    SpecialGuest,
}

// Represents a row from the 'attractions' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Attraction {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    #[sqlx(rename = "type")] // Map 'type' column to 'kind' field
    pub kind: AttractionType,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// Payload for assigning an attraction to an event.
#[derive(Debug, Deserialize)]
pub struct AssignAttractionPayload {
    pub attraction_id: i32,
    pub performance_time: Option<DateTime<Utc>>,
    pub stage_name: Option<String>,
}