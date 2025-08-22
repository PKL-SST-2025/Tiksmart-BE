-- migrations/YYYYMMDDHHMMSS_create_tickets_table/down.sql

-- Drop the table first.
DROP TABLE IF EXISTS tickets;

-- Then, drop the custom ENUM type.
DROP TYPE IF EXISTS ticket_status;

-- migrations/YYYYMMDDHHMMSS_create_tickets_table/up.sql

-- An ENUM type for the lifecycle of a single ticket.
CREATE TYPE ticket_status AS ENUM (
    'valid',        -- The ticket is active and can be used for entry.
    'checked_in',   -- The ticket has been scanned and used for entry.
    'voided',       -- The ticket has been cancelled or refunded and is no longer valid.
    'resold'        -- The ticket has been transferred or resold to another user.
);

-- The tickets table. Each row represents one unique ticket purchased by a user.
CREATE TABLE tickets (
    -- A UUID is a secure, non-guessable primary key for a ticket.
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Foreign key to the order this ticket is a part of.
    -- If an order is deleted, all its associated tickets are also deleted.
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,

    -- Foreign key to the user who owns the ticket.
    user_id INT NOT NULL REFERENCES users(id) ON DELETE RESTRICT,

    -- Foreign key to the event the ticket is for.
    event_id INT NOT NULL REFERENCES events(id) ON DELETE RESTRICT,

    -- Foreign key to the pricing tier. This tells us what was sold (e.g., "GA", "VIP").
    ticket_tier_id INT NOT NULL REFERENCES ticket_tiers(id) ON DELETE RESTRICT,

    -- Foreign key to a specific seat.
    -- This is NULLABLE to support General Admission tickets, which are not tied to a specific seat.
    seat_id INT REFERENCES seats(id) ON DELETE RESTRICT,

    -- The actual price paid for this single ticket (for historical records).
    price_paid DECIMAL(10, 2) NOT NULL,

    -- The unique data to be encoded into a QR code for scanning.
    qr_code_data TEXT UNIQUE NOT NULL DEFAULT uuid_generate_v4(),

    -- The current status of the ticket.
    status ticket_status NOT NULL DEFAULT 'valid',

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    checked_in_at TIMESTAMPTZ -- Becomes non-null when the ticket is scanned at the event.
);

-- ### CONSTRAINTS & INDEXES ###

-- A specific seat can only be sold once for a given event.
-- This constraint enforces that rule at the database level.
-- NOTE: This works for reserved seats. The application logic must handle GA inventory limits.
CREATE UNIQUE INDEX idx_unique_seat_per_event ON tickets(event_id, seat_id) WHERE seat_id IS NOT NULL;

-- Index for quickly finding all tickets in a specific order.
CREATE INDEX idx_tickets_order_id ON tickets(order_id);

-- Index for quickly finding all tickets belonging to a user.
CREATE INDEX idx_tickets_user_id ON tickets(user_id);