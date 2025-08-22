-- migrations/YYYYMMDDHHMMSS_create_venues_table/down.sql

-- Drop the venues table if it exists.
DROP TABLE IF EXISTS venues;

-- migrations/YYYYMMDDHHMMSS_create_venues_table/up.sql

-- Table to store information about event venues.
CREATE TABLE venues (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,

    -- Location Details
    address_line_1 VARCHAR(255),
    address_line_2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    state VARCHAR(100),
    postal_code VARCHAR(20) NOT NULL,
    country VARCHAR(100) NOT NULL,

    -- Venue specific information
    capacity INT, -- Maximum number of attendees

    -- Optional Geospatial Data for mapping and location-based searches
    -- For more advanced use-cases, consider using the PostGIS extension.
    latitude DECIMAL(9, 6),
    longitude DECIMAL(9, 6),

    -- Contact Information
    phone_number VARCHAR(50),
    website_url VARCHAR(255),

    -- Metadata
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add an index on city and state for faster location-based lookups.
CREATE INDEX idx_venues_city_state ON venues(city, state);

-- Create a trigger to automatically update 'last_updated' on modification.
-- This reuses the function created in the first migration.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON venues
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();