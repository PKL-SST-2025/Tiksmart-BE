// File: src/models/permission_tier.rs

use serde::Serialize;

// Represents a row in the 'permission_tier' table.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PermissionTier {
    pub id: i32,
    pub permission: String,
}