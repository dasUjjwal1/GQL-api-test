-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users(
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    photo VARCHAR(255) NOT NULL DEFAULT 'default.png',
    status BOOLEAN DEFAULT TRUE,
    mobile VARCHAR(100),
    personal_email VARCHAR(255),
    gender SMALLINT,
    dob DATE,
    employee_id VARCHAR(100),
    address jsonb,
    qualification jsonb,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX user_email_idx ON users (email);