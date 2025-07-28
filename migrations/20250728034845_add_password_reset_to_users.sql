-- Add nullable columns to the users table for password reset functionality.
-- A token will be stored here when a user requests a password reset.
-- It is nullable because most of the time, users are not in a reset state.

ALTER TABLE users
ADD COLUMN password_reset_token VARCHAR(64),
ADD COLUMN password_reset_expires_at TIMESTAMPTZ;

-- Add an index on the reset token for fast lookups when a user provides it.
CREATE INDEX idx_users_password_reset_token ON users(password_reset_token);