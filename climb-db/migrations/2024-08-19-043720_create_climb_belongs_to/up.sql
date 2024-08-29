-- Your SQL goes here
CREATE TABLE climb_belongs_to (
    climb_id INTEGER PRIMARY KEY REFERENCES climbs(id) ON DELETE CASCADE,
    area_id INTEGER REFERENCES areas(id) ON DELETE RESTRICT,
    formation_id INTEGER REFERENCES formations(id) ON DELETE RESTRICT,
    CHECK (
        (formation_id IS NOT NULL AND area_id IS NULL) OR
        (formation_id IS NULL AND area_id IS NOT NULL)
    )
);
