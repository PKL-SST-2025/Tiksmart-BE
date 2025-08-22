use crate::{
    errors::AppError,
    models::{EventSeat, Seat, SeatMapInfo, SeatStatus, Section, Row},
};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};

// --- Layout Management Queries (The Template) ---

/// Creates a new section within a seating chart.
pub async fn create_section(
    pool: &PgPool,
    seating_chart_id: i32,
    name: &str,
) -> Result<Section, AppError> {
    sqlx::query_as!(
        Section,
        "INSERT INTO sections (seating_chart_id, name) VALUES ($1, $2) RETURNING *",
        seating_chart_id,
        name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Creates a new row within a section.
pub async fn create_row(pool: &PgPool, section_id: i32, name: &str) -> Result<Row, AppError> {
    sqlx::query_as!(
        Row,
        "INSERT INTO rows (section_id, name) VALUES ($1, $2) RETURNING *",
        section_id,
        name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Creates a new physical seat within a row, defining its position.
pub async fn create_seat(
    pool: &PgPool,
    row_id: i32,
    seat_number: &str,
    pos_x: Decimal,
    pos_y: Decimal,
) -> Result<Seat, AppError> {
    sqlx::query_as!(
        Seat,
        "INSERT INTO seats (row_id, seat_number, pos_x, pos_y) VALUES ($1, $2, $3, $4) RETURNING *",
        row_id,
        seat_number,
        pos_x,
        pos_y
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

// --- Event Seat Management Queries (The Instance) ---

/// Populates the `event_seats` table for a new event.
/// This crucial function creates a dynamic record for every physical seat in a chart,
/// assigning them to a default ticket tier.
/// MUST be run in a transaction.
pub async fn create_event_seats_for_chart(
    tx: &mut Transaction<'_, Postgres>,
    event_id: i32,
    seating_chart_id: i32,
    default_ticket_tier_id: i32,
) -> Result<u64, AppError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO event_seats (event_id, seat_id, ticket_tier_id, status)
        SELECT
            $1 as event_id,
            s.id as seat_id,
            $3 as ticket_tier_id,
            'available' as status
        FROM seats s
        JOIN rows r ON s.row_id = r.id
        JOIN sections sec ON r.section_id = sec.id
        WHERE sec.seating_chart_id = $2
        "#,
        event_id,
        seating_chart_id,
        default_ticket_tier_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected())
}

/// Fetches all the data needed to render a complete, interactive seat map for an event.
/// This joins the static layout data with the dynamic event-specific data.
pub async fn get_seat_map_for_event(
    pool: &PgPool,
    event_id: i32,
) -> Result<Vec<SeatMapInfo>, AppError> {
    sqlx::query_as!(
        SeatMapInfo,
        r#"
        SELECT
            s.id as seat_id,
            s.seat_number,
            s.pos_x,
            s.pos_y,
            es.status as "status: _",
            es.ticket_tier_id,
            tt.name as ticket_tier_name,
            o.price
        FROM event_seats es
        JOIN seats s ON es.seat_id = s.id
        JOIN ticket_tiers tt ON es.ticket_tier_id = tt.id
        -- We need a representative offer to get the price.
        -- This assumes one primary 'on_sale' offer per tier for display.
        LEFT JOIN offers o ON tt.id = o.ticket_tier_id AND o.status = 'on_sale' AND o.access_code IS NULL
        WHERE es.event_id = $1
        "#,
        event_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}


/// Attempts to lock a specific seat for an event.
/// This is a critical atomic operation for the checkout process.
/// It will fail if the seat is not 'available'.
pub async fn lock_seat(
    pool: &PgPool,
    event_id: i32,
    seat_id: i32,
    lock_duration_minutes: i64,
) -> Result<(), AppError> {
    let lock_expires_at = chrono::Utc::now()
        + chrono::Duration::minutes(lock_duration_minutes);

    let result = sqlx::query!(
        "UPDATE event_seats
         SET status = 'locked', lock_expires_at = $1
         WHERE event_id = $2 AND seat_id = $3 AND status = 'available'",
        lock_expires_at,
        event_id,
        seat_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        // This is not a RowNotFound error, it's a conflict. The seat was likely taken.
        // We can create a specific error for this in `AppError` later if needed.
        Err(AppError::BadRequest("Seat is not available for locking.".to_string()))
    } else {
        Ok(())
    }
}

/// Unlocks a specific seat (e.g., user removes it from their cart).
pub async fn unlock_seat(pool: &PgPool, event_id: i32, seat_id: i32) -> Result<(), AppError> {
    sqlx::query!(
        "UPDATE event_seats
         SET status = 'available', lock_expires_at = NULL
         WHERE event_id = $1 AND seat_id = $2 AND status = 'locked'",
        event_id,
        seat_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Finds and releases all expired seat locks across the system.
/// This should be run periodically by a background worker/job.
pub async fn release_expired_locks(pool: &PgPool) -> Result<u64, AppError> {
    let result = sqlx::query!(
        "UPDATE event_seats
         SET status = 'available', lock_expires_at = NULL
         WHERE status = 'locked' AND lock_expires_at < NOW()"
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}