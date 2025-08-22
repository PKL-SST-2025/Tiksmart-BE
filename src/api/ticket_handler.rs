use crate::{errors::AppError, models::TicketDetails, service::ticket_service, AppState};
use axum::{
    extract::{Extension, State},
    Json,
};

/// Handler for an authenticated user to get a list of all their tickets.
/// GET /api/me/tickets
#[tracing::instrument(skip(app_state))]
pub async fn get_my_tickets(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<Vec<TicketDetails>>, AppError> {
    let tickets = ticket_service::get_user_tickets(&app_state.db_pool, user_id).await?;
    Ok(Json(tickets))
}