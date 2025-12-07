CREATE TABLE map_tiles (
    server_id UUID NOT NULL REFERENCES servers(id),
    x INT NOT NULL,
    y INT NOT NULL,
    terrain VARCHAR(20) NOT NULL, -- 'grassland', 'forest', 'mountain', 'water', 'swamp'
    oasis_type VARCHAR(20), -- 'wood', 'clay', 'iron', 'crop', NULL
    oasis_bonus DECIMAL(4,2),
    PRIMARY KEY (server_id, x, y)
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    player_id UUID REFERENCES players(id), -- NULL if account-level purchase
    amount_thb DECIMAL(10,2) NOT NULL,
    gold_amount INT NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    payment_provider VARCHAR(50) NOT NULL,
    external_id VARCHAR(255), -- Payment gateway reference
    status VARCHAR(20) DEFAULT 'pending', -- pending, completed, failed, refunded
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_transactions_user ON transactions(user_id, created_at DESC);
CREATE INDEX idx_transactions_status ON transactions(status) WHERE status = 'pending';

CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_sessions_token ON sessions(token_hash);
CREATE INDEX idx_sessions_user ON sessions(user_id);
