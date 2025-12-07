CREATE TYPE building_type AS ENUM (
    'main_building', 'warehouse', 'granary', 'barracks', 'stable',
    'elephant_ground', 'workshop', 'academy', 'smithy', 'market',
    'embassy', 'palace', 'residence', 'wall', 'rally_point',
    'cranny', 'hero_mansion', 'tavern', 'duck_farm',
    'woodcutter', 'clay_pit', 'iron_mine', 'crop_field'
);

CREATE TABLE buildings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    building_type building_type NOT NULL,
    slot INT NOT NULL, -- Position in village (0-39)
    level INT DEFAULT 0,
    is_upgrading BOOLEAN DEFAULT FALSE,
    upgrade_started_at TIMESTAMPTZ,
    upgrade_ends_at TIMESTAMPTZ,
    UNIQUE(village_id, slot)
);

CREATE INDEX idx_buildings_village ON buildings(village_id);
CREATE INDEX idx_buildings_upgrading ON buildings(is_upgrading) WHERE is_upgrading = TRUE;

CREATE TABLE building_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    target_level INT NOT NULL,
    position INT NOT NULL, -- Queue position
    wood_cost INT NOT NULL,
    clay_cost INT NOT NULL,
    iron_cost INT NOT NULL,
    crop_cost INT NOT NULL,
    duration_seconds INT NOT NULL,
    started_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
