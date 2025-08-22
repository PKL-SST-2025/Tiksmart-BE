use crate::{
    errors::AppError,
    models::{CreateOfferPayload, CreateTicketTierPayload, Offer, TicketTier},
    service::pricing_service,
    AppState,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    Json,
};

// --- Ticket Tier Handlers ---

/// Handler for an organizer to create a ticket tier for their event.
/// POST /api/events/:event_id/tiers
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_ticket_tier(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
    Extension(organizer_id): Extension<i32>,
    Json(payload): Json<CreateTicketTierPayload>,
) -> Result<(StatusCode, Json<TicketTier>), AppError> {
    let tier = pricing_service::create_ticket_tier(&app_state.db_pool, event_id, organizer_id, &payload).await?;
    Ok((StatusCode::CREATED, Json(tier)))
}

/// Handler to list all tiers for an event.
/// GET /api/events/:event_id/tiers
#[tracing::instrument(skip(app_state))]
pub async fn list_tiers_for_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<Json<Vec<TicketTier>>, AppError> {
    let tiers = pricing_service::list_tiers_for_event(&app_state.db_pool, event_id).await?;
    Ok(Json(tiers))
}

// --- Offer Handlers ---

/// Handler for an organizer to create a sales offer for a ticket tier.
/// POST /api/tiers/:tier_id/offers
#[tracing::instrument(skip(app_state, payload))]
pub async fn create_offer(
    State(app_state): State<AppState>,
    Path(tier_id): Path<i32>,
    Extension(organizer_id): Extension<i32>,
    Json(payload): Json<CreateOfferPayload>,
) -> Result<(StatusCode, Json<Offer>), AppError> {
    let offer = pricing_service::create_offer(&app_state.db_pool, tier_id, organizer_id, &payload).await?;
    Ok((StatusCode::CREATED, Json(offer)))
}

/// Handler to list all publicly available offers for an event.
/// This is what customers will see when they view an event page.
/// GET /api/events/:event_id/offers
#[tracing::instrument(skip(app_state))]
pub async fn list_public_offers_for_event(
    State(app_state): State<AppState>,
    Path(event_id): Path<i32>,
) -> Result<Json<Vec<Offer>>, AppError> {
    let offers = pricing_service::list_public_offers_for_event(&app_state.db_pool, event_id).await?;
    Ok(Json(offers))
}