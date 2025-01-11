-- Add up migration script here
CREATE TABLE TagCategory (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);

CREATE TABLE Tag (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  export_rule TEXT NOT NULL,
  tag_category_id INTEGER NOT NULL,
  FOREIGN KEY (tag_category_id) REFERENCES TagCategory(id) ON DELETE SET NULL
);
