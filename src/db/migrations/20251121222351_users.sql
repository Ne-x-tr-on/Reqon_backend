CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name VARCHAR(150) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    phone VARCHAR(20),
    employee_number VARCHAR(50) UNIQUE NOT NULL,
    dob DATE NOT NULL,       -- user enters this
    age INT GENERATED ALWAYS AS (DATE_PART('year', AGE(dob))) STORED, -- auto-calculated
    gender VARCHAR(10),

    department_id INT REFERENCES departments(id),
    role_id INT REFERENCES roles(id),

    created_at TIMESTAMP DEFAULT NOW()
);
