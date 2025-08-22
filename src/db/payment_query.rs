use crate::{errors::AppError, models::Payment};
use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/// Creates a new payment record linked to an order, starting in a 'pending' state.
/// This should be called right after creating the pending order.
pub async fn create_pending_payment(
    tx: &mut Transaction<'_, Postgres>,
    order_id: Uuid,
    amount: Decimal,
    stripe_payment_intent_id: &str,
) -> Result<Payment, AppError> {
    sqlx::query_as!(
        Payment,
        r#"
        INSERT INTO payments (order_id, amount_charged, stripe_payment_intent_id, status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id, order_id, status AS "status: _", amount_charged, currency, amount_refunded,
                  stripe_payment_intent_id, stripe_customer_id, payment_method_type,
                  created_at, last_updated
        "#,
        order_id,
        amount,
        stripe_payment_intent_id
    )
    .fetch_one(&mut **tx)
    .await
    .map_err(AppError::from)
}

/// Marks a payment as 'succeeded' upon successful confirmation from Stripe.
pub async fn mark_payment_succeeded(
    tx: &mut Transaction<'_, Postgres>,
    stripe_payment_intent_id: &str,
) -> Result<(), AppError> {
    let result = sqlx::query!(
        "UPDATE payments SET status = 'succeeded' WHERE stripe_payment_intent_id = $1 AND status = 'pending'",
        stripe_payment_intent_id
    )
    .execute(&mut **tx)
    .await?;

    if result.rows_affected() == 0 {
        // This could mean a duplicate webhook or a webhook for an already processed payment.
        // Log it, but don't treat it as a hard error unless business logic requires it.
        tracing::warn!("Attempted to mark a non-pending or non-existent payment as succeeded: {}", stripe_payment_intent_id);
    }
    Ok(())
}