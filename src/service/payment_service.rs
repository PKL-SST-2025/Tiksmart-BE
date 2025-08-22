use crate::{
    db::{order_query, payment_query, ticket_query},
    errors::AppError,
    models::{order::OrderItemPayload, Ticket}, // Assuming a way to get order items
};
use sqlx::PgPool;
use uuid::Uuid;

/// This is the most critical transaction in the application.
/// It's triggered by a Stripe webhook when a payment succeeds.
/// It finalizes the order and issues the tickets.
pub async fn finalize_order_on_payment_success(
    pool: &PgPool,
    stripe_payment_intent_id: &str,
) -> Result<Vec<Ticket>, AppError> {
    // 1. Begin a database transaction.
    let mut tx = pool.begin().await?;

    // 2. Mark the payment as succeeded in our DB.
    payment_query::mark_payment_succeeded(&mut tx, stripe_payment_intent_id).await?;

    // 3. Fetch the order details needed for finalization.
    // In a real app, you'd fetch the order and its items. Here we simplify.
    let order_info: (Uuid, i32) = sqlx::query_as(
        "SELECT o.id, o.user_id FROM orders o JOIN payments p ON o.id = p.order_id WHERE p.stripe_payment_intent_id = $1"
    )
    .bind(stripe_payment_intent_id)
    .fetch_one(&mut *tx)
    .await?;
    
    let (order_id, user_id) = order_info;

    // 4. Mark the order itself as 'completed'.
    order_query::mark_order_completed(&mut tx, order_id).await?;

    // 5. This is where you would fetch the original order items to create tickets.
    // For this example, we'll assume a placeholder. In a real app, you'd query them.
    let placeholder_items: Vec<OrderItemPayload> = vec![]; // This needs to be populated from the DB

    // 6. Create the actual tickets.
    let tickets = ticket_query::create_tickets_for_order(&mut tx, order_id, user_id, &placeholder_items).await?;

    // 7. Commit the transaction. If any step failed, the rollback is handled by `?`.
    tx.commit().await?;
    
    // Optional: Send a confirmation email to the user.

    Ok(tickets)
}