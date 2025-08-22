use crate::{
    db::{order_query, payment_query},
    errors::AppError,
    models::{CreateOrderPayload, Order},
    utils::validation,
};
use sqlx::PgPool;
  use num_traits::ToPrimitive;

// We would have a stripe_client module to interact with the Stripe API
// For now, we'll just define a placeholder struct.
pub struct StripeClient;
impl StripeClient {
    // This function would call the Stripe API to create a PaymentIntent
    pub async fn create_payment_intent(
        &self,
        amount: i64, // Stripe expects amount in cents
        currency: &str,
    ) -> Result<(String, String), AppError> {
        // In a real app:
        // let params = ...
        // let pi = stripe::PaymentIntent::create(&self.client, params).await?;
        // Ok((pi.id, pi.client_secret.unwrap_or_default()))

        // Placeholder for compilation:
        let placeholder_id = format!("pi_{}", uuid::Uuid::new_v4().simple());
        let placeholder_secret = format!("pi_{}_secret_{}", uuid::Uuid::new_v4().simple(), uuid::Uuid::new_v4().simple());
        Ok((placeholder_id, placeholder_secret))
    }
}

/// The primary service function for starting a checkout process.
/// It creates a pending order, locks inventory, and generates a payment intent.
/// This is a transactional operation.
pub async fn create_order(
    pool: &PgPool,    
    stripe_client: &StripeClient,
    user_id: i32,
    payload: &CreateOrderPayload,
) -> Result<(Order, String), AppError> {
    // 1. Validate the incoming payload.
    validation::validate_payload(payload)?;

    // 2. Begin a database transaction.
    let mut tx = pool.begin().await?;

    // 3. Define business rules
    const ORDER_EXPIRY_MINUTES: i64 = 15;

    // 4. Call the transactional query to create the pending order and lock inventory.
    let order_result =
        order_query::create_pending_order(&mut tx, user_id, payload, ORDER_EXPIRY_MINUTES).await;

    let order = match order_result {
        Ok(order) => order,
        Err(e) => {
            tx.rollback().await?; // Rollback on failure
            return Err(e);
        }
    };

    // 5. Create a Payment Intent with Stripe.
    // This happens *after* the DB lock but *before* the commit. If Stripe fails, we can still rollback.
    let amount_in_cents = (order.total_amount * rust_decimal::Decimal::from(100)).to_i64()
        .ok_or_else(|| AppError::InternalServerError("Failed to convert amount to cents".to_string()))?;
    let payment_intent_result = stripe_client
        .create_payment_intent(amount_in_cents, "usd")
        .await;

    let (pi_id, client_secret) = match payment_intent_result {
        Ok(pi) => pi,
        Err(e) => {
            tx.rollback().await?;
            return Err(e);
        }
    };

    // 6. Create the pending payment record in our DB, linking our order to the Stripe PI.
    let payment_result =
        payment_query::create_pending_payment(&mut tx, order.id, order.total_amount, &pi_id).await;

    if let Err(e) = payment_result {
        tx.rollback().await?;
        return Err(e);
    }

    // 7. If everything has succeeded, commit the transaction.
    tx.commit().await?;

    // 8. Return the created order and the client secret for the frontend to use.
    Ok((order, client_secret))
}