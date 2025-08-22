-- Add migration script here

-- Drop in reverse order of creation.
DROP INDEX IF EXISTS idx_sessions_event_id;
DROP TRIGGER IF EXISTS set_timestamp ON event_sessions;
DROP TABLE IF EXISTS event_sessions;
DROP TYPE IF EXISTS session_status;

-- Add migration script here

-- An ENUM type for the status of a specific session.
CREATE TYPE session_status AS ENUM ('scheduled', 'sold_out', 'cancelled');

CREATE TABLE event_sessions (
    id SERIAL PRIMARY KEY,
    -- Foreign key to the parent event. If the event is deleted, its sessions are also deleted.
    event_id INT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    -- Using NUMERIC for price is best practice for currency to avoid floating point issues.
    price NUMERIC(10, 2) NOT NULL DEFAULT 0.00,
    total_tickets INT NOT NULL,
    tickets_sold INT NOT NULL DEFAULT 0,
    status session_status NOT NULL DEFAULT 'scheduled',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Ensure a session's end time is after its start time.
    CONSTRAINT check_session_times CHECK (end_time > start_time)
);

-- Trigger to automatically update the 'last_updated' column.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON event_sessions
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();

-- Add an index on the foreign key for faster lookups of sessions by event.
CREATE INDEX idx_sessions_event_id ON event_sessions(event_id);