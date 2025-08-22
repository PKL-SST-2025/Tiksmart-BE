use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::utils::validation;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "offer_status", rename_all = "snake_case")]
pub enum OfferStatus {
    Scheduled,
    OnSale,
    Paused,
    SoldOut,
    Ended,
}

// Represents a row from the 'ticket_tiers' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TicketTier {
    pub id: i32,
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub total_inventory: i32,
}

// Represents a row from the 'offers' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Offer {
    pub id: i32,
    pub ticket_tier_id: i32,
    pub name: String,
    pub status: OfferStatus,
    pub price: Decimal,
    pub quantity_for_sale: i32,
    pub quantity_sold: i32,
    pub sale_start_time: Option<DateTime<Utc>>,
    pub sale_end_time: Option<DateTime<Utc>>,
    pub min_per_order: i32,
    pub max_per_order: i32,
    #[serde(skip)] // Hide the access code from public view
    pub access_code: Option<String>,
}

// Payload for creating a new ticket tier for an event.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTicketTierPayload {
    #[validate(length(min = 3, message = "Tier name must be at least 3 characters."))]
    pub name: String,
    pub description: Option<String>,
    #[validate(range(min = 1, message = "Inventory must be at least 1."))]
    pub total_inventory: i32,
}

// Payload for creating a new offer for a ticket tier.
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateOfferPayload {
    #[validate(length(min = 3, message = "Offer name must be at least 3 characters."))]
    pub name: String,
    
    // Use our custom validator for the Decimal type
    #[validate(custom(function = "validation::is_non_negative_decimal"))]
    pub price: Decimal,
    
    #[validate(range(min = 1, message = "Quantity must be at least 1."))]
    pub quantity_for_sale: i32,
    
    pub sale_start_time: Option<DateTime<Utc>>,
    pub sale_end_time: Option<DateTime<Utc>>,
    pub access_code: Option<String>,
}