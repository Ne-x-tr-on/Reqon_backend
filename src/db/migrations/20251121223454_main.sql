-- Add migration script here
-- Core schema for global entities
CREATE SCHEMA core;

-- Users table, Roles, Departments
CREATE TABLE core.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(150) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,  -- hashed password
    phone VARCHAR(20),
    employee_number VARCHAR(50) UNIQUE NOT NULL,
    dob DATE NOT NULL,
    gender VARCHAR(10),
    department_id SMALLINT REFERENCES core.departments(id),
    role_id SMALLINT REFERENCES core.roles(id),
    created_at TIMESTAMP DEFAULT NOW()
);



-- Roles
CREATE TABLE core.roles (
    id SMALLINT PRIMARY KEY,    -- small number for efficiency
    name VARCHAR(50) UNIQUE NOT NULL
);


-- Departments
CREATE TABLE core.departments (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);


-- Separate schema for each department forms
CREATE SCHEMA safety;
CREATE SCHEMA production;
CREATE SCHEMA engineering;
