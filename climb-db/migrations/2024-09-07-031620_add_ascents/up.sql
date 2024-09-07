-- Your SQL goes here
CREATE TABLE climbers (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL
);

CREATE TABLE ascents (
    id SERIAL PRIMARY KEY,
    climb_id INT NOT NULL REFERENCES climbs(id) ON DELETE CASCADE,
    ascent_date DATERANGE
);

CREATE TABLE ascent_parties (
    ascent_id INT NOT NULL REFERENCES ascents(id) ON DELETE CASCADE,
    climber_id INT NOT NULL REFERENCES ascents(id) ON DELETE CASCADE,
    PRIMARY KEY (ascent_id, climber_id)
);

CREATE TABLE ascent_grades (
    ascent_id INT NOT NULL REFERENCES ascents(id) ON DELETE CASCADE,
    grade_id INT NOT NULL REFERENCES grades(id) ON DELETE CASCADE,
    PRIMARY KEY (ascent_id, grade_id)
);
