-- Remove Chief troops
DELETE FROM troop_definitions WHERE troop_type IN ('royal_advisor', 'harbor_master', 'elder_chief');

-- Remove loyalty_reduction column
ALTER TABLE troop_definitions DROP COLUMN IF EXISTS loyalty_reduction;

-- Note: Cannot remove enum values in PostgreSQL without recreating the type
-- The enum values will remain but the troops are deleted
