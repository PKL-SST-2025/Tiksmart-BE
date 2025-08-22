-- migrations/YYYYMMDDHHMMSS_create_organizer_profiles/up.sql

DROP TABLE IF EXISTS organizer_profiles;

-- This table stores extra information ONLY for users with the 'organizer' role.
CREATE TABLE organizer_profiles (
    -- This is a one-to-one relationship with the users table.
    -- The user_id is both the primary key and the foreign key.
    user_id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,

    -- Organizer-specific information
    company_name VARCHAR(255),
    contact_phone VARCHAR(50),
    website_url VARCHAR(255),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Optional: Create a trigger to automatically update 'last_updated'
-- This reuses the function created in the very first migration.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON organizer_profiles
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();