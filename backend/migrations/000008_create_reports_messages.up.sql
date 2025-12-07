CREATE TYPE report_type AS ENUM ('battle', 'trade', 'scout', 'support', 'system');

CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id),
    report_type report_type NOT NULL,
    title VARCHAR(200) NOT NULL,
    data JSONB NOT NULL, -- Battle details, trade info, etc.
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_reports_player ON reports(player_id, created_at DESC);
CREATE INDEX idx_reports_unread ON reports(player_id, is_read) WHERE is_read = FALSE;

CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sender_id UUID NOT NULL REFERENCES players(id),
    recipient_id UUID REFERENCES players(id), -- NULL if alliance message
    alliance_id UUID REFERENCES alliances(id), -- NULL if private message
    content TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_messages_recipient ON messages(recipient_id, created_at DESC);
CREATE INDEX idx_messages_alliance ON messages(alliance_id, created_at DESC);
