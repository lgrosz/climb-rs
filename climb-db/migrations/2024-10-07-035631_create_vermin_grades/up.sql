-- Your SQL goes here
CREATE TABLE climb_vermin_grades (
    climb_id INTEGER NOT NULL REFERENCES climbs(id) ON DELETE cascade,
    value INTEGER NOT NULL CHECK (value >= 0),
    PRIMARY KEY (climb_id, value)
);
