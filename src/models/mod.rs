// File: src/models/mod.rs

pub mod auth;

// Re-export for easier access in other parts of the application.
pub use auth::{
    ForgotPasswordPayload, LoginPayload, LoginResponse, TokenClaims,
};

pub mod user;