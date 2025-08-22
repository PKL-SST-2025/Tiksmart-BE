use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus {
    Valid,
    CheckedIn,
    Voided,
    Resold,
}

// Represents a row from the 'tickets' table. This is the final, issued ticket.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: i32,
    pub event_id: i32,
    pub ticket_tier_id: i32,
    pub seat_id: Option<i32>, // Nullable for General Admission
    pub price_paid: Decimal,
    pub qr_code_data: String,
    pub status: TicketStatus,
    pub created_at: DateTime<Utc>,
    pub checked_in_at: Option<DateTime<Utc>>,
}

// A more detailed DTO for showing ticket details to a user.
// It joins data from multiple tables for a richer response.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TicketDetails {
    // From Ticket
    pub ticket_id: Uuid,
    pub qr_code_data: String,
    pub ticket_status: TicketStatus,

    // From Event
    pub event_title: String,
    pub event_start_time: DateTime<Utc>,

    // From Venue
    pub venue_name: String,
    pub venue_city: String,

    // From Ticket Tier
    pub ticket_tier_name: String,

    // From Seat (optional)
    pub section_name: Option<String>,
    pub row_name: Option<String>,
    pub seat_number: Option<String>,
}