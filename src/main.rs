// src/main.rs

use axum::{
    extract::FromRef,
    http::{header, HeaderValue, Method, StatusCode},
    Router,
};
use axum_csrf::{CsrfConfig, Key};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tokio::time::{self, Duration};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use tracing_subscriber::{fmt, EnvFilter};

use dashmap::DashMap;
use tokio::sync::broadcast;
use crate::{config::CONFIG};
use stripe::Client as StripeClient; 

// Declare all your modules
mod api;
mod config;
mod db;
mod errors;
mod models;
mod utils;
mod middleware;
mod service;
mod clients;

// The central state for our application
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>, // Already Arc
    pub csrf_config: CsrfConfig,
    // NEW: DashMap to store broadcast senders per project.
    // project_id -> broadcast::Sender<String> (we'll send JSON strings)
    pub project_ws_senders: Arc<DashMap<i32, broadcast::Sender<String>>>,
        pub stripe_client: Arc<StripeClient>,
}

// Implement `FromRef` for the new Stripe client
impl FromRef<AppState> for Arc<StripeClient> {
    fn from_ref(state: &AppState) -> Self {
        state.stripe_client.clone()
    }
}

// Implement `FromRef` so extractors can get their dependencies from `AppState`
impl FromRef<AppState> for Arc<PgPool> { // Corrected trait implementation
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

impl FromRef<AppState> for CsrfConfig {
    fn from_ref(state: &AppState) -> Self {
        state.csrf_config.clone()
    }
}

// NEW: Implement FromRef for the new project_ws_senders
impl FromRef<AppState> for Arc<DashMap<i32, broadcast::Sender<String>>> {
    fn from_ref(state: &AppState) -> Self {
        state.project_ws_senders.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting server with config: {:?}", *CONFIG);

    // --- CSRF Configuration ---
    let csrf_key = Key::from(CONFIG.csrf_secret.as_bytes());
    let csrf_config = CsrfConfig::default()
        .with_secure(CONFIG.env != "development") // Use secure cookies in production
        .with_key(Some(csrf_key));


    // --- Database Pool ---
    let pool = db::pool::create_pool()
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database connected successfully");

    // Wrap the pool in Arc for shared state
    let shared_db_pool = Arc::new(pool);

    // Initialize the DashMap for WebSocket senders
    let project_ws_senders = Arc::new(DashMap::new());

    // --- Stripe Client Initialization ---
    let stripe_client = clients::stripe_client::create_stripe_client();
    let shared_stripe_client = Arc::new(stripe_client); // Correctly wrapped in Arc

    // --- Create the single AppState ---
    let app_state = AppState {
        db_pool: shared_db_pool,
        csrf_config,
        project_ws_senders: project_ws_senders,
        stripe_client: shared_stripe_client, 
    };

    // --- CORS Layer ---
    let cors = CorsLayer::new()
        .allow_origin(
            CONFIG
                .frontend_origin
                .parse::<HeaderValue>()
                .expect("Invalid frontend origin"),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // IMPORTANT: You MUST allow the `x-csrf-token` header.
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true);

    // --- Rate Limiting Layer ---
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(CONFIG.rate_limit_period_seconds)
            .burst_size(CONFIG.rate_limit_requests)
            .finish()
            .unwrap(),
    );
    let governor_layer = GovernorLayer {
        config: Box::leak(governor_conf),
    };

    // --- Security Headers Layer ---
    let security_headers_layer = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ));

    // --- Build The App ---
    let serve_dir =
        ServeDir::new("../fe/dist").not_found_service(ServeFile::new("../fe/dist/index.html"));

    let api_service = api::api_router() // Pass cloned app_state instance
        // .layer(governor_layer) // TODO: FIX THIS GOVERNOR_LAYER NO IDEA WHY IT WONT WORK
        .layer(cors);

    let app = Router::new()
        .nest("/api", api_service)
        .fallback_service(serve_dir)
        .layer(security_headers_layer)
        .with_state(app_state);


    // --- Run The Server ---
    let listener = TcpListener::bind(&CONFIG.server_address)
        .await
        .expect("Cannot create TCP listener.");
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

