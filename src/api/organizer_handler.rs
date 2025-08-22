use crate::{
    errors::AppError,
    service::organizer_service,
    AppState,
};
use axum::{
    extract::{Extension, State},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct OnboardingLinkResponse {
    pub url: String,
}

/// Handler for an organizer to get a Stripe onboarding link.
/// POST /api/organizer/stripe/onboarding-link
#[tracing::instrument(skip(app_state))]
pub async fn get_onboarding_link(
    State(app_state): State<AppState>,
    Extension(organizer_user_id): Extension<i32>,
) -> Result<Json<OnboardingLinkResponse>, AppError> {
    let url = organizer_service::create_stripe_onboarding_link(
        &app_state.db_pool,
        &app_state.stripe_client,
        organizer_user_id,
    )
    .await?;

    Ok(Json(OnboardingLinkResponse { url }))
}