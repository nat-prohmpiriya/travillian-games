-- Remove hero_slot from gold_feature_costs
DELETE FROM gold_feature_costs WHERE feature = 'hero_slot';

-- Note: PostgreSQL does not support removing enum values directly
-- The hero_slot value will remain in the enum but be unused
