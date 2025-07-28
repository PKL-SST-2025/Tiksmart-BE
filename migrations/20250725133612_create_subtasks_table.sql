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


-- Subtasks Table
-- A list of child items for a parent task.
CREATE TABLE subtasks (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,
    task_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE
);

-- #############################################################################
-- ### INDEXES for Performance
-- #############################################################################

-- Indexes are automatically created for PRIMARY KEY and UNIQUE constraints.
-- We add them for foreign keys and frequently queried columns.

-- Subtasks
CREATE INDEX idx_subtasks_task_id ON subtasks(task_id);
