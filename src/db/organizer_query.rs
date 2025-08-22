use crate::{
    errors::AppError, models::user::OrganizerProfile,
};
use sqlx::{Executor, PgPool, Postgres};

/// Fetches an organizer's profile from the database using their user ID.
pub async fn get_profile_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<OrganizerProfile, AppError> {
    // FIX: Replaced `SELECT *` with an explicit list of all columns
    // The order of columns here MUST match the order of fields in the OrganizerProfile struct.
    sqlx::query_as!(
        OrganizerProfile,
        "SELECT
            user_id,
            company_name,
            contact_phone,
            website_url,
            stripe_account_id,
            created_at,
            last_updated
         FROM organizer_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

/// Creates a new organizer profile, linking it to an existing user.
pub async fn create_profile<'e, E>(
    executor: E,
    user_id: i32,
    company_name: Option<String>,
) -> Result<OrganizerProfile, AppError>
where
    E: Executor<'e, Database = Postgres>,
{
    // FIX: Replaced `RETURNING *` with an explicit list of all columns
    // to match the OrganizerProfile struct.
    sqlx::query_as!(
        OrganizerProfile,
        "INSERT INTO organizer_profiles (user_id, company_name) VALUES ($1, $2)
         RETURNING
            user_id,
            company_name,
            contact_phone,
            website_url,
            stripe_account_id,
            created_at,
            last_updated",
        user_id,
        company_name,
    )
    .fetch_one(executor)
    .await
    .map_err(AppError::from)
}

/// Updates an organizer's profile with their new Stripe Connect Account ID.
/// This function does not use `query_as!` so it was already correct.
pub async fn set_stripe_account_id(
    pool: &PgPool,
    user_id: i32,
    stripe_account_id: &str,
) -> Result<(), AppError> {
    let result = sqlx::query!(
        "UPDATE organizer_profiles SET stripe_account_id = $1 WHERE user_id = $2",
        stripe_account_id,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::Sqlx(sqlx::Error::RowNotFound));
    }

    Ok(())
}