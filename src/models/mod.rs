// File: src/models/mod.rs

pub mod auth;
pub mod user;

// Re-export for easier access.
pub use auth::{LoginPayload, LoginResponse, TokenClaims}; 
pub use user::{CreateUserPayload, User};