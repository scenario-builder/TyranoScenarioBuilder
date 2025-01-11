-- Add up migration script here
CREATE TABLE "ConstantCategory" (
  id INTEGER PRIMARY KEY,
  display_name TEXT NOT NULL,
  project_id INTEGER NOT NULL,
  owner_category_id INTEGER,
  FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE SET NULL,
  FOREIGN KEY (owner_category_id) REFERENCES ConstantCategory(id) ON DELETE SET NULL
);

CREATE TABLE "ConstantValue" (
  id INTEGER PRIMARY KEY,
  program_name TEXT NOT NULL,
  owner_category_id INTEGER NOT NULL,
  FOREIGN KEY (owner_category_id) REFERENCES ConstantCategory(id) ON DELETE SET NULL
);
