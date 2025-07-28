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

-- Roles Table
-- Defines roles specific to each project (e.g., 'Frontend Developer', 'QA Tester').
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    role VARCHAR(100) NOT NULL,
    description TEXT,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    -- A role name must be unique within a single project.
    UNIQUE(project_id, role)
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.

-- Roles
CREATE INDEX idx_roles_project_id ON roles(project_id);