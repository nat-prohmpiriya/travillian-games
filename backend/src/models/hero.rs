use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::troop::TribeType;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "hero_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum HeroStatus {
    Idle,
    Moving,
    InAdventure,
    InBattle,
    Dead,
    Reviving,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "item_rarity", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "item_slot", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemSlot {
    Helmet,
    Weapon,
    ArmorLeft,
    ArmorRight,
    Boots,
    Horse,
    Bag,
    Bandage,
    Consumable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "adventure_difficulty", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AdventureDifficulty {
    Short,
    Long,
}

// ==================== Database Models ====================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Hero {
    pub id: Uuid,
    pub user_id: Uuid,
    pub slot_number: i32,

    // Basic info
    pub name: String,
    pub tribe: TribeType,

    // Location
    pub home_village_id: Uuid,
    pub current_village_id: Option<Uuid>,

    // Status
    pub status: HeroStatus,

    // Stats
    pub level: i32,
    pub experience: i32,
    pub experience_to_next: i32,

    // Health
    pub health: i32,
    pub health_regen_rate: Decimal,

    // Attribute points
    pub unassigned_points: i32,

    // Attributes
    pub fighting_strength: i32,
    pub off_bonus: i32,
    pub def_bonus: i32,
    pub resources_bonus: i32,

    // Base stats
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_speed: Decimal,

    // Timestamps
    pub last_health_update: DateTime<Utc>,
    pub died_at: Option<DateTime<Utc>>,
    pub revive_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Hero {
    /// Calculate total attack power
    pub fn total_attack(&self) -> i32 {
        self.base_attack + (self.fighting_strength * 80)
    }

    /// Calculate total defense
    pub fn total_defense(&self) -> i32 {
        self.base_defense
    }

    /// Calculate off bonus percentage
    pub fn off_bonus_percent(&self) -> f64 {
        self.off_bonus as f64 * 0.2
    }

    /// Calculate def bonus percentage
    pub fn def_bonus_percent(&self) -> f64 {
        self.def_bonus as f64 * 0.2
    }

    /// Check if hero is available for actions
    pub fn is_available(&self) -> bool {
        self.status == HeroStatus::Idle && self.health > 0
    }

    /// Check if hero is dead
    pub fn is_dead(&self) -> bool {
        self.status == HeroStatus::Dead || self.health <= 0
    }

    /// Calculate experience needed for level
    pub fn exp_for_level(level: i32) -> i32 {
        // Exponential growth: 100 * 1.5^(level-1)
        (100.0 * 1.5_f64.powi(level - 1)) as i32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemDefinition {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slot: ItemSlot,
    pub rarity: ItemRarity,

    // Requirements
    pub required_level: i32,

    // Stats bonuses
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub speed_bonus: Decimal,
    pub health_regen_bonus: Decimal,
    pub experience_bonus: i32,
    pub resource_bonus: i32,
    pub carry_bonus: i32,

    // For consumables
    pub health_restore: i32,
    pub is_consumable: bool,

    // For bags
    pub extra_inventory_slots: i32,

    // Value
    pub sell_value: i32,

    // Availability
    pub can_drop_adventure: bool,
    pub can_buy_auction: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HeroItem {
    pub id: Uuid,
    pub hero_id: Uuid,
    pub item_definition_id: Uuid,

    // Item state
    pub is_equipped: bool,
    pub equipped_slot: Option<ItemSlot>,

    // For stackable items
    pub quantity: i32,

    // Timestamps
    pub obtained_at: DateTime<Utc>,
    pub equipped_at: Option<DateTime<Utc>>,
}

/// Flattened struct for joined hero_items + item_definitions query
#[derive(Debug, Clone, FromRow)]
pub struct HeroItemWithDefinition {
    // HeroItem fields
    pub id: Uuid,
    pub hero_id: Uuid,
    pub item_definition_id: Uuid,
    pub is_equipped: bool,
    pub equipped_slot: Option<ItemSlot>,
    pub quantity: i32,
    pub obtained_at: DateTime<Utc>,
    pub equipped_at: Option<DateTime<Utc>>,

    // ItemDefinition fields (prefixed with item_)
    pub item_id: Uuid,
    pub item_name: String,
    pub item_description: Option<String>,
    pub item_slot: ItemSlot,
    pub item_rarity: ItemRarity,
    pub item_required_level: i32,
    pub item_attack_bonus: i32,
    pub item_defense_bonus: i32,
    pub item_speed_bonus: Decimal,
    pub item_health_regen_bonus: Decimal,
    pub item_experience_bonus: i32,
    pub item_resource_bonus: i32,
    pub item_carry_bonus: i32,
    pub item_health_restore: i32,
    pub item_is_consumable: bool,
    pub item_extra_inventory_slots: i32,
    pub item_sell_value: i32,
}

impl HeroItemWithDefinition {
    pub fn into_parts(self) -> (HeroItem, ItemDefinition) {
        let hero_item = HeroItem {
            id: self.id,
            hero_id: self.hero_id,
            item_definition_id: self.item_definition_id,
            is_equipped: self.is_equipped,
            equipped_slot: self.equipped_slot,
            quantity: self.quantity,
            obtained_at: self.obtained_at,
            equipped_at: self.equipped_at,
        };

        let item_def = ItemDefinition {
            id: self.item_id,
            name: self.item_name,
            description: self.item_description,
            slot: self.item_slot,
            rarity: self.item_rarity,
            required_level: self.item_required_level,
            attack_bonus: self.item_attack_bonus,
            defense_bonus: self.item_defense_bonus,
            speed_bonus: self.item_speed_bonus,
            health_regen_bonus: self.item_health_regen_bonus,
            experience_bonus: self.item_experience_bonus,
            resource_bonus: self.item_resource_bonus,
            carry_bonus: self.item_carry_bonus,
            health_restore: self.item_health_restore,
            is_consumable: self.item_is_consumable,
            extra_inventory_slots: self.item_extra_inventory_slots,
            sell_value: self.item_sell_value,
            can_drop_adventure: true,
            can_buy_auction: true,
            created_at: Utc::now(),
        };

        (hero_item, item_def)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HeroAdventure {
    pub id: Uuid,
    pub hero_id: Uuid,

    // Adventure details
    pub difficulty: AdventureDifficulty,

    // Timing
    pub started_at: DateTime<Utc>,
    pub duration_seconds: i32,
    pub ends_at: DateTime<Utc>,

    // Status
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,

    // Rewards
    pub reward_experience: Option<i32>,
    pub reward_silver: Option<i32>,
    pub reward_resources: Option<serde_json::Value>,
    pub reward_item_id: Option<Uuid>,

    // Damage taken
    pub health_lost: Option<i32>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AvailableAdventure {
    pub id: Uuid,
    pub user_id: Uuid,

    pub difficulty: AdventureDifficulty,

    // Duration range
    pub min_duration_seconds: i32,
    pub max_duration_seconds: i32,

    // Reward hints
    pub potential_reward_type: Option<String>,
    pub potential_item_rarity: Option<ItemRarity>,

    // Availability
    pub expires_at: DateTime<Utc>,
    pub is_taken: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HeroSlotPrice {
    pub slot_number: i32,
    pub gold_cost: i32,
}

// ==================== Request DTOs ====================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateHeroRequest {
    pub name: String,
    pub tribe: TribeType,
    pub home_village_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssignAttributesRequest {
    pub fighting_strength: i32,
    pub off_bonus: i32,
    pub def_bonus: i32,
    pub resources_bonus: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeHomeVillageRequest {
    pub village_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EquipItemRequest {
    pub item_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnequipItemRequest {
    pub slot: ItemSlot,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UseItemRequest {
    pub item_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartAdventureRequest {
    pub adventure_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReviveHeroRequest {
    pub use_gold: bool,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize)]
pub struct HeroResponse {
    pub id: Uuid,
    pub slot_number: i32,
    pub name: String,
    pub tribe: TribeType,

    // Location
    pub home_village_id: Uuid,
    pub current_village_id: Option<Uuid>,

    // Status
    pub status: HeroStatus,

    // Stats
    pub level: i32,
    pub experience: i32,
    pub experience_to_next: i32,

    // Health
    pub health: i32,
    pub health_regen_rate: Decimal,

    // Attribute points
    pub unassigned_points: i32,

    // Attributes
    pub fighting_strength: i32,
    pub off_bonus: i32,
    pub def_bonus: i32,
    pub resources_bonus: i32,

    // Calculated stats
    pub total_attack: i32,
    pub total_defense: i32,
    pub off_bonus_percent: f64,
    pub def_bonus_percent: f64,
    pub base_speed: Decimal,

    // Timestamps
    pub died_at: Option<DateTime<Utc>>,
    pub revive_at: Option<DateTime<Utc>>,
}

impl From<Hero> for HeroResponse {
    fn from(h: Hero) -> Self {
        Self {
            id: h.id,
            slot_number: h.slot_number,
            name: h.name.clone(),
            tribe: h.tribe,
            home_village_id: h.home_village_id,
            current_village_id: h.current_village_id,
            status: h.status,
            level: h.level,
            experience: h.experience,
            experience_to_next: h.experience_to_next,
            health: h.health,
            health_regen_rate: h.health_regen_rate,
            unassigned_points: h.unassigned_points,
            fighting_strength: h.fighting_strength,
            off_bonus: h.off_bonus,
            def_bonus: h.def_bonus,
            resources_bonus: h.resources_bonus,
            total_attack: h.total_attack(),
            total_defense: h.total_defense(),
            off_bonus_percent: h.off_bonus_percent(),
            def_bonus_percent: h.def_bonus_percent(),
            base_speed: h.base_speed,
            died_at: h.died_at,
            revive_at: h.revive_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroListResponse {
    pub heroes: Vec<HeroResponse>,
    pub total_slots: i32,
    pub used_slots: i32,
    pub next_slot_cost: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemDefinitionResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slot: ItemSlot,
    pub rarity: ItemRarity,
    pub required_level: i32,

    // Stats
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub speed_bonus: Decimal,
    pub health_regen_bonus: Decimal,
    pub experience_bonus: i32,
    pub resource_bonus: i32,
    pub carry_bonus: i32,

    // Consumable
    pub health_restore: i32,
    pub is_consumable: bool,

    // Bag
    pub extra_inventory_slots: i32,

    pub sell_value: i32,
}

impl From<ItemDefinition> for ItemDefinitionResponse {
    fn from(d: ItemDefinition) -> Self {
        Self {
            id: d.id,
            name: d.name,
            description: d.description,
            slot: d.slot,
            rarity: d.rarity,
            required_level: d.required_level,
            attack_bonus: d.attack_bonus,
            defense_bonus: d.defense_bonus,
            speed_bonus: d.speed_bonus,
            health_regen_bonus: d.health_regen_bonus,
            experience_bonus: d.experience_bonus,
            resource_bonus: d.resource_bonus,
            carry_bonus: d.carry_bonus,
            health_restore: d.health_restore,
            is_consumable: d.is_consumable,
            extra_inventory_slots: d.extra_inventory_slots,
            sell_value: d.sell_value,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroItemResponse {
    pub id: Uuid,
    pub item: ItemDefinitionResponse,
    pub is_equipped: bool,
    pub equipped_slot: Option<ItemSlot>,
    pub quantity: i32,
    pub obtained_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EquippedItemsResponse {
    pub helmet: Option<HeroItemResponse>,
    pub weapon: Option<HeroItemResponse>,
    pub armor_left: Option<HeroItemResponse>,
    pub armor_right: Option<HeroItemResponse>,
    pub boots: Option<HeroItemResponse>,
    pub horse: Option<HeroItemResponse>,
    pub bag: Option<HeroItemResponse>,
    pub bandage: Option<HeroItemResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InventoryResponse {
    pub equipped: EquippedItemsResponse,
    pub items: Vec<HeroItemResponse>,
    pub total_slots: i32,
    pub used_slots: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableAdventureResponse {
    pub id: Uuid,
    pub difficulty: AdventureDifficulty,
    pub duration_range: String,
    pub potential_reward: Option<String>,
    pub potential_rarity: Option<ItemRarity>,
    pub expires_at: DateTime<Utc>,
}

impl From<AvailableAdventure> for AvailableAdventureResponse {
    fn from(a: AvailableAdventure) -> Self {
        let min_mins = a.min_duration_seconds / 60;
        let max_mins = a.max_duration_seconds / 60;
        let duration_range = if max_mins < 60 {
            format!("{}-{} min", min_mins, max_mins)
        } else {
            format!("{}-{} hr", min_mins / 60, max_mins / 60)
        };

        Self {
            id: a.id,
            difficulty: a.difficulty,
            duration_range,
            potential_reward: a.potential_reward_type,
            potential_rarity: a.potential_item_rarity,
            expires_at: a.expires_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroAdventureResponse {
    pub id: Uuid,
    pub hero_id: Uuid,
    pub difficulty: AdventureDifficulty,
    pub started_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub is_completed: bool,

    // Rewards (if completed)
    pub reward_experience: Option<i32>,
    pub reward_silver: Option<i32>,
    pub reward_resources: Option<serde_json::Value>,
    pub reward_item: Option<ItemDefinitionResponse>,
    pub health_lost: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviveInfoResponse {
    pub hero_id: Uuid,
    pub revive_at: DateTime<Utc>,
    pub remaining_seconds: i64,
    pub gold_cost_instant: i32,
    pub resource_cost: ReviveResourceCost,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviveResourceCost {
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct HeroSlotPurchaseResponse {
    pub success: bool,
    pub new_slot_number: i32,
    pub gold_spent: i32,
    pub new_balance: i32,
    pub total_slots: i32,
}
