-- migrations/YYYYMMDDHHMMSS_create_events_table/down.sql

-- Drop the table first, only if it exists.
-- This will automatically drop its indexes and triggers.
DROP TABLE IF EXISTS events;

-- Then, drop the custom ENUM type, only if it exists.
DROP TYPE IF EXISTS event_status;

-- migrations/YYYYMMDDHHMMSS_create_events_table/up.sql

-- An ENUM type for the status of an event.
CREATE TYPE event_status AS ENUM ('draft', 'published', 'cancelled', 'completed', 'on_sale', 'sold_out');

-- The main events table, now with price range fields.
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    organizer_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    venue_id INT REFERENCES venues(id) ON DELETE SET NULL,
    segment_id INT REFERENCES segments(id) ON DELETE SET NULL,
    genre_id INT REFERENCES genres(id) ON DELETE SET NULL,
    sub_genre_id INT REFERENCES sub_genres(id) ON DELETE SET NULL,

    -- Core event details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status event_status NOT NULL DEFAULT 'draft',

    -- Event timing
    start_time TIMESTAMptz NOT NULL,
    end_time TIMESTAMPTZ,

    -- Price range fields for display purposes.
    -- These are denormalized and should be updated by application logic or a trigger
    -- whenever an offer for this event is created or changed.
    price_min DECIMAL(10, 2),
    price_max DECIMAL(10, 2),

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ### INDEXES ###
CREATE INDEX idx_events_organizer_id ON events(organizer_id);
CREATE INDEX idx_events_venue_id ON events(venue_id);
CREATE INDEX idx_events_segment_id ON events(segment_id);
CREATE INDEX idx_events_start_time ON events(start_time);

-- Create a trigger to automatically update 'last_updated'
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON events
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();