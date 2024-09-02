-- Your SQL goes here
CREATE TABLE climb_variations (
  root_id INT NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
  variation_id INT NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
  PRIMARY KEY (root_id, variation_id),
  CONSTRAINT no_self_reference CHECK (root_id <> variation_id)
);

