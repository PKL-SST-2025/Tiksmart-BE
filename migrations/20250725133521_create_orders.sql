-- migrations/YYYYMMDDHHMMSS_create_orders_table/down.sql

-- Drop the table first, as it depends on the order_status type.
DROP TABLE IF EXISTS orders;

-- Then, drop the custom ENUM type.
DROP TYPE IF EXISTS order_status;

-- migrations/YYYYMMDDHHMMSS_create_orders_table/up.sql

-- First, ensure the UUID extension is enabled.
-- Most modern Postgres instances have this by default.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- An ENUM type for the lifecycle status of an order.
CREATE TYPE order_status AS ENUM (
    'pending',      -- The order has been created, but payment is not complete (cart state).
    'completed',    -- Payment was successful, and tickets are issued.
    'failed',       -- Payment failed.
    'cancelled',    -- The user or system cancelled the order before completion.
    'refunded'      -- The order was completed but later refunded.
);

-- The orders table, representing a single transaction.
CREATE TABLE orders (
    -- Use a UUID for the primary key. It's not guessable like a serial integer.
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Foreign key to the user who placed the order.
    -- ON DELETE RESTRICT prevents deleting a user who has order history, which is good for record-keeping.
    user_id INT NOT NULL REFERENCES users(id) ON DELETE RESTRICT,

    -- The current status of the order.
    status order_status NOT NULL DEFAULT 'pending',

    -- Financial Details
    subtotal DECIMAL(10, 2) NOT NULL,      -- The sum of all ticket prices.
    service_fee DECIMAL(10, 2) NOT NULL,   -- Your platform's fee.
    total_amount DECIMAL(10, 2) NOT NULL,  -- The final amount charged to the user.

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- This is crucial for managing locked seats. A 'pending' order is essentially a temporary reservation.
    -- If the order isn't completed by this time, a background job should cancel it and release the seats.
    expires_at TIMESTAMPTZ NOT NULL
);

-- ### INDEXES for Performance ###
-- Index for quickly finding all orders for a specific user.
CREATE INDEX idx_orders_user_id ON orders(user_id);

-- A partial index to efficiently find pending orders that have expired. This is critical for your cleanup worker.
CREATE INDEX idx_orders_pending_expires_at ON orders(status, expires_at) WHERE status = 'pending';

-- Create a trigger to automatically update 'last_updated' on modification.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON orders
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();