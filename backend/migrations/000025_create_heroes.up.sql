-- Hero System with Option B (Multiple Heroes with Slots)

-- Add hero slots to users (start with 1, can buy more)
ALTER TABLE users ADD COLUMN IF NOT EXISTS hero_slots INTEGER NOT NULL DEFAULT 1;

-- Hero status enum
CREATE TYPE hero_status AS ENUM ('idle', 'moving', 'in_adventure', 'in_battle', 'dead', 'reviving');

-- Heroes table
CREATE TABLE heroes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    slot_number INTEGER NOT NULL CHECK (slot_number >= 1 AND slot_number <= 5),

    -- Basic info
    name VARCHAR(50) NOT NULL,
    tribe tribe_type NOT NULL,

    -- Location
    home_village_id UUID NOT NULL REFERENCES villages(id),
    current_village_id UUID REFERENCES villages(id),

    -- Status
    status hero_status NOT NULL DEFAULT 'idle',

    -- Stats
    level INTEGER NOT NULL DEFAULT 1,
    experience INTEGER NOT NULL DEFAULT 0,
    experience_to_next INTEGER NOT NULL DEFAULT 100,

    -- Health
    health INTEGER NOT NULL DEFAULT 100 CHECK (health >= 0 AND health <= 100),
    health_regen_rate DECIMAL(5,2) NOT NULL DEFAULT 10.0, -- HP per hour

    -- Attribute points
    unassigned_points INTEGER NOT NULL DEFAULT 0,

    -- Attributes (can be increased with points)
    fighting_strength INTEGER NOT NULL DEFAULT 0,    -- +80 attack power per point
    off_bonus INTEGER NOT NULL DEFAULT 0,            -- +0.2% attack bonus per point
    def_bonus INTEGER NOT NULL DEFAULT 0,            -- +0.2% defense bonus per point
    resources_bonus INTEGER NOT NULL DEFAULT 0,      -- +resource production per point

    -- Base stats (from tribe)
    base_attack INTEGER NOT NULL DEFAULT 80,
    base_defense INTEGER NOT NULL DEFAULT 80,
    base_speed DECIMAL(5,2) NOT NULL DEFAULT 7.0,

    -- Timestamps
    last_health_update TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    died_at TIMESTAMPTZ,
    revive_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(user_id, slot_number)
);

-- Index for hero lookups
CREATE INDEX idx_heroes_user ON heroes(user_id);
CREATE INDEX idx_heroes_home_village ON heroes(home_village_id);
CREATE INDEX idx_heroes_current_village ON heroes(current_village_id) WHERE current_village_id IS NOT NULL;
CREATE INDEX idx_heroes_status ON heroes(status) WHERE status != 'idle';

-- Item rarity enum
CREATE TYPE item_rarity AS ENUM ('common', 'uncommon', 'rare', 'epic', 'legendary');

-- Item slot enum
CREATE TYPE item_slot AS ENUM (
    'helmet',
    'weapon',
    'armor_left',
    'armor_right',
    'boots',
    'horse',
    'bag',
    'bandage',
    'consumable'
);

-- Item definitions (static data)
CREATE TABLE item_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    slot item_slot NOT NULL,
    rarity item_rarity NOT NULL DEFAULT 'common',

    -- Requirements
    required_level INTEGER NOT NULL DEFAULT 1,

    -- Stats bonuses
    attack_bonus INTEGER NOT NULL DEFAULT 0,
    defense_bonus INTEGER NOT NULL DEFAULT 0,
    speed_bonus DECIMAL(5,2) NOT NULL DEFAULT 0,
    health_regen_bonus DECIMAL(5,2) NOT NULL DEFAULT 0,
    experience_bonus INTEGER NOT NULL DEFAULT 0,          -- % bonus
    resource_bonus INTEGER NOT NULL DEFAULT 0,            -- % bonus to production
    carry_bonus INTEGER NOT NULL DEFAULT 0,               -- Extra carry capacity

    -- For consumables
    health_restore INTEGER NOT NULL DEFAULT 0,
    is_consumable BOOLEAN NOT NULL DEFAULT FALSE,

    -- For bags
    extra_inventory_slots INTEGER NOT NULL DEFAULT 0,

    -- Value
    sell_value INTEGER NOT NULL DEFAULT 0,                -- Silver value

    -- Availability
    can_drop_adventure BOOLEAN NOT NULL DEFAULT TRUE,
    can_buy_auction BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Hero items (inventory)
CREATE TABLE hero_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL REFERENCES heroes(id) ON DELETE CASCADE,
    item_definition_id UUID NOT NULL REFERENCES item_definitions(id),

    -- Item state
    is_equipped BOOLEAN NOT NULL DEFAULT FALSE,
    equipped_slot item_slot,

    -- For stackable items (consumables)
    quantity INTEGER NOT NULL DEFAULT 1,

    -- Timestamps
    obtained_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    equipped_at TIMESTAMPTZ
);

-- Index for hero items
CREATE INDEX idx_hero_items_hero ON hero_items(hero_id);
CREATE INDEX idx_hero_items_equipped ON hero_items(hero_id, is_equipped) WHERE is_equipped = TRUE;

-- Adventure difficulty enum
CREATE TYPE adventure_difficulty AS ENUM ('short', 'long');

-- Hero adventures
CREATE TABLE hero_adventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL REFERENCES heroes(id) ON DELETE CASCADE,

    -- Adventure details
    difficulty adventure_difficulty NOT NULL,

    -- Timing
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    duration_seconds INTEGER NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,

    -- Status
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,
    completed_at TIMESTAMPTZ,

    -- Rewards (set when completed)
    reward_experience INTEGER,
    reward_silver INTEGER,
    reward_resources JSONB,          -- {"wood": 100, "clay": 100, ...}
    reward_item_id UUID REFERENCES item_definitions(id),

    -- Damage taken
    health_lost INTEGER,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for adventures
CREATE INDEX idx_hero_adventures_hero ON hero_adventures(hero_id);
CREATE INDEX idx_hero_adventures_active ON hero_adventures(hero_id, ends_at)
    WHERE is_completed = FALSE;

-- Available adventures (generated periodically)
CREATE TABLE available_adventures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    difficulty adventure_difficulty NOT NULL,

    -- Duration range
    min_duration_seconds INTEGER NOT NULL,
    max_duration_seconds INTEGER NOT NULL,

    -- Reward hints
    potential_reward_type VARCHAR(50), -- 'resources', 'item', 'silver'
    potential_item_rarity item_rarity,

    -- Availability
    expires_at TIMESTAMPTZ NOT NULL,
    is_taken BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for available adventures
CREATE INDEX idx_available_adventures_user ON available_adventures(user_id, expires_at)
    WHERE is_taken = FALSE;

-- Hero slot prices (for shop)
CREATE TABLE hero_slot_prices (
    slot_number INTEGER PRIMARY KEY CHECK (slot_number >= 2 AND slot_number <= 5),
    gold_cost INTEGER NOT NULL
);

-- Seed hero slot prices
INSERT INTO hero_slot_prices (slot_number, gold_cost) VALUES
    (2, 100),
    (3, 200),
    (4, 400),
    (5, 800);

-- Seed item definitions
INSERT INTO item_definitions (name, description, slot, rarity, attack_bonus, defense_bonus, speed_bonus, health_regen_bonus, sell_value) VALUES
    -- Helmets
    ('Leather Cap', 'Basic leather protection', 'helmet', 'common', 0, 5, 0, 0.5, 10),
    ('Iron Helmet', 'Solid iron helmet', 'helmet', 'uncommon', 0, 15, 0, 1.0, 50),
    ('Steel Helmet', 'Reinforced steel helmet', 'helmet', 'rare', 0, 30, 0, 2.0, 200),
    ('Champion Helm', 'Helmet of champions', 'helmet', 'epic', 5, 50, 0, 3.0, 500),
    ('Dragon Helm', 'Legendary dragon scale helmet', 'helmet', 'legendary', 10, 80, 0, 5.0, 1500),

    -- Weapons
    ('Wooden Club', 'Simple wooden weapon', 'weapon', 'common', 10, 0, 0, 0, 10),
    ('Iron Sword', 'Standard iron sword', 'weapon', 'uncommon', 25, 0, 0, 0, 50),
    ('Steel Blade', 'Sharp steel blade', 'weapon', 'rare', 50, 0, 0, 0, 200),
    ('War Axe', 'Deadly war axe', 'weapon', 'epic', 80, 0, 0, 0, 500),
    ('Dragon Slayer', 'Legendary dragon slaying sword', 'weapon', 'legendary', 120, 10, 0, 0, 1500),

    -- Armor Left
    ('Leather Shield', 'Basic leather shield', 'armor_left', 'common', 0, 8, 0, 0, 10),
    ('Iron Shield', 'Solid iron shield', 'armor_left', 'uncommon', 0, 20, 0, 0, 50),
    ('Tower Shield', 'Large tower shield', 'armor_left', 'rare', 0, 40, 0, 0, 200),
    ('Guardian Shield', 'Shield of guardians', 'armor_left', 'epic', 0, 65, 0, 0, 500),
    ('Aegis Shield', 'Legendary aegis', 'armor_left', 'legendary', 0, 100, 0, 0, 1500),

    -- Armor Right
    ('Leather Vest', 'Basic leather vest', 'armor_right', 'common', 0, 8, 0, 0, 10),
    ('Chain Mail', 'Linked chain mail', 'armor_right', 'uncommon', 0, 20, 0, 0, 50),
    ('Plate Armor', 'Heavy plate armor', 'armor_right', 'rare', 0, 40, 0, 0, 200),
    ('Royal Armor', 'Armor of royalty', 'armor_right', 'epic', 0, 65, 0, 0, 500),
    ('Dragon Scale', 'Legendary dragon scale armor', 'armor_right', 'legendary', 0, 100, 0, 0, 1500),

    -- Boots
    ('Leather Boots', 'Basic leather boots', 'boots', 'common', 0, 3, 1.0, 0, 10),
    ('Travel Boots', 'Boots for traveling', 'boots', 'uncommon', 0, 5, 2.0, 0, 50),
    ('Swift Boots', 'Swift movement boots', 'boots', 'rare', 0, 8, 4.0, 0, 200),
    ('Wind Striders', 'Boots of the wind', 'boots', 'epic', 0, 12, 6.0, 0, 500),
    ('Hermes Sandals', 'Legendary speed sandals', 'boots', 'legendary', 0, 15, 10.0, 0, 1500),

    -- Horses
    ('Pony', 'Small but reliable', 'horse', 'common', 0, 0, 3.0, 0, 20),
    ('Horse', 'Standard war horse', 'horse', 'uncommon', 0, 0, 5.0, 0, 100),
    ('War Horse', 'Trained war horse', 'horse', 'rare', 5, 5, 7.0, 0, 400),
    ('Stallion', 'Powerful stallion', 'horse', 'epic', 10, 10, 10.0, 0, 1000),
    ('Pegasus', 'Legendary winged horse', 'horse', 'legendary', 15, 15, 15.0, 0, 3000);

-- Seed consumable items
INSERT INTO item_definitions (name, description, slot, rarity, health_restore, is_consumable, sell_value) VALUES
    ('Small Bandage', 'Restores 10 HP', 'consumable', 'common', 10, true, 5),
    ('Bandage', 'Restores 25 HP', 'consumable', 'uncommon', 25, true, 15),
    ('Large Bandage', 'Restores 50 HP', 'consumable', 'rare', 50, true, 40),
    ('Healing Salve', 'Restores 75 HP', 'consumable', 'epic', 75, true, 100),
    ('Elixir of Life', 'Fully restores HP', 'consumable', 'legendary', 100, true, 300);

-- Seed bag items
INSERT INTO item_definitions (name, description, slot, rarity, extra_inventory_slots, sell_value) VALUES
    ('Small Pouch', 'Adds 3 inventory slots', 'bag', 'common', 3, 20),
    ('Leather Bag', 'Adds 5 inventory slots', 'bag', 'uncommon', 5, 80),
    ('Travel Pack', 'Adds 8 inventory slots', 'bag', 'rare', 8, 250),
    ('Adventurer Bag', 'Adds 12 inventory slots', 'bag', 'epic', 12, 600),
    ('Bag of Holding', 'Adds 20 inventory slots', 'bag', 'legendary', 20, 2000);
