-- Add migration script here
CREATE TABLE departments (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) UNIQUE NOT NULL
);
