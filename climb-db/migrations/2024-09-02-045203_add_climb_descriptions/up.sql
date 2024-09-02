-- Your SQL goes here
CREATE TABLE climb_description_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

INSERT INTO climb_description_types (name) VALUES
    ('brief'),
    ('desc'),
    ('hist');

CREATE TABLE climb_descriptions (
    climb_id INTEGER NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
    climb_description_type_id INTEGER NOT NULL REFERENCES climb_description_types(id) ON DELETE CASCADE,
    value TEXT NOT NULL,
    PRIMARY KEY (climb_id, climb_description_type_id)
);

