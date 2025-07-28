-- Add migration script here

-- An ENUM type for task statuses for better data integrity and performance.
CREATE TYPE task_status AS ENUM (
    'To Do',
    'In Progress',
    'In Review',
    'Blocked',
    'Done'
);


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

-- Tasks Table
-- The central table for tasks within a project.
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'To Do',
    lead_member_id INTEGER REFERENCES members(id) ON DELETE SET NULL, -- If a lead member is removed, the task becomes unassigned.
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    archived_on TIMESTAMPTZ -- If NULL, the task is active. If not NULL, it's archived.
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.

-- Tasks
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_lead_member_id ON tasks(lead_member_id);
CREATE INDEX idx_tasks_status ON tasks(status);
