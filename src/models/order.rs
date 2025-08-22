use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
    Refunded,
}

// Represents a row from the 'orders' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub user_id: i32,
    pub status: OrderStatus,
    pub subtotal: Decimal,
    pub service_fee: Decimal,
    pub total_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// Payload for creating a new order. This is the "checkout" action.
// The server will calculate fees and total amount.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrderPayload {
    // A list of items the user wants to purchase.
    #[validate(length(min = 1, message = "Order must contain at least one item."))]
    pub items: Vec<OrderItemPayload>,
}


// Represents one item in the checkout payload.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OrderItemPayload {
    // The specific offer the user is purchasing.
    pub offer_id: i32,

    // The ID of the specific seat, if this is for a reserved seating event.
    // This would be null for General Admission.
    pub seat_id: Option<i32>,

    // For GA events, the user specifies a quantity.
    // For reserved seating, quantity is implicitly 1 per seat_id.
    #[validate(range(min = 1, message = "Quantity must be at least 1."))]
    pub quantity: i32,
}

// A dedicated response struct for the order creation endpoint.
#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    // We can use #[serde(flatten)] if we want to merge the fields,
    // but nesting is often clearer.
    pub order: Order,
    pub stripe_client_secret: String,
}