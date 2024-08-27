-- Your SQL goes here
CREATE TABLE area_belongs_to (
    area_id INTEGER PRIMARY KEY REFERENCES areas(id) ON DELETE CASCADE,
    super_area_id INTEGER NOT NULL,
    CONSTRAINT fk_areas FOREIGN KEY (super_area_id) REFERENCES areas(id) ON DELETE RESTRICT
);

CREATE FUNCTION prevent_area_belongs_to_cycle() RETURNS trigger AS $$
DECLARE
    v_parent_id INTEGER;
BEGIN
    v_parent_id := NEW.super_area_id;

    WHILE v_parent_id IS NOT NULL LOOP
        IF v_parent_id = NEW.area_id THEN
            RAISE EXCEPTION 'Cycle detected: area cannot be its own ancestor';
        END IF;
        SELECT super_area_id INTO v_parent_id
        FROM area_belongs_to
        WHERE area_id = v_parent_id;
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_prevent_area_belongs_to_cycle
BEFORE INSERT OR UPDATE ON area_belongs_to
FOR EACH ROW EXECUTE FUNCTION prevent_area_belongs_to_cycle();

