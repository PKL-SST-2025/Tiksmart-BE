use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "seat_status", rename_all = "snake_case")]
pub enum SeatStatus {
    Available,
    Locked,
    Sold,
    Unavailable,
}

// Represents a row from the 'seating_charts' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SeatingChart {
    pub id: i32,
    pub venue_id: i32,
    pub name: String,
    pub background_image_url: Option<String>,
}

// Represents a row from the 'sections' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Section {
    pub id: i32,
    pub seating_chart_id: i32,
    pub name: String,
}

// Represents a row from the 'rows' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Row {
    pub id: i32,
    pub section_id: i32,
    pub name: String,
}

// Represents a physical seat from the 'seats' table (the template).
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Seat {
    pub id: i32,
    pub row_id: i32,
    pub seat_number: String,
    pub pos_x: Decimal,
    pub pos_y: Decimal,
    pub width: Decimal,
    pub height: Decimal,
}

// Represents the dynamic status of a seat for a specific event.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EventSeat {
    pub event_id: i32,
    pub seat_id: i32,
    pub ticket_tier_id: i32,
    pub status: SeatStatus,
    #[serde(skip)]
    pub lock_expires_at: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,
}

// A combined structure for rendering the seat map on the frontend.
// This is a DTO, not a direct DB model.
#[derive(Debug, Serialize)]
pub struct SeatMapInfo {
    // Info from the Seat (template)
    pub seat_id: i32,
    pub seat_number: String,
    pub pos_x: Decimal,
    pub pos_y: Decimal,
    
    // Info from the EventSeat (dynamic status)
    pub status: SeatStatus,
    
    // Info from the TicketTier (price)
    pub ticket_tier_id: i32,
    pub ticket_tier_name: String,
    pub price: Decimal,
}