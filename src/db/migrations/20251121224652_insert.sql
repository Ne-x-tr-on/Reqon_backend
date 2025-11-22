-- Add migration script here
-- Roles
CREATE TABLE core.roles (
    id SMALLINT PRIMARY KEY,    -- small number for efficiency
    name VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO core.roles (id, name) VALUES
(1, 'employee'),
(2, 'supervisor'),
(3, 'admin'),
(4, 'superadmin');

-- Departments
CREATE TABLE core.departments (
    id SMALLINT PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO core.departments (id, name) VALUES
(1, 'Production'),
(2, 'Safety'),
(3, 'Engineering'),
(4, 'Finance'),
(5, 'QA'),
(6, 'HR');
