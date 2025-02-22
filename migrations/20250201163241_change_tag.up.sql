-- Add up migration script here
ALTER TABLE TagCategory RENAME TO TagType;

ALTER TABLE Tag ADD COLUMN category_name TEXT;
