-- migrations/YYYYMMDDHHMMSS_create_payments_table/down.sql

-- Drop the table first.
DROP TABLE IF EXISTS payments;

-- Then, drop the custom ENUM type.
DROP TYPE IF EXISTS payment_status;

-- migrations/YYYYMMDDHHMMSS_create_payments_table/up.sql

-- An ENUM for the lifecycle of a payment transaction.
CREATE TYPE payment_status AS ENUM (
    'pending',      -- Payment initiated but not yet confirmed.
    'succeeded',    -- The payment was successful.
    'failed',       -- The payment failed.
    'refunded'      -- The payment was successfully refunded (partially or fully).
);

-- The payments table, acting as a ledger for all transactions.
CREATE TABLE payments (
    -- A UUID for the primary key is a good practice for payment records.
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Foreign key to the order this payment is for. An order should have one successful payment.
    order_id UUID NOT NULL UNIQUE REFERENCES orders(id) ON DELETE RESTRICT,

    -- The current status of this payment attempt.
    status payment_status NOT NULL DEFAULT 'pending',

    -- Financial Details
    amount_charged DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'usd', -- ISO 4217 currency code.
    amount_refunded DECIMAL(10, 2) NOT NULL DEFAULT 0.00,

    -- ### STRIPE-SPECIFIC FIELDS ###
    -- This is the most important field. It stores the PaymentIntent ID (e.g., 'pi_...').
    -- It's the primary link to the transaction in your Stripe dashboard.
    stripe_payment_intent_id VARCHAR(255) UNIQUE NOT NULL,

    -- Optional, but highly recommended. Stores the Stripe Customer ID (e.g., 'cus_...').
    -- Useful for saved payment methods and repeat customers.
    stripe_customer_id VARCHAR(255),

    -- The payment method used (e.g., 'card', 'apple_pay'). Stripe provides this.
    payment_method_type VARCHAR(50),

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ### INDEXES for Performance ###
-- The UNIQUE constraint on order_id and stripe_payment_intent_id already creates indexes.
-- An index on the Stripe Customer ID can be useful for finding all payments by a Stripe customer.
CREATE INDEX idx_payments_stripe_customer_id ON payments(stripe_customer_id);

-- Create a trigger to automatically update 'last_updated' on modification.
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON payments
FOR EACH ROW
EXECUTE PROCEDURE update_last_updated_column();