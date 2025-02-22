-- Add up migration script here
ALTER TABLE ProjectUser ADD COLUMN accessed_at TIMESTAMP;
