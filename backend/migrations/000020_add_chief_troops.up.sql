-- Add loyalty_reduction column to troop_definitions
-- Chief-type troops can reduce village loyalty when conquering

ALTER TABLE troop_definitions ADD COLUMN loyalty_reduction INT NOT NULL DEFAULT 0;

-- Add Chief troop types to enum
ALTER TYPE troop_type ADD VALUE 'royal_advisor';
ALTER TYPE troop_type ADD VALUE 'harbor_master';
ALTER TYPE troop_type ADD VALUE 'elder_chief';

-- Add comment
COMMENT ON COLUMN troop_definitions.loyalty_reduction IS 'Amount of loyalty reduced when this troop conquers a village. Only Chief-type troops have non-zero values.';
