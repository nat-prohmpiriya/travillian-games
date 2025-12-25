-- Remove is_stationed column from armies table

DROP INDEX IF EXISTS idx_armies_player_stationed;
DROP INDEX IF EXISTS idx_armies_stationed;
ALTER TABLE armies DROP COLUMN IF EXISTS is_stationed;
