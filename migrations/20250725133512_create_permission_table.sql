-- A trigger function to automatically update the 'last_updated' timestamp
-- on any row modification.
CREATE OR REPLACE FUNCTION update_last_updated_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.last_updated = now();
   RETURN NEW;
END;
$$ language 'plpgsql';


-- #############################################################################
-- ### TABLE CREATION
-- #############################################################################
-- Add migration script here

-- Permission Tier Table
-- A lookup table for system-wide permission levels.
CREATE TABLE permission_tier (
    id SERIAL PRIMARY KEY,
    permission VARCHAR(50) UNIQUE NOT NULL -- e.g., 'Admin', 'Editor', 'Viewer'
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.
