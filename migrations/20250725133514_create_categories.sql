-- migrations/YYYYMMDDHHMMSS_create_event_categories/down.sql

-- Drop tables in reverse order of dependency.
DROP TABLE IF EXISTS sub_genres;
DROP TABLE IF EXISTS genres;
DROP TABLE IF EXISTS segments;

-- migrations/YYYYMMDDHHMMSS_create_event_categories/up.sql

-- The top-level category, e.g., 'Music', 'Sports', 'Arts & Theater'.
CREATE TABLE segments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- The second-level category, which belongs to a Segment.
-- e.g., 'Rock' (belongs to 'Music'), 'Football' (belongs to 'Sports').
CREATE TABLE genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    -- Foreign key linking this genre to a parent segment.
    segment_id INT NOT NULL REFERENCES segments(id) ON DELETE CASCADE,
    -- Ensure a genre name is unique within its segment.
    -- (e.g., you can have 'Folk' in Music and 'Folk' in Arts, but not two 'Folk' in Music).
    UNIQUE(segment_id, name)
);

-- The third-level category, which belongs to a Genre.
-- e.g., 'Alternative Rock' (belongs to 'Rock').
CREATE TABLE sub_genres (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    -- Foreign key linking this sub-genre to a parent genre.
    genre_id INT NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
    -- Ensure a sub-genre name is unique within its genre.
    UNIQUE(genre_id, name)
);