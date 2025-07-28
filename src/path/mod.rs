// File: be-api/src/path/mod.rs

use crate::api; // Import our api module
use axum::Router;
use sqlx::PgPool;

/// Creates the master router for the entire application,
/// mounting all sub-routers at their designated paths.
pub fn router() -> Router<PgPool> {
    Router::new()
        // Mount the router from the `api` module at the "/api" path.
        // All routes defined in `api_router` will now be prefixed with "/api".
        .nest("/api", api::api_router())
    // If you had another module for web pages, you could add it here:
    // .nest("/", web::web_router())
}