use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Succeeded,
    Failed,
    Refunded,
}

// Represents a row from the 'payments' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: PaymentStatus,
    pub amount_charged: Decimal,
    pub currency: String,
    pub amount_refunded: Decimal,
    #[serde(skip)]
    pub stripe_payment_intent_id: String,
    #[serde(skip)]
    pub stripe_customer_id: Option<String>,
    pub payment_method_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}