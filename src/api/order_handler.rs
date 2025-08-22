use crate::{
    errors::AppError,
    models::{CreateOrderPayload},
    service::order_service::{self, StripeClient}, // Assuming StripeClient is in order_service
    AppState,
};
use crate::models::order::CreateOrderResponse;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    Json,
};

/// Handler to initiate the checkout process.
/// Creates a pending order, locks inventory, and returns a Stripe client secret.
/// POST /api/orders
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_order(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<CreateOrderPayload>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), AppError> {
    // In a real app, the Stripe client would be part of AppState.
    // let stripe_client = &app_state.stripe_client;
    let stripe_client = StripeClient; // Using the placeholder for now

    let (order, stripe_client_secret) =
        order_service::create_order(&app_state.db_pool, &stripe_client, user_id, &payload)
            .await?;

    let response = CreateOrderResponse {
        order,
        stripe_client_secret,
    };

    Ok((StatusCode::CREATED, Json(response)))
}