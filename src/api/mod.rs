// src/api/mod.rs

use crate::{middleware::auth_guard::auth_guard, AppState}; // <-- IMPORTANT: Make sure AppState is imported
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
// We don't need to import PgPool here anymore, as it's part of AppState
// use sqlx::PgPool; 

// Declare your handler modules
pub mod auth_handler;
pub mod user_handler;
pub mod csrf_handler; // I'm assuming you have this from previous steps


/// Assembles a router for all API-specific endpoints.
// V-- THIS IS THE FIX --V
// Change the return type from Router<PgPool> to Router<AppState>
pub fn api_router() -> Router<AppState> {
    // Bring the handler functions into scope.
    use auth_handler::{forgot_password, get_me, login};
    use user_handler::{create_user, get_user_by_id};
    use csrf_handler::{get_token_handler, protected_post_handler};

    // Build and return the router.
    Router::new()
        // --- User Routes ---
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        
        // --- Auth Routes ---
        .route("/auth/login", post(login))
        .route("/auth/forgot-password", post(forgot_password))
        
        // --- CSRF Routes ---
        // Provides a token to the frontend
        .route("/csrf/token", get(get_token_handler)) 
        // An example of a form submission protected by CSRF
        .route("/csrf/protected-form", post(protected_post_handler))

        // --- Protected Route with Auth Middleware ---
        // This route requires a user to be logged in.
        .route(
            "/auth/me",
            get(get_me).route_layer(middleware::from_fn_with_state(
                // Your auth_guard needs access to the state (e.g., for DB checks)
                // So we use from_fn_with_state
                get_me.clone(), 
                auth_guard,
            )),
        )
}