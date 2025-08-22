use crate::{
    db::{event_query, seating_query},
    errors::AppError,
    models::{SeatMapInfo, Seat}, // We don't need to expose all sub-models here
};
use rust_decimal::Decimal;
use sqlx::PgPool;

// --- Layout Management Services (Admin/Organizer Setup) ---

/// Service to create a new physical seat.
/// In a real application, this would likely be part of a larger, transactional
/// "bulk create" service for setting up an entire venue layout at once.
/// For simplicity, here's the single-seat version.
pub async fn create_seat(
    pool: &PgPool,
    row_id: i32,
    seat_number: &str,
    pos_x: Decimal,
    pos_y: Decimal,
) -> Result<Seat, AppError> {
    // Authorization logic would go here (e.g., check if user is venue admin)
    seating_query::create_seat(pool, row_id, seat_number, pos_x, pos_y).await
}

/// The critical service function to "instantiate" a seating chart for a new event.
/// This populates the `event_seats` table with a record for every seat in the chart.
/// This is a transactional operation.
pub async fn initialize_event_seating(
    pool: &PgPool,
    event_id: i32,
    organizer_id: i32, // User performing the action
    seating_chart_id: i32,
    default_ticket_tier_id: i32, // All seats will be assigned this tier initially
) -> Result<u64, AppError> {
    // 1. Authorization: Check if the user is the organizer of the event.
    let event = event_query::get_by_id(pool, event_id).await?;
    if event.organizer_id != organizer_id {
        return Err(AppError::Forbidden(
            "You are not authorized to configure seating for this event.".to_string(),
        ));
    }
    
    // In a real app, you'd also check that the seating_chart_id is valid for the event's venue
    // and that the ticket_tier_id belongs to this event.

    // 2. Begin a transaction.
    let mut tx = pool.begin().await?;

    // 3. Call the transactional database query.
    let result = seating_query::create_event_seats_for_chart(
        &mut tx,
        event_id,
        seating_chart_id,
        default_ticket_tier_id,
    )
    .await;

    // 4. Commit or rollback.
    match result {
        Ok(rows_affected) => {
            tx.commit().await?;
            Ok(rows_affected)
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
}

// --- Public / Checkout Services ---

/// Service to fetch the complete seat map for rendering on the frontend.
/// This is a public-facing, read-only operation.
pub async fn get_seat_map_for_event(
    pool: &PgPool,
    event_id: i32,
) -> Result<Vec<SeatMapInfo>, AppError> {
    seating_query::get_seat_map_for_event(pool, event_id).await
}

/// Service to lock a seat during the checkout process.
/// This is an internal function, likely called by the `order_service`.
pub async fn lock_seat(
    pool: &PgPool,
    event_id: i32,
    seat_id: i32,
) -> Result<(), AppError> {
    const LOCK_DURATION_MINUTES: i64 = 15; // The business rule for cart expiration
    seating_query::lock_seat(pool, event_id, seat_id, LOCK_DURATION_MINUTES).await
}

/// Service to unlock a seat if a user abandons their cart.
pub async fn unlock_seat(
    pool: &PgPool,
    event_id: i32,
    seat_id: i32,
) -> Result<(), AppError> {
    seating_query::unlock_seat(pool, event_id, seat_id).await
}

// --- Background Job Service ---

/// Service function for a background worker to periodically clean up expired locks.
pub async fn release_expired_locks(pool: &PgPool) -> Result<u64, AppError> {
    let rows_affected = seating_query::release_expired_locks(pool).await?;
    if rows_affected > 0 {
        tracing::info!("Released {} expired seat locks.", rows_affected);
    }
    Ok(rows_affected)
}