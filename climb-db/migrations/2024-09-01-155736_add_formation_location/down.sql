-- This file should undo anything in `up.sql`
ALTER TABLE formations
	DROP COLUMN location;

DROP EXTENSION postgis;
