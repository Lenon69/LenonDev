-- migrations/{timestamp}_create_projects_table.sql
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    technologies VARCHAR(255) NOT NULL,
    image_url VARCHAR(255),
    project_url VARCHAR(255)
);-- Add migration script here
