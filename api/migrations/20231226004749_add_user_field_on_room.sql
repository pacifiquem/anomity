-- Add migration script here
ALTER TABLE rooms ADD COLUMN user_id uuid REFERENCES users(id) ON DELETE CASCADE;