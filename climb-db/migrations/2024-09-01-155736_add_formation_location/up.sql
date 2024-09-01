-- Your SQL goes here
CREATE EXTENSION postgis;

ALTER TABLE formations
	ADD COLUMN location geometry(POINT, 4326);
