// this is at path btw not api
use crate::{api::{ api_router, csrf_handler}, AppState}; // <-- Import AppState
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
/// Creates the master router for the entire application.
/// It must be generic over `AppState` to be compatible with `main.rs`.
pub fn router() -> Router<AppState> { // <-- CHANGE 1: Return Router<AppState>
    Router::new()
        // Mount the API sub-router.
        // `api::api_router()` will also return a `Router<AppState>`.
        .nest("/api", api_router()) // <-- CHANGE 2: This now calls the new api_router
}