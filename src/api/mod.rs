// src/api/mod.rs

use crate::{
    middleware::{
        auth_guard::auth_guard,
        csrf_guard::csrf_guard,
    },
    AppState,
};
use axum::{
    middleware,
    routing::{get, post, patch, delete},
    Router,
};

// Declare all your handler modules
pub mod auth_handler;
pub mod csrf_handler;
pub mod user_handler;
pub mod websocket_handler;

/// Assembles a router for all API-specific endpoints.
pub fn api_router(app_state: AppState) -> Router<AppState> {
    // Public routes that don't need CSRF protection (GET requests)
    let public_router_no_csrf = Router::new()
        .route("/csrf/token", get(csrf_handler::get_csrf_token));

    // Public routes that need CSRF protection (POST requests)
    let public_router = Router::new()
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/token", get(auth_handler::get_auth_token))
        .route("/auth/forgot-password", post(auth_handler::forgot_password))
        .route("/users/reset-password", post(user_handler::reset_password_handler))
        .route("/users", post(user_handler::create_user))
        .route("/auth/logout", post(auth_handler::logout))
        .layer(middleware::from_fn(csrf_guard));

    // Protected routes that don't fit into a nested resource structure
    let protected_router_base = Router::new()
        .route("/auth/me", get(auth_handler::get_me))
        .route("/users/:id", get(user_handler::get_user_by_id));


    // Main protected router assembly
    let final_protected_router = Router::new()
        .route("/projects", get("project_handler::get_my_projects"))
        .merge(protected_router_base)
        .layer(middleware::from_fn(csrf_guard)); 
    

    // --- NEW: WebSocket Router (Auth protected, but NO CSRF) ---
    let ws_router: Router<AppState> = Router::new()
        .route("/projects/:project_id/ws", get(websocket_handler::project_ws_handler));

    // Final root router
    Router::new()
        .merge(public_router_no_csrf)
        .merge(public_router)
        .merge(final_protected_router.layer(middleware::from_fn(auth_guard)))
        .merge(ws_router.layer(middleware::from_fn(auth_guard)))
}