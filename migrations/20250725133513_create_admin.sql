-- migrations/create_admins_table/down.sql

-- Drop the table first, as it depends on the admin_role type.
DROP TABLE IF EXISTS admins;

-- Then, drop the custom ENUM type.
DROP TYPE IF EXISTS admin_role;

-- migrations/YYYYMMDDHHMMSS_create_admins_table/up.sql

-- First, create an ENUM type for different administrator roles.
CREATE TYPE admin_role AS ENUM ('superadmin', 'moderator');

-- Create the admins table for users with elevated privileges.
-- This table is separate from the general 'users' table.
CREATE TABLE admins (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,

    -- Role-based access control
    role admin_role NOT NULL DEFAULT 'moderator',

    -- Metadata
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);