// File: src/api/payment_handler.rs

use crate::config::CONFIG;
use crate::errors::AppError;
use crate::service::payment_service;
use crate::AppState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    body::Bytes, // We will take Bytes and convert to &str
};

// Use the correct imports for the synchronous `stripe` crate.
use stripe::{EventObject, EventType, Webhook};

/// Handler for incoming Stripe webhooks.
/// This endpoint is NOT protected by auth or CSRF guards. Stripe authenticates
/// with a signature header.
/// POST /api/webhooks/stripe
#[tracing::instrument(skip(app_state, body))]
pub async fn stripe_webhook_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    body: Bytes, // Take the raw body as Bytes
) -> Result<StatusCode, AppError> {
    // 1. Get the signature from the headers.
    let signature = headers
        .get("stripe-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::BadRequest("Missing Stripe signature header.".to_string()))?;

    // Convert the Bytes body to a &str, which is what the `stripe` crate expects.
    let body_str = std::str::from_utf8(&body)
        .map_err(|_| AppError::BadRequest("Invalid UTF-8 in webhook body.".to_string()))?;

    // 2. Verify the webhook signature. This is a SYNCHRONOUS call. NO .await
    let event = Webhook::construct_event(body_str, signature, &CONFIG.stripe_webhook_secret)
        .map_err(|e| {
            tracing::error!("Stripe webhook signature verification failed: {:?}", e);
            AppError::BadRequest("Invalid Stripe webhook signature or body.".to_string())
        })?;

    // 3. Handle the event based on its type. Use `event.type_`
    match event.type_ {
        EventType::PaymentIntentSucceeded => {
            // Pattern match on `event.data.object` to get the PaymentIntent object.
            // This pattern is correct for the `stripe` crate as well.
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                tracing::info!("Received payment_intent.succeeded for {}", payment_intent.id);
                // The service call is still async.
                payment_service::finalize_order_on_payment_success(
                    &app_state.db_pool,
                    &payment_intent.id.to_string(),
                )
                .await?;
            } else {
                tracing::warn!(
                    "payment_intent.succeeded event received, but object was not a PaymentIntent: {:?}",
                    event.data.object
                );
                return Err(AppError::BadRequest(
                    "Webhook data object type mismatch.".to_string(),
                ));
            }
        }
        other_event_type => {
            tracing::info!("Received unhandled Stripe event type: {:?}", other_event_type);
        }
    }

    // 4. Return a 200 OK to Stripe to acknowledge receipt.
    Ok(StatusCode::OK)
}