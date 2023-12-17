-- Add up migration script here
CREATE TABLE IF NOT EXISTS employee_role_junction(
    employee_id UUID REFERENCES users(id),
    role_id SMALLINT REFERENCES roles(role_id),
    CONSTRAINT employee_role_junction_pk PRIMARY KEY(employee_id, role_id)
);