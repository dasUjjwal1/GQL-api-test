-- Add up migration script here
CREATE TABLE IF NOT EXISTS roles(
    role_id SMALLINT NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    role_name VARCHAR(255) NOT NULL,
    parent SMALLINT
)