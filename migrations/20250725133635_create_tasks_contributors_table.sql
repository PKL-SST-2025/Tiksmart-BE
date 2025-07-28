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

-- TasksContributors Table (Many-to-Many Join Table)
-- Tracks which members are contributing to a task.
CREATE TABLE tasks_contributors (
    id SERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    member_id INTEGER NOT NULL REFERENCES members(id) ON DELETE CASCADE,

    -- A member can only be a contributor to a task once.
    UNIQUE(task_id, member_id)
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.


CREATE INDEX idx_tasks_contributors_task_id ON tasks_contributors(task_id);
CREATE INDEX idx_tasks_contributors_member_id ON tasks_contributors(member_id);