-- Your SQL goes here
CREATE TABLE formation_belongs_to (
    formation_id INTEGER PRIMARY KEY REFERENCES formations(id) ON DELETE CASCADE,
    area_id INTEGER,
    super_formation_id INTEGER,
    CONSTRAINT fk_areas FOREIGN KEY (area_id) REFERENCES areas(id) ON DELETE RESTRICT,
    CONSTRAINT fk_formations FOREIGN KEY (super_formation_id) REFERENCES formations(id) ON DELETE RESTRICT,
    CHECK (
        (super_formation_id IS NOT NULL AND area_id IS NULL) OR
        (super_formation_id IS NULL AND area_id IS NOT NULL)
    )
);

CREATE FUNCTION prevent_cycle() RETURNS trigger AS $$
DECLARE
    v_parent_id INTEGER;
BEGIN
    v_parent_id := NEW.super_formation_id;

    WHILE v_parent_id IS NOT NULL LOOP
        IF v_parent_id = NEW.formation_id THEN
            RAISE EXCEPTION 'Cycle detected: formation cannot be its own ancestor';
        END IF;
        SELECT super_formation_id INTO v_parent_id
        FROM formation_belongs_to
        WHERE formation_id = v_parent_id;
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_prevent_cycle
BEFORE INSERT OR UPDATE ON formation_belongs_to
FOR EACH ROW EXECUTE FUNCTION prevent_cycle();
