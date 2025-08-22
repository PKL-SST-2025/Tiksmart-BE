// File: src/service/organizer_service.rs

use std::str::FromStr;

use crate::{
    config::CONFIG, db::organizer_query, errors::AppError
};
use sqlx::PgPool;
// --- Imports for the synchronous `stripe` crate ---
use stripe::{
    Account, AccountId, AccountLink, AccountLinkCollect, AccountLinkType, AccountType, Client, CreateAccount, CreateAccountLink
};

/// Service to create a Stripe Connected Account and generate an onboarding link for an organizer.
pub async fn create_stripe_onboarding_link(
    pool: &PgPool,
    stripe_client: &Client,
    organizer_user_id: i32, // The ID of the organizer logged in
) -> Result<String, AppError> {
    // 1. Fetch the organizer's profile from your database.
    let profile = organizer_query::get_profile_by_user_id(pool, organizer_user_id).await?;

    // Use a variable to hold the Stripe Account ID, whether it's existing or new.
    let account_id_str: String;

    // 2. Check if the organizer already has a Stripe Account ID.
    if let Some(existing_account_id) = &profile.stripe_account_id {
        account_id_str = existing_account_id.clone();
        tracing::info!(
            "Organizer {} already has Stripe account: {}",
            organizer_user_id,
            account_id_str
        );
    } else {
        // 3. If no account exists, create a new Stripe Express Account.
        tracing::info!(
            "Creating new Stripe Express account for organizer {}",
            organizer_user_id
        );

        let mut create_account_params = CreateAccount::new();
        create_account_params.type_ = Some(AccountType::Express);
        // You would also fetch the organizer's email from the `users` table and set it here.
        // create_account_params.email = Some("organizer@example.com");

        let account = Account::create(stripe_client, create_account_params).await?;
        let new_account_id = account.id.to_string();

        // 4. Save the new Stripe Account ID to YOUR database.
        organizer_query::set_stripe_account_id(pool, organizer_user_id, &new_account_id).await?;
        account_id_str = new_account_id;

    };

    
    // 1. Call `from_str` which returns a `Result`.
    // 2. Use `map_err` to convert the specific `ParseIdError` into our general `AppError`.
    // 3. Use `?` to unwrap the `AccountId` if successful, or return the `AppError` if not.
    let account_id = AccountId::from_str(&account_id_str).map_err(|e| {
        tracing::error!("Failed to parse Stripe Account ID from database: {}. ID was: {}", e, account_id_str);
        // This should never happen if our data is clean, so it's an internal server error.
        AppError::InternalServerError("Invalid Stripe Account ID format encountered.".to_string())
    })?;

    let return_url = format!("{}/stripe/return", &CONFIG.frontend_origin);
    let refresh_url = format!("{}/stripe/refresh", &CONFIG.frontend_origin);

    let server_origin = CONFIG.server_address.to_string();
    // 5. Create an Account Link. This is a short-lived, unique URL.
    let account_link_params = CreateAccountLink {
        // FIX 1: The `account` field expects a `&str`, not a `String`.
        // We borrow the String `account_id_str` to get a `&str`.
        account: account_id,

        collect: Some(AccountLinkCollect::CurrentlyDue),

        // The required URL fields
        return_url: Some(&return_url),
        refresh_url: Some(&refresh_url),
        type_: AccountLinkType::AccountOnboarding,

        collection_options: Default::default(), // These sub-structs often do implement Default
        expand: &[],
    };

    // Call the create method on the AccountLink resource itself.
    let account_link = AccountLink::create(stripe_client, account_link_params).await?;

    // 6. Return the URL from the Account Link object for the frontend to use.
    Ok(account_link.url)
}