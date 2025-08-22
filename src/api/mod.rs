use crate::{
    middleware::{
        admin_guard::admin_guard, auth_guard::auth_guard, csrf_guard::csrf_guard,
    },
    AppState,
};
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

// Declare all the handler modules we've created.
pub mod attraction_handler;
pub mod auth_handler;
pub mod category_handler;
pub mod csrf_handler;
pub mod event_handler;
pub mod order_handler;
pub mod payment_handler;
pub mod pricing_handler;
pub mod seating_handler;
pub mod ticket_handler;
pub mod user_handler;
pub mod venue_handler;
pub mod organizer_handler; // <-- ADD the new handler module

/// Assembles the master router for all API endpoints.
pub fn api_router() -> Router<AppState> {
    // --- Public Routes (No Auth required) ---
    let public_routes = Router::new()
        // Events
        .route("/events", get(event_handler::list_published_events))
        .route("/events/:id", get(event_handler::get_event_by_id))
        .route("/events/:event_id/offers", get(pricing_handler::list_public_offers_for_event))
        .route("/events/:event_id/seat-map", get(seating_handler::get_seat_map_for_event))
        // Venues
        .route("/venues", get(venue_handler::list_venues))
        .route("/venues/:id", get(venue_handler::get_venue_by_id))
        // Categories
        .route("/segments", get(category_handler::list_segments))
        .route("/segments/:id/genres", get(category_handler::list_genres_by_segment))
        .route("/genres/:id/sub-genres", get(category_handler::list_sub_genres_by_genre));

    // --- Authentication Routes (Public but state-changing, need CSRF) ---
    let auth_routes = Router::new()
        .route("/auth/register", post(auth_handler::register))
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/logout", post(auth_handler::logout))
        // .route("/auth/forgot-password", post(auth_handler::forgot_password)) // NO idea why this causes error T-T
        .route("/users/reset-password-otp", post(user_handler::reset_password_with_otp_handler));
        // Removed reset_password_handler as it's replaced by the OTP version

    // --- Protected Routes (Auth required) ---
    let protected_routes = Router::new()
        // User Profile & Tickets
        .route("/auth/me", get(auth_handler::get_me))
        .route("/me/tickets", get(ticket_handler::get_my_tickets))
        
        // Orders (Customer action)
        .route("/orders", post(order_handler::create_order))
        
        // Event Management (Organizer role)
        .route("/events", post(event_handler::create_event))
        .route("/events/:id", patch(event_handler::update_event))
        .route("/events/:id", delete(event_handler::delete_event))
        
        // Nested Event Resources (Organizer role)
        .route("/events/:event_id/attractions", post(attraction_handler::add_attraction_to_event))
        .route("/events/:event_id/attractions/:attraction_id", delete(attraction_handler::remove_attraction_from_event))
        .route("/events/:event_id/tiers", post(pricing_handler::create_ticket_tier))
        .route("/tiers/:tier_id/offers", post(pricing_handler::create_offer))

        // --- ADDED: Organizer-specific routes ---
        .route("/organizer/stripe/onboarding-link", post(organizer_handler::get_onboarding_link));


    // --- Admin-Only Routes (Auth and Admin Role required) ---
    let admin_routes = Router::new()
        .route("/venues", post(venue_handler::create_venue))
        .route("/segments", post(category_handler::create_segment))
        .route("/segments/:id/genres", post(category_handler::create_genre))
        .route("/genres/:id/sub-genres", post(category_handler::create_sub_genre))
        // You would also need an admin login endpoint, e.g., /admin/login in auth_routes
        .layer(middleware::from_fn(admin_guard));

    // --- Webhook Routes (No Auth, No CSRF - special case) ---
    let webhook_routes = Router::<AppState>::new()
        .route("/webhooks/stripe", post(payment_handler::stripe_webhook_handler));

    // --- Assemble the Final Router ---
    Router::new()
        // Public GET routes that are safe from CSRF
        .merge(public_routes)
        .route("/csrf/token", get(csrf_handler::get_csrf_token))
        
        // Public POST routes that require CSRF protection
        .merge(auth_routes.layer(middleware::from_fn(csrf_guard)))

        // All protected and admin routes need CSRF and then Auth.
        .merge(
            Router::new()
                .merge(protected_routes)
                .merge(admin_routes)
                .layer(middleware::from_fn(csrf_guard))
                .layer(middleware::from_fn(auth_guard)),
        )
        
        // Webhooks are a special case and live at the top level
        .merge(webhook_routes)
}