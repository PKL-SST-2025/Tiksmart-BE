use crate::{
    errors::AppError,
    models::{CreateOrderPayload, Order},
};
use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// Creates a new order in a 'pending' state and locks the associated seats/inventory.
/// This is the first step in the checkout process and MUST be executed within a transaction.
/// It calculates the total price based on the items provided.
pub async fn create_pending_order(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i32,
    payload: &CreateOrderPayload,
    order_expiry_minutes: i64,
) -> Result<Order, AppError> {
    // 1. Calculate totals and gather item details from the database
    let mut subtotal = Decimal::ZERO;
    let service_fee_per_ticket = Decimal::new(250, 2); // Example: $2.50 fee

    for item in &payload.items {
        let offer_price: (Decimal,) = sqlx::query_as("SELECT price FROM offers WHERE id = $1")
            .bind(item.offer_id)
            .fetch_one(&mut **tx)
            .await?;
        
        subtotal += offer_price.0 * Decimal::from(item.quantity);

        // 2. Lock the inventory
        if let Some(seat_id) = item.seat_id {
            // Lock a specific seat for reserved seating
            let result = sqlx::query!(
                "UPDATE event_seats SET status = 'locked'
                 WHERE event_id = (SELECT tt.event_id FROM ticket_tiers tt JOIN offers o ON tt.id = o.ticket_tier_id WHERE o.id = $1)
                 AND seat_id = $2 AND status = 'available'",
                item.offer_id,
                seat_id
            )
            .execute(&mut **tx)
            .await?;
            if result.rows_affected() == 0 {
                return Err(AppError::BadRequest(format!("Seat {} is no longer available.", seat_id)));
            }
        } else {
            // Decrement inventory for General Admission
            let result = sqlx::query!(
                "UPDATE offers SET quantity_sold = quantity_sold + $1
                 WHERE id = $2 AND (quantity_for_sale - quantity_sold) >= $1",
                item.quantity,
                item.offer_id
            )
            .execute(&mut **tx)
            .await?;
            if result.rows_affected() == 0 {
                return Err(AppError::BadRequest("Not enough tickets available for this offer.".to_string()));
            }
        }
    }
    
    let total_service_fee = service_fee_per_ticket * Decimal::from(payload.items.iter().map(|i| i.quantity as u64).sum::<u64>());
    let total_amount = subtotal + total_service_fee;
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(order_expiry_minutes);

    // 3. Create the 'pending' order record
    let order = sqlx::query_as!(
        Order,
        r#"
        INSERT INTO orders (user_id, subtotal, service_fee, total_amount, expires_at, status)
        VALUES ($1, $2, $3, $4, $5, 'pending')
        RETURNING id, user_id, status AS "status: _", subtotal, service_fee, total_amount, created_at, last_updated, expires_at
        "#,
        user_id,
        subtotal,
        total_service_fee,
        total_amount,
        expires_at
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(order)
}

/// Updates an order's status to 'completed'.
pub async fn mark_order_completed(
    tx: &mut Transaction<'_, Postgres>,
    order_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE orders SET status = 'completed' WHERE id = $1 AND status = 'pending'",
        order_id
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}