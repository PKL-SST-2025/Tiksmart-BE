-- migrations/YYYYMMDDHHMMSS_add_stripe_account_id_to_organizers/up.sql


ALTER TABLE organizer_profiles
DROP COLUMN IF EXISTS stripe_account_id;

-- Add a column to store the organizer's Stripe Connect Account ID.
-- It's nullable because an organizer might sign up on your platform
-- before completing the Stripe onboarding process.
ALTER TABLE organizer_profiles
ADD COLUMN stripe_account_id VARCHAR(255) UNIQUE;

-- Add an index for quick lookups.
CREATE INDEX idx_organizer_profiles_stripe_account_id ON organizer_profiles(stripe_account_id);