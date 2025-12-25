-- Add hero_slot to gold_feature_costs (after enum value was committed in 000026)
INSERT INTO gold_feature_costs (feature, base_cost, description) VALUES
    ('hero_slot', 0, 'Cost varies by slot number')
ON CONFLICT (feature) DO NOTHING;
