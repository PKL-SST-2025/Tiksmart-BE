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

-- TasksRequiredRoles Table (Many-to-Many Join Table)
-- Specifies which roles are required or suggested for a task.
CREATE TABLE tasks_required_roles (
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,

    -- Prevent duplicate role requirements for the same task.
    UNIQUE(task_id, role_id)
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.


-- Join Tables
CREATE INDEX idx_tasks_required_roles_task_id ON tasks_required_roles(task_id);
CREATE INDEX idx_tasks_required_roles_role_id ON tasks_required_roles(role_id);
