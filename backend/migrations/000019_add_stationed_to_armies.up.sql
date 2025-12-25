-- Add is_stationed column to armies table for support missions
-- When support troops arrive, they stay stationed at the target village

ALTER TABLE armies ADD COLUMN is_stationed BOOLEAN DEFAULT FALSE;

-- Index for finding stationed troops at a village
CREATE INDEX idx_armies_stationed ON armies(to_village_id, is_stationed) WHERE is_stationed = TRUE;

-- Index for finding support sent by a player
CREATE INDEX idx_armies_player_stationed ON armies(player_id, is_stationed) WHERE is_stationed = TRUE;
