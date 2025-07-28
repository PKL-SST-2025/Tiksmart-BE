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

-- Members Table
-- Links a user to a project, effectively making them a member.
CREATE TABLE members (
    id SERIAL PRIMARY KEY, -- Own primary key for easy referencing
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES roles(id) ON DELETE SET NULL, -- If a role is deleted, the member remains but without that role.
    full_name VARCHAR(255) NOT NULL, -- Stored here for project-specific context
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,

    -- A user can only be a member of a project once.
    UNIQUE(user_id, project_id)
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.

-- Members
CREATE INDEX idx_members_user_id ON members(user_id);
CREATE INDEX idx_members_project_id ON members(project_id);
CREATE INDEX idx_members_role_id ON members(role_id);
