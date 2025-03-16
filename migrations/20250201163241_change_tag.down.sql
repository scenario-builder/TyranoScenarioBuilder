-- Add down migration script here
ALTER TABLE TagType RENAME TO TagCategory;

ALTER TABLE Tag DROP COLUMN category_name;
