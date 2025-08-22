use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Represents a row from the 'venues' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Venue {
    pub id: i32,
    pub name: String,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub capacity: Option<i32>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub phone_number: Option<String>,
    pub website_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// Payload for creating a new venue.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateVenuePayload {
    #[validate(length(min = 2, message = "Venue name must be at least 2 characters."))]
    pub name: String,
    #[validate(length(min = 2, message = "City name must be at least 2 characters."))]
    pub city: String,
    #[validate(length(min = 2, message = "Postal code must be at least 2 characters."))]
    pub postal_code: String,
    #[validate(length(min = 2, message = "Country name must be at least 2 characters."))]
    pub country: String,
    pub address_line_1: Option<String>,
    // ... add other optional fields as needed
}