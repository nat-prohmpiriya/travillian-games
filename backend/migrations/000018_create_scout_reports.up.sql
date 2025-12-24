-- Scout reports for reconnaissance missions
CREATE TABLE scout_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    attacker_player_id UUID NOT NULL REFERENCES users(id),
    defender_player_id UUID REFERENCES users(id),
    attacker_village_id UUID NOT NULL REFERENCES villages(id),
    defender_village_id UUID REFERENCES villages(id),

    -- Scout combat (scouts vs scouts)
    attacker_scouts INT NOT NULL DEFAULT 0,
    defender_scouts INT NOT NULL DEFAULT 0,
    attacker_scouts_lost INT NOT NULL DEFAULT 0,
    defender_scouts_lost INT NOT NULL DEFAULT 0,

    -- Result
    success BOOLEAN NOT NULL DEFAULT FALSE,

    -- Scouted information (NULL if scout failed)
    scouted_resources JSONB, -- {wood, clay, iron, crop}
    scouted_troops JSONB,    -- {troop_type: count}

    -- Timestamps
    occurred_at TIMESTAMPTZ NOT NULL,
    read_by_attacker BOOLEAN DEFAULT FALSE,
    read_by_defender BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_scout_reports_attacker ON scout_reports(attacker_player_id);
CREATE INDEX idx_scout_reports_defender ON scout_reports(defender_player_id);
