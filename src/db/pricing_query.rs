use crate::{
    errors::AppError,
    models::{CreateOfferPayload, CreateTicketTierPayload, Offer, TicketTier},
};
use rust_decimal::Decimal;
use sqlx::{Executor, PgPool, Postgres, Transaction};

// --- Ticket Tier Queries ---

/// Creates a new ticket tier for a specific event.
pub async fn create_ticket_tier<'e, E>(
    executor: E,
    event_id: i32,
    payload: &CreateTicketTierPayload,
) -> Result<TicketTier, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as!(
        TicketTier,
        "INSERT INTO ticket_tiers (event_id, name, description, total_inventory)
         VALUES ($1, $2, $3, $4) RETURNING *",
        event_id,
        payload.name,
        payload.description,
        payload.total_inventory
    )
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

/// Lists all ticket tiers for a given event.
pub async fn list_tiers_for_event(
    pool: &PgPool,
    event_id: i32,
) -> Result<Vec<TicketTier>, AppError> {
    sqlx::query_as!(
        TicketTier,
        "SELECT * FROM ticket_tiers WHERE event_id = $1 ORDER BY name",
        event_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

// --- Offer Queries ---

/// Creates a new sales offer for a specific ticket tier.
/// This function MUST be called within a transaction because it also updates the event's price range.
pub async fn create_offer(
    tx: &mut Transaction<'_, Postgres>,
    ticket_tier_id: i32,
    payload: &CreateOfferPayload,
) -> Result<Offer, AppError> {
    // Step 1: Insert the new offer
    let offer = sqlx::query_as!(
        Offer,
        r#"
        INSERT INTO offers (ticket_tier_id, name, price, quantity_for_sale, sale_start_time, sale_end_time, access_code)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, ticket_tier_id, name, status AS "status: _", price, quantity_for_sale, quantity_sold,
                  sale_start_time, sale_end_time, min_per_order, max_per_order, access_code
        "#,
        ticket_tier_id,
        payload.name,
        payload.price,
        payload.quantity_for_sale,
        payload.sale_start_time,
        payload.sale_end_time,
        payload.access_code
    )
    .fetch_one(&mut **tx)
    .await?;

    // Step 2: Update the denormalized price range on the parent event
    // This is a classic use case for a transaction to ensure data consistency.
    sqlx::query!(
        r#"
        WITH event_price_range AS (
            SELECT
                tt.event_id,
                MIN(o.price) as min_price,
                MAX(o.price) as max_price
            FROM offers o
            JOIN ticket_tiers tt ON o.ticket_tier_id = tt.id
            WHERE tt.event_id = (SELECT event_id FROM ticket_tiers WHERE id = $1)
            GROUP BY tt.event_id
        )
        UPDATE events e
        SET
            price_min = epr.min_price,
            price_max = epr.max_price
        FROM event_price_range epr
        WHERE e.id = epr.event_id;
        "#,
        ticket_tier_id,
    )
    .execute(&mut **tx)
    .await?;

    Ok(offer)
}

/// Lists all publicly visible and currently on-sale offers for a given event.
/// This is the query a customer would use to see available tickets.
pub async fn list_public_offers_for_event(
    pool: &PgPool,
    event_id: i32,
) -> Result<Vec<Offer>, AppError> {
    sqlx::query_as!(
        Offer,
        r#"
        SELECT
            o.id, o.ticket_tier_id, o.name, o.status AS "status: _", o.price, o.quantity_for_sale, 
            o.quantity_sold, o.sale_start_time, o.sale_end_time, o.min_per_order, 
            o.max_per_order, o.access_code
        FROM offers o
        JOIN ticket_tiers tt ON o.ticket_tier_id = tt.id
        WHERE
            tt.event_id = $1
            AND o.status = 'on_sale'
            AND o.access_code IS NULL -- Exclude presale offers
            AND (o.sale_start_time IS NULL OR o.sale_start_time <= NOW())
            AND (o.sale_end_time IS NULL OR o.sale_end_time > NOW())
        ORDER BY o.price ASC
        "#,
        event_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}