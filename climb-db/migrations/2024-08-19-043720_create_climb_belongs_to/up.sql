-- Your SQL goes here
CREATE TABLE climb_belongs_to (
    climb_id INTEGER PRIMARY KEY REFERENCES climbs(id) ON DELETE CASCADE,
    area_id INTEGER,
    formation_id INTEGER,
    CONSTRAINT fk_areas FOREIGN KEY (area_id) REFERENCES areas(id) ON DELETE RESTRICT,
    CONSTRAINT fk_formations FOREIGN KEY (formation_id) REFERENCES formations(id) ON DELETE RESTRICT,
    CHECK (
        (formation_id IS NOT NULL AND area_id IS NULL) OR
        (formation_id IS NULL AND area_id IS NOT NULL)
    )
);
