-- migrations/YYYYMMDDHHMMSS_create_attractions/down.sql

-- Drop tables in reverse order of dependency.
DROP TABLE IF EXISTS event_attractions;
DROP TABLE IF EXISTS attractions;

-- Finally, drop the custom ENUM type.
DROP TYPE IF EXISTS attraction_type;

-- migrations/YYYYMMDDHHMMSS_create_attractions/up.sql

-- An ENUM type for the kind of attraction (e.g., musical act, speaker).
CREATE TYPE attraction_type AS ENUM ('music', 'speaker', 'comedy', 'special_guest');

-- The main table to store information about individual attractions.
CREATE TABLE attractions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    image_url VARCHAR(255),
    type attraction_type NOT NULL,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- The join table to create a many-to-many relationship between events and attractions.
CREATE TABLE event_attractions (
    -- Foreign key to the event. If the event is deleted, this link is removed.
    event_id INT NOT NULL REFERENCES events(id) ON DELETE CASCADE,

    -- Foreign key to the attraction. If the attraction is deleted, this link is removed.
    attraction_id INT NOT NULL REFERENCES attractions(id) ON DELETE CASCADE,
    
    -- Optional: You can add details specific to this performance, like the time or stage.
    performance_time TIMESTAMPTZ,
    stage_name VARCHAR(100),

    -- The primary key is the combination of both IDs, ensuring an attraction
    -- cannot be added to the same event more than once.
    PRIMARY KEY (event_id, attraction_id)
);

-- ### INDEXES ###
-- It is crucial to index foreign keys in a join table for performance.
-- The primary key already creates an index on (event_id, attraction_id).
-- An additional index on just attraction_id can be useful for finding all events for a given attraction.
CREATE INDEX idx_event_attractions_attraction_id ON event_attractions(attraction_id);


-- Create a trigger to automatically update 'last_updated' for the attractions table.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON attractions
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();