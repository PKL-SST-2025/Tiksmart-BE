-- migrations/YYYYMMDDHHMMSS_create_ticket_tiers_and_offers/down.sql

-- Drop tables in reverse order of dependency.
DROP TABLE IF EXISTS offers;
DROP TABLE IF EXISTS ticket_tiers;

-- Finally, drop the custom ENUM type.
DROP TYPE IF EXISTS offer_status;

-- migrations/YYYYMMDDHHMMSS_create_ticket_tiers_and_offers/up.sql

-- ENUM for the status of a specific sales offer.
CREATE TYPE offer_status AS ENUM ('scheduled', 'on_sale', 'paused', 'sold_out', 'ended');

-- Represents a type/section of ticket for a specific event (e.g., "General Admission Floor").
-- This table holds the master inventory count.
CREATE TABLE ticket_tiers (
    id SERIAL PRIMARY KEY,
    event_id INT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    total_inventory INT NOT NULL,

    -- A tier name should be unique for a given event.
    UNIQUE(event_id, name)
);

-- Represents a specific offer to purchase tickets from a tier.
-- An event can have many offers (e.g., Presale, General On-sale, VIP Package).
CREATE TABLE offers (
    id SERIAL PRIMARY KEY,
    ticket_tier_id INT NOT NULL REFERENCES ticket_tiers(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    status offer_status NOT NULL DEFAULT 'scheduled',

    -- Pricing and inventory for this specific offer.
    price DECIMAL(10, 2) NOT NULL,
    quantity_for_sale INT NOT NULL, -- How many tickets are allocated to this offer from the tier's total inventory.
    quantity_sold INT NOT NULL DEFAULT 0,

    -- Sales window for this offer.
    sale_start_time TIMESTAMPTZ,
    sale_end_time TIMESTAMPTZ,

    -- Purchase rules
    min_per_order INT NOT NULL DEFAULT 1,
    max_per_order INT NOT NULL DEFAULT 8,
    access_code VARCHAR(255) -- For presales or special access.
);


-- ### INDEXES ###
CREATE INDEX idx_ticket_tiers_event_id ON ticket_tiers(event_id);
CREATE INDEX idx_offers_ticket_tier_id ON offers(ticket_tier_id);
-- Index for efficiently finding currently active offers.
CREATE INDEX idx_offers_status_sale_times ON offers(status, sale_start_time, sale_end_time);