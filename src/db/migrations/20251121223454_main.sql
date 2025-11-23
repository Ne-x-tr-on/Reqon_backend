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

CREATE TABLE core.forms (
    id SERIAL PRIMARY KEY,
    department_id SMALLINT REFERENCES core.departments(id),
    name VARCHAR(150) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE core.form_fields (
    id SERIAL PRIMARY KEY,
    form_id INT REFERENCES core.forms(id) ON DELETE CASCADE,
    field_name VARCHAR(150) NOT NULL,
    field_type VARCHAR(50) NOT NULL, -- e.g., text, number, date
    required BOOLEAN DEFAULT TRUE
);


CREATE TABLE core.form_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id INT REFERENCES core.forms(id),
    submitted_by UUID REFERENCES core.users(id),
    submitted_at TIMESTAMP DEFAULT NOW(),
    status VARCHAR(50) DEFAULT 'pending' -- pending, approved, rejected
);

CREATE TABLE core.form_submission_values (
    id SERIAL PRIMARY KEY,
    submission_id UUID REFERENCES core.form_submissions(id) ON DELETE CASCADE,
    field_id INT REFERENCES core.form_fields(id),
    value TEXT
);


CREATE TABLE core.submission_actions (
    id SERIAL PRIMARY KEY,
    submission_id UUID REFERENCES core.form_submissions(id) ON DELETE CASCADE,
    action_by UUID REFERENCES core.users(id),
    action_type VARCHAR(50), -- approved, rejected
    comment TEXT,
    action_at TIMESTAMP DEFAULT NOW()
);


CREATE TABLE core.audit_logs (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES core.users(id),
    action VARCHAR(150),
    target_table VARCHAR(50),
    target_id UUID,
    created_at TIMESTAMP DEFAULT NOW()
);











-- Departments
CREATE TABLE core.departments (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);








CREATE TABLE core.users AS TABLE core_v2.users;

