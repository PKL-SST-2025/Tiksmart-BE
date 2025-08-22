use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Our Rust enum mapping to the 'event_status' PG ENUM.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "event_status", rename_all = "snake_case")]
pub enum EventStatus {
    Draft,
    Published,
    Cancelled,
    Completed,
    OnSale,
    SoldOut,
}

// Represents a row from the 'events' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub organizer_id: i32,
    pub venue_id: Option<i32>,
    pub segment_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub sub_genre_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: EventStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub price_min: Option<Decimal>,
    pub price_max: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// Payload for creating a new event.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateEventPayload {
    #[validate(length(min = 3, message = "Title must be at least 3 characters long."))]
    pub title: String,
    pub description: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub venue_id: Option<i32>,
    pub segment_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub sub_genre_id: Option<i32>,
}

// Payload for updating an existing event (all fields are optional).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEventPayload {
    #[validate(length(min = 3, message = "Title must be at least 3 characters long."))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<EventStatus>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub venue_id: Option<i32>,
    // ... add any other fields you want to be updatable
}