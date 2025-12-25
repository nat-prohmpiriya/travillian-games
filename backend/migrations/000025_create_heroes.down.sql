-- Rollback Hero System

DROP TABLE IF EXISTS hero_slot_prices;
DROP TABLE IF EXISTS available_adventures;
DROP TABLE IF EXISTS hero_adventures;
DROP TABLE IF EXISTS hero_items;
DROP TABLE IF EXISTS item_definitions;
DROP TABLE IF EXISTS heroes;

DROP TYPE IF EXISTS adventure_difficulty;
DROP TYPE IF EXISTS item_slot;
DROP TYPE IF EXISTS item_rarity;
DROP TYPE IF EXISTS hero_status;

ALTER TABLE users DROP COLUMN IF EXISTS hero_slots;
