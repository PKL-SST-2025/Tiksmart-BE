-- migrations/YYYYMMDDHHMMSS_add_role_to_users/up.sql

-- migrations/YYYYMMDDHHMMSS_add_role_to_users/down.sql

ALTER TABLE users DROP COLUMN IF EXISTS role;
DROP TYPE IF EXISTS user_role;

-- Create a new type for user roles
CREATE TYPE user_role AS ENUM ('attendee', 'organizer');

-- Add the new role column to the existing users table
-- Existing users will default to 'attendee'
ALTER TABLE users ADD COLUMN role user_role NOT NULL DEFAULT 'attendee';