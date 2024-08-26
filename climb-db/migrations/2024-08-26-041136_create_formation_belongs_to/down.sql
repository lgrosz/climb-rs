-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "formation_belongs_to";
DROP TRIGGER IF EXISTS trigger_prevent_cycle ON formation_belongs_to;
DROP FUNCTION IF EXISTS prevent_cycle();

