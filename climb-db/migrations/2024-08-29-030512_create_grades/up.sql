-- Your SQL goes here
CREATE TABLE grade_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

INSERT INTO grade_types (name) VALUES
('vermin');

CREATE TABLE grades (
    id SERIAL PRIMARY KEY,
    grade_type_id INTEGER NOT NULL REFERENCES grade_types(id) ON DELETE CASCADE,
    value VARCHAR(50) NOT NULL,
    UNIQUE (grade_type_id, value)
);

CREATE TABLE climb_grades (
    climb_id INTEGER NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
    grade_id INTEGER NOT NULL REFERENCES grades(id) ON DELETE CASCADE,
    PRIMARY KEY (climb_id, grade_id)
);

-- NOTE `grades` has the potential to generate many orphans as rows in
-- `climb_grades` are removed.
