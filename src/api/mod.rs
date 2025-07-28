// File: be-api/src/api/mod.rs
// This file is already doing its job correctly: creating a self-contained router for API endpoints.
use crate::middleware::auth_guard::auth_guard;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

// Declare the sub-modules.
pub mod auth_handler;
pub mod user_handler;

/// Assembles a router for all API-specific endpoints.
pub fn api_router() -> Router<PgPool> {
    // Bring the handler functions into scope.
    use auth_handler::{forgot_password, get_me, login}; 
    use user_handler::{create_user, create_user_bulk, get_user_by_id}; 
    
    // Build and return the router. Note it doesn't have the "/api" prefix here.
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/auth/login", post(login))
        // This is the protected route.
        .route(
            "/auth/me",
            get(get_me).route_layer(middleware::from_fn(auth_guard)), // <-- Apply middleware here
        )
         .route("/auth/forgot-password", post(forgot_password)) 
         
}