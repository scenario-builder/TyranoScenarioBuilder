-- Add up migration script here
CREATE TABLE "Scenario" (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  project_id INTEGER,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE SET NULL
);

CREATE TABLE "ScenarioTag" (
  id INTEGER PRIMARY KEY,
  scenario_id INTEGER,
  tag_id INTEGER,
  front_scenario_tag_id INTEGER,
  FOREIGN KEY (scenario_id) REFERENCES Scenario(id) ON DELETE SET NULL,
  FOREIGN KEY (tag_id) REFERENCES Tag(id) ON DELETE SET NULL,
  FOREIGN KEY (front_scenario_tag_id) REFERENCES ScenarioTag(id) ON DELETE SET NULL
);

CREATE TABLE "InputContent" (
  id INTEGER PRIMARY KEY,
  input_id INTEGER NOT NULL,
  scenario_tag_id INTEGER,
  text TEXT,
  number INTEGER,
  boolean BOOLEAN,
  constant_value_id INTEGER,
  FOREIGN KEY (input_id) REFERENCES Input(id) ON DELETE SET NULL,
  FOREIGN KEY (scenario_tag_id) REFERENCES ScenarioTag(id) ON DELETE SET NULL,
  FOREIGN KEY (constant_value_id) REFERENCES ConstantValue(id) ON DELETE SET NULL
);
