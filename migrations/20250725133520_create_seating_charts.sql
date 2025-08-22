-- migrations/YYYYMMDDHHMMSS_create_seating_charts/down.sql

-- Drop tables in reverse order of their creation.
DROP TABLE IF EXISTS event_seats;
DROP TABLE IF EXISTS seats;
DROP TABLE IF EXISTS rows;
DROP TABLE IF EXISTS sections;
DROP TABLE IF EXISTS seating_charts;

-- Finally, drop the custom ENUM type.
DROP TYPE IF EXISTS seat_status;

-- migrations/YYYYMMDDHHMMSS_create_seating_charts/up.sql

-- A simple ENUM for the status of a seat during an event's sale period.
CREATE TYPE seat_status AS ENUM (
    'available',    -- Open for purchase
    'locked',       -- Temporarily held in a user's cart
    'sold',         -- Purchased
    'unavailable'   -- Not for sale (e.g., for production crew, broken seat)
);

-- #############################################################################
-- ### Part 1: The Venue Layout Template (Static Data)
-- ### This data is defined once per venue configuration.
-- #############################################################################

-- Represents a specific seating configuration for a venue (e.g., "Concert Layout").
CREATE TABLE seating_charts (
    id SERIAL PRIMARY KEY,
    venue_id INT NOT NULL REFERENCES venues(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL UNIQUE,
    background_image_url VARCHAR(255) -- Optional: An SVG/PNG of the venue map
);

-- A seating chart is composed of multiple sections (e.g., "Section 101", "Orchestra").
CREATE TABLE sections (
    id SERIAL PRIMARY KEY,
    seating_chart_id INT NOT NULL REFERENCES seating_charts(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL
);

-- Each section contains one or more rows.
CREATE TABLE rows (
    id SERIAL PRIMARY KEY,
    section_id INT NOT NULL REFERENCES sections(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL -- e.g., "A", "B", or "1", "2"
);

-- Each row contains the individual physical seats. This is the core of the layout.
CREATE TABLE seats (
    id SERIAL PRIMARY KEY,
    row_id INT NOT NULL REFERENCES rows(id) ON DELETE CASCADE,
    seat_number VARCHAR(10) NOT NULL,

    -- Simple & Powerful: Define each seat as a rectangle for easy rendering.
    -- These are the coordinates for the top-left corner of the seat.
    pos_x DECIMAL(8, 2) NOT NULL,
    pos_y DECIMAL(8, 2) NOT NULL,
    -- The dimensions of the seat's rectangle.
    width DECIMAL(8, 2) NOT NULL DEFAULT 20.0,
    height DECIMAL(8, 2) NOT NULL DEFAULT 20.0
);


-- #############################################################################
-- ### Part 2: The Event Seat Instance (Dynamic Data)
-- ### This table is populated for EACH event that uses reserved seating.
-- #############################################################################

-- This table tracks the status and price of every single seat for a specific event.
CREATE TABLE event_seats (
    event_id INT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    seat_id INT NOT NULL REFERENCES seats(id) ON DELETE CASCADE,

    -- A seat's price is determined by the Ticket Tier it belongs to for THIS event.
    ticket_tier_id INT NOT NULL REFERENCES ticket_tiers(id) ON DELETE RESTRICT,

    -- The current status of this seat for this event.
    status seat_status NOT NULL DEFAULT 'available',

    -- If a seat is locked in a cart, this tracks who has it and when their lock expires.
    lock_expires_at TIMESTAMPTZ,

    -- This would link to a future 'orders' table once the seat is sold.
    order_id INT,

    -- A seat can only exist once per event.
    PRIMARY KEY (event_id, seat_id)
);

-- ### INDEXES for performance ###
CREATE INDEX idx_sections_seating_chart_id ON sections(seating_chart_id);
CREATE INDEX idx_rows_section_id ON rows(section_id);
CREATE INDEX idx_seats_row_id ON seats(row_id);
-- This index is vital for quickly finding all seats (and their statuses) for an event.
CREATE INDEX idx_event_seats_event_id ON event_seats(event_id, status);
-- This index helps a background job find and release expired locks efficiently.
CREATE INDEX idx_event_seats_lock_expires_at ON event_seats(lock_expires_at);