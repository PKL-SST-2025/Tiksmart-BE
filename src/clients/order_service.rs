// File: src/service/order_service.rs

// Remove the placeholder StripeClient struct.
// use crate::service::order_service::{self, StripeClient}; // DELETE THIS

use stripe::{
    CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods, Currency, PaymentIntent,
}; // <-- Add stripe imports

// Update the function signature to accept the real client from AppState
pub async fn create_order(
    pool: &PgPool,
    stripe_client: &stripe::Client, // <-- Use the real client type
    user_id: i32,
    payload: &CreateOrderPayload,
) -> Result<(Order, String), AppError> {
    // ... (validation, begin transaction, create pending order)

    // 5. Create a real Payment Intent with Stripe.
    let amount_in_cents = (order.total_amount * rust_decimal::Decimal::from(100)).to_i64()
        .ok_or_else(|| AppError::InternalServerError("Failed to convert amount to cents".to_string()))?;

    let mut params = CreatePaymentIntent::new(amount_in_cents, Currency::USD);
    params.automatic_payment_methods = Some(CreatePaymentIntentAutomaticPaymentMethods { enabled: true });

    let pi = PaymentIntent::create(stripe_client, params).await?;
    let client_secret = pi.client_secret.unwrap_or_default(); // The frontend needs this

    // 6. Create the pending payment record in our DB, linking our order to the Stripe PI.
    let payment_result =
        payment_query::create_pending_payment(&mut tx, order.id, order.total_amount, &pi.id.to_string()).await;
        
    // ... (rest of the function: commit, return result)
    
    // 8. Return the created order and the client secret.
    Ok((order, client_secret))
}