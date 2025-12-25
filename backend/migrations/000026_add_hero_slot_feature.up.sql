-- Add hero_slot to gold_feature enum
-- Note: Can't use the new value in the same transaction, so INSERT is in migration 27
ALTER TYPE gold_feature ADD VALUE IF NOT EXISTS 'hero_slot';
