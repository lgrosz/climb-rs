-- Your SQL goes here
CREATE TABLE climb_link_ups (
  root_id INT NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
  link_id INT NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
  link_order INT NOT NULL,
  PRIMARY KEY (root_id, link_id),
  CONSTRAINT no_self_reference CHECK (root_id <> link_id),
  CONSTRAINT valid_order CHECK (link_order > 0)
);

