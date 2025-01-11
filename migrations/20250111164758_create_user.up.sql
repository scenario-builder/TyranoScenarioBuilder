-- Add up migration script here
CREATE TABLE "User" (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  mail TEXT NOT NULL,
  password VARCHAR(64) NOT NULL,
  is_changed BOOLEAN NOT NULL DEFAULT FALSE
);
