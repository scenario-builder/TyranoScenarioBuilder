-- Add up migration script here
CREATE TABLE InputType (
  id INTEGER PRIMARY KEY,
  type TEXT NOT NULL
);

CREATE TABLE Input (
  id INTEGER PRIMARY KEY,
  sort_index INTEGER NOT NULL,
  title TEXT NOT NULL,
  tag_id INTEGER,
  input_type_id INTEGER,
  constant_group_id INTEGER,
  FOREIGN KEY (tag_id) REFERENCES Tag(id) ON DELETE SET NULL,
  FOREIGN KEY (input_type_id) REFERENCES InputType(id) ON DELETE SET NULL,
  FOREIGN KEY (constant_group_id) REFERENCES ConstantGroup(id) ON DELETE SET NULL
);
