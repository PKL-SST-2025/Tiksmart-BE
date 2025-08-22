-- migrations/20250725133511_create_users_table_and_trigger/up.sql

-- A trigger function to automatically update a 'last_updated' timestamp.
-- Note: The provided users table does not have a 'last_updated' column,
-- but we will create the function as requested for potential future use.
CREATE OR REPLACE FUNCTION update_last_updated_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.last_updated = now();
   RETURN NEW;
END;
$$ language 'plpgsql';


-- Users Table
-- A central table for all users in the system.
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    password_reset_token VARCHAR(6),
    password_reset_expires_at TIMESTAMPTZ
);