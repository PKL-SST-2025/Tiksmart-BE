use axum::{
    http::{header, HeaderValue},
    http::StatusCode,
    Router,
    extract::{State},
    routing::{get, patch},
    Json,
};

use tower_http::set_header::SetResponseHeader;

use serde::{Deserialize, Serialize};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};

use tokio::net::TcpListener; 

use tower_http::services::{ServeDir, ServeFile};

use tracing_subscriber::{fmt, EnvFilter};

// Cors
use axum::http::{header, HeaderValue, Method};
use tower_http::cors::CorsLayer;

// tower
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tracing_subscriber::FmtSubscriber;

use crate::config::CONFIG;

// Declare the top-level modules of our application
mod api;
mod errors; 
mod models;
mod db;
mod path; 
mod service;
mod middleware;
mod utils;
mod config;

#[tokio::main]
async fn main() {
    // Expose env
    dotenvy::dotenv().expect("Unable to access .env file");

    // This will read the RUST_LOG environment variable to determine the log level.
    // If RUST_LOG is not set, it will default to `info`.
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting server with config: {:?}", *config::CONFIG);
    
    // --> Build the CORS layer (For Security. Duh.) <--
    let cors = CorsLayer::new()
        // Allow requests from our frontend origin.
        .allow_origin(
            config::CONFIG
                .frontend_origin
                .parse::<HeaderValue>()
                .expect("Invalid frontend origin"),
        )
        // Allow specific HTTP methods.
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // Allow specific headers.
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        // Allow credentials (e.g., cookies) to be sent.
        .allow_credentials(true);

        

    // Configure the rate limiter
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            // How many requests per period
            .per_second(CONFIG.rate_limit_period_seconds)
            // The burst size
            .burst_size(CONFIG.rate_limit_requests) // Allow a burst of 5 requests
            .finish()
            .unwrap(),
    );

    // Now we access the validated, typed config directly.
    let server_address = &config::CONFIG.server_address;

    let db_pool = db::pool::create_pool()
        .await
        .expect("Failed to create database pool");

    println!("🗃️ Database connected successfully");

    let listener = TcpListener::bind(server_address) // <-- Use the config variable
        .await
        .expect("Cannot create TCP listener.");

    println!("🚀 Listening on http://{}", listener.local_addr().unwrap());

    // Create the router and pass it the database pool
    let app = create_router(db_pool).layer(cors).layer(
        ServiceBuilder::new().layer(GovernorLayer {
            config: Box::leak(governor_conf),
        }),
    )
    .layer(SetResponseHeader::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        ))
        // X-Content-Type-Options: Prevents the browser from trying to
        // guess the MIME type of a response.
        .layer(SetResponseHeader::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        // X-Frame-Options: Prevents your site from being rendered in an
        // <iframe>, <frame>, <embed> or <object>, which helps prevent
        // clickjacking attacks.
        .layer(SetResponseHeader::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        // Optional: Content-Security-Policy (CSP) - A powerful but complex header.
        // This is a very strict default policy. You will likely need to customize it
        // for your frontend's needs (e.g., to allow scripts from certain domains).
        .layer(SetResponseHeader::if_not_present(
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'self'"),
        ));

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

/// Assembles the final application router.
fn create_router(db_pool: PgPool) -> Router {
    let serve_dir = ServeDir::new("../fe/dist")
        .not_found_service(ServeFile::new("../fe/dist/index.html"));

    Router::new()
        // The rate limiting will now be applied inside `path::router()`
        .merge(path::router())
        .fallback_service(serve_dir)
        .with_state(db_pool)
}

