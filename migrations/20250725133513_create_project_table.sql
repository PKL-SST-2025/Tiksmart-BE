-- Add migration script here

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

-- Projects Table
-- A project is the top-level container.
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    project_name VARCHAR(255) NOT NULL,
    business_name VARCHAR(255),
    description TEXT,
    owner_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT, -- A project must have an owner
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.

-- Projects
CREATE INDEX idx_projects_owner_user_id ON projects(owner_user_id);
