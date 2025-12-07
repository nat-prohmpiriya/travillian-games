CREATE TYPE tribe_code AS ENUM ('phasuttha', 'nava', 'kiri');
CREATE TYPE server_status AS ENUM ('preparing', 'running', 'ended');

CREATE TABLE servers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(20) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    region VARCHAR(50) NOT NULL,
    status server_status DEFAULT 'preparing',
    speed_multiplier DECIMAL(3,1) DEFAULT 1.0,
    map_size_x INT DEFAULT 200,
    map_size_y INT DEFAULT 200,
    started_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE tribes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code tribe_code UNIQUE NOT NULL,
    name_i18n JSONB NOT NULL,
    description_i18n JSONB NOT NULL,
    bonus_attack DECIMAL(4,2) DEFAULT 1.0,
    bonus_defense DECIMAL(4,2) DEFAULT 1.0,
    bonus_speed DECIMAL(4,2) DEFAULT 1.0,
    bonus_capacity DECIMAL(4,2) DEFAULT 1.0
);
