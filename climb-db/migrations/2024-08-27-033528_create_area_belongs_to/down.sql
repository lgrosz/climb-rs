-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "area_belongs_to";
DROP TRIGGER IF EXISTS trigger_prevent_area_belongs_to_cycle ON area_belongs_to;
DROP FUNCTION IF EXISTS prevent_area_belongs_to_cycle();

