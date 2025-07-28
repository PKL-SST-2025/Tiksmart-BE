// src/main.rs

use axum::{
    extract::FromRef,
    http::{header, HeaderValue, Method, StatusCode},
    Router,
};
use axum_csrf::{CsrfConfig, Key};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::CONFIG;

// Declare all your modules
mod api;
mod config;
mod db;
mod errors;
mod models;
mod path;
mod utils;
mod middleware;
mod service;

// The central state for our application
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    csrf_config: CsrfConfig,
}

// Implement `FromRef` so extractors can get their dependencies from `AppState`
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db_pool.clone()
    }
}

impl FromRef<AppState> for CsrfConfig {
    fn from_ref(state: &AppState) -> Self {
        state.csrf_config.clone()
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
    let db_pool = db::pool::create_pool()
        .await
        .expect("Failed to create database pool");
    tracing::info!("Database connected successfully");

    // --- Create the single AppState ---
    let app_state = AppState {
        db_pool,
        csrf_config,
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

    let app = Router::new()
        .merge(path::router()) // Add our API routes
        .fallback_service(serve_dir) // Serve the frontend
        .with_state(app_state) // Provide the state to the entire app
        .layer(governor_layer) // Apply rate limiting
        .layer(cors) // Apply CORS
        .layer(security_headers_layer); // Apply security headers

    // --- Run The Server ---
    let listener = TcpListener::bind(&CONFIG.server_address)
        .await
        .expect("Cannot create TCP listener.");
    tracing::info!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

