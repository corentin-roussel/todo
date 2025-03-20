CREATE DATABASE todo_list;

-- Switch to the todo_list database
\c todo_list;

-- Create a table for the todo items
CREATE TABLE todo_items (
    id SERIAL PRIMARY KEY,  -- Unique identifier for each todo item
    title VARCHAR(255) NOT NULL,  -- Title of the todo item
    description TEXT,  -- Description of the todo item (optional)
    is_completed BOOLEAN DEFAULT FALSE,  -- Whether the item is completed or not
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- Timestamp when the item was created
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- Timestamp when the item was last updated
);
