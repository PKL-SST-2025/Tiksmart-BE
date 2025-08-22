use crate::{errors::AppError, models::{order::OrderItemPayload, Ticket}};
use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// Creates all the tickets associated with a now-completed order.
/// This is the final step of a successful purchase and MUST be run in the same transaction
/// as `mark_order_completed` and `create_payment`.
pub async fn create_tickets_for_order(
    tx: &mut Transaction<'_, Postgres>,
    order_id: Uuid,
    user_id: i32,
    // We would pass the original `CreateOrderPayload` items here
    items: &Vec<OrderItemPayload>,
) -> Result<Vec<Ticket>, AppError> {
    let mut created_tickets = Vec::new();

    for item in items {
        // Fetch necessary IDs for ticket creation
        let ids: (i32, i32, Decimal) = sqlx::query_as(
            "SELECT tt.event_id, o.ticket_tier_id, o.price
             FROM offers o JOIN ticket_tiers tt ON o.ticket_tier_id = tt.id
             WHERE o.id = $1"
        )
        .bind(item.offer_id)
        .fetch_one(&mut **tx)
        .await?;

        let (event_id, ticket_tier_id, price_paid) = ids;

        // Create a ticket for each quantity
        for _ in 0..item.quantity {
             let ticket = sqlx::query_as!(
                Ticket,
                r#"
                INSERT INTO tickets (order_id, user_id, event_id, ticket_tier_id, seat_id, price_paid)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, order_id, user_id, event_id, ticket_tier_id, seat_id, price_paid,
                          qr_code_data, status AS "status: _", created_at, checked_in_at
                "#,
                order_id,
                user_id,
                event_id,
                ticket_tier_id,
                item.seat_id, // This will be the same for quantity=1, null for GA
                price_paid
            )
            .fetch_one(&mut **tx)
            .await?;

            created_tickets.push(ticket);
        }
    }

    Ok(created_tickets)
}