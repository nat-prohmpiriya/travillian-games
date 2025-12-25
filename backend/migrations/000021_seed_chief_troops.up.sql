-- Insert Chief troop definitions
-- These troops can reduce loyalty when sent on Conquer missions

INSERT INTO troop_definitions (
    troop_type, tribe, name, description,
    attack, defense_infantry, defense_cavalry, speed, carry_capacity, crop_consumption,
    training_time_seconds, wood_cost, clay_cost, iron_cost, crop_cost,
    required_building, required_building_level, loyalty_reduction
) VALUES
-- Phasuttha Chief: Royal Advisor (ราชมนตรี)
(
    'royal_advisor', 'phasuttha', 'Royal Advisor',
    'A respected court official who can persuade villagers to change allegiance. Reduces enemy village loyalty.',
    40, 30, 25, 4, 0, 4,
    18000, 30750, 27200, 25000, 27250,
    'academy', 15, 25
),
-- Nava Chief: Harbor Master (นายท่า)
(
    'harbor_master', 'nava', 'Harbor Master',
    'A powerful maritime leader who controls trade routes. Can convince coastal villages to surrender.',
    35, 40, 30, 5, 0, 4,
    16200, 28000, 24500, 22000, 25500,
    'academy', 15, 22
),
-- Kiri Chief: Elder Chief (ผู้เฒ่า)
(
    'elder_chief', 'kiri', 'Elder Chief',
    'A wise mountain elder whose words carry great weight. Skilled at undermining enemy morale.',
    30, 35, 35, 4, 0, 4,
    19800, 32000, 28000, 26000, 28000,
    'academy', 15, 28
);
