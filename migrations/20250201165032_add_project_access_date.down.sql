-- Add down migration script here
ALTER TABLE ProjectUser DROP COLUMN accessed_at TIMESTAMP;
