-- Mission types for army movements
CREATE TYPE mission_type AS ENUM (
    'raid',      -- Quick attack, steal resources
    'attack',    -- Full attack, kill troops
    'conquer',   -- Attack to take over village
    'support',   -- Send troops to defend
    'scout',     -- Reconnaissance mission
    'settle'     -- Found new village with settlers
);

-- Army movements and attacks
CREATE TABLE armies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES users(id),
    from_village_id UUID NOT NULL REFERENCES villages(id),
    to_x INT NOT NULL,
    to_y INT NOT NULL,
    to_village_id UUID REFERENCES villages(id), -- NULL if empty tile
    mission mission_type NOT NULL,
    troops JSONB NOT NULL DEFAULT '{}', -- {"infantry": 100, "war_elephant": 10}
    resources JSONB NOT NULL DEFAULT '{}', -- Resources being carried
    departed_at TIMESTAMPTZ NOT NULL,
    arrives_at TIMESTAMPTZ NOT NULL,
    returns_at TIMESTAMPTZ, -- NULL if one-way (support/settle)
    is_returning BOOLEAN DEFAULT FALSE,
    battle_report_id UUID, -- Link to battle report after combat
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Battle reports
CREATE TABLE battle_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attacker_player_id UUID NOT NULL REFERENCES users(id),
    defender_player_id UUID REFERENCES users(id), -- NULL if Natarian/NPC
    attacker_village_id UUID NOT NULL REFERENCES villages(id),
    defender_village_id UUID REFERENCES villages(id),
    mission mission_type NOT NULL,
    -- Troops before battle
    attacker_troops JSONB NOT NULL DEFAULT '{}',
    defender_troops JSONB NOT NULL DEFAULT '{}',
    -- Casualties
    attacker_losses JSONB NOT NULL DEFAULT '{}',
    defender_losses JSONB NOT NULL DEFAULT '{}',
    -- Resources
    resources_stolen JSONB NOT NULL DEFAULT '{}',
    -- Result
    winner VARCHAR(20) NOT NULL, -- 'attacker', 'defender', 'draw'
    occurred_at TIMESTAMPTZ NOT NULL,
    read_by_attacker BOOLEAN DEFAULT FALSE,
    read_by_defender BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_armies_player ON armies(player_id);
CREATE INDEX idx_armies_from_village ON armies(from_village_id);
CREATE INDEX idx_armies_arrives ON armies(arrives_at);
CREATE INDEX idx_armies_destination ON armies(to_x, to_y);
CREATE INDEX idx_battle_reports_attacker ON battle_reports(attacker_player_id);
CREATE INDEX idx_battle_reports_defender ON battle_reports(defender_player_id);
