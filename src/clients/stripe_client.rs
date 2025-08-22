// File: src/clients/stripe_client.rs

use crate::config::CONFIG;

// Use the synchronous `stripe` crate's Client
use stripe::Client;

/// Creates and returns a new Stripe client using the secret key from the config.
pub fn create_stripe_client() -> Client {
    Client::new(&CONFIG.stripe_secret_key)
}