use crate::{
    db::{event_query, pricing_query},
    errors::AppError,
    models::{CreateOfferPayload, CreateTicketTierPayload, Offer, TicketTier},
    utils::validation,
};
use sqlx::PgPool;

// --- Ticket Tier Services ---

/// Service to create a new ticket tier for an event.
pub async fn create_ticket_tier(
    pool: &PgPool,
    event_id: i32,
    organizer_id: i32, // ID of the user making the request
    payload: &CreateTicketTierPayload,
) -> Result<TicketTier, AppError> {
    // 1. Validate the payload.
    validation::validate_payload(payload)?;

    // 2. Authorization: Check if the user is the organizer of the event.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to add tiers to this event.".to_string(),
        ));
    }

    // 3. Call the database query to create the tier.
    pricing_query::create_ticket_tier(pool, event_id, payload).await
}

/// Service to list all ticket tiers for a given event.
pub async fn list_tiers_for_event(pool: &PgPool, event_id: i32) -> Result<Vec<TicketTier>, AppError> {
    pricing_query::list_tiers_for_event(pool, event_id).await
}

// --- Offer Services ---

/// Service to create a new sales offer. This is a transactional operation.
pub async fn create_offer(
    pool: &PgPool,
    ticket_tier_id: i32,
    organizer_id: i32, // ID of the user making the request
    payload: &CreateOfferPayload,
) -> Result<Offer, AppError> {
    // 1. Validate the payload.
    validation::validate_payload(payload)?;

    // 2. Authorization: Check if the user is the organizer of the event that this tier belongs to.
    let event_organizer_id: (i32,) = sqlx::query_as(
        "SELECT e.organizer_id FROM events e JOIN ticket_tiers tt ON e.id = tt.event_id WHERE tt.id = $1"
    )
    .bind(ticket_tier_id)
    .fetch_one(pool)
    .await
    // Map RowNotFound to a more specific error, since it means the tier doesn't exist.
    .map_err(|_| AppError::BadRequest(format!("Ticket tier with ID {} not found.", ticket_tier_id)))?;

    if event_organizer_id.0 != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to add offers to this tier.".to_string(),
        ));
    }

    // 3. Begin a database transaction.
    let mut tx = pool.begin().await?;

    // 4. Call the transactional database query.
    let offer_result = pricing_query::create_offer(&mut tx, ticket_tier_id, payload).await;

    // 5. Commit or rollback the transaction based on the result.
    match offer_result {
        Ok(created_offer) => {
            tx.commit().await?; // The transaction is only saved if the query succeeds.
            Ok(created_offer)
        }
        Err(e) => {
            tx.rollback().await?; // If any part of the query fails, all changes are undone.
            Err(e)
        }
    }
}

/// Service to list all publicly available offers for an event.
pub async fn list_public_offers_for_event(pool: &PgPool, event_id: i32) -> Result<Vec<Offer>, AppError> {
    pricing_query::list_public_offers_for_event(pool, event_id).await
}