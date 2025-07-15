-- migrations/{timestamp}_create_project_images_table.sql
CREATE TABLE project_images (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    image_url VARCHAR(255) NOT NULL
);
