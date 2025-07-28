use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::CONFIG; 

/// Creates and returns a database connection pool.
/// Reads the DATABASE_URL from the environment.
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(16)
        .connect(&CONFIG.database_url) 
        .await
}