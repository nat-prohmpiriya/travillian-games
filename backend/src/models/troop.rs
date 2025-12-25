use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::building::BuildingType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "troop_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TroopType {
    // Phasuttha (Mainland/Thai-inspired)
    Infantry,
    Spearman,
    WarElephant,
    BuffaloWagon,
    // Nava (Maritime/Malay-inspired)
    KrisWarrior,
    SeaDiver,
    WarPrahu,
    MerchantShip,
    // Kiri (Highland/Hill tribe-inspired)
    Crossbowman,
    MountainWarrior,
    HighlandPony,
    TrapMaker,
    // Special units
    SwampDragon,
    LocustSwarm,
    BattleDuck,
    PortugueseMusketeer,
    // Chief units (can reduce loyalty)
    RoyalAdvisor,
    HarborMaster,
    ElderChief,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "tribe_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TribeType {
    Phasuttha,
    Nava,
    Kiri,
    Special,
}

impl TroopType {
    pub fn tribe(&self) -> TribeType {
        match self {
            TroopType::Infantry | TroopType::Spearman | TroopType::WarElephant | TroopType::BuffaloWagon | TroopType::RoyalAdvisor => TribeType::Phasuttha,
            TroopType::KrisWarrior | TroopType::SeaDiver | TroopType::WarPrahu | TroopType::MerchantShip | TroopType::HarborMaster => TribeType::Nava,
            TroopType::Crossbowman | TroopType::MountainWarrior | TroopType::HighlandPony | TroopType::TrapMaker | TroopType::ElderChief => TribeType::Kiri,
            _ => TribeType::Special,
        }
    }

    pub fn is_cavalry(&self) -> bool {
        matches!(
            self,
            TroopType::WarElephant
                | TroopType::BuffaloWagon
                | TroopType::HighlandPony
                | TroopType::WarPrahu
                | TroopType::MerchantShip
        )
    }

    pub fn is_infantry(&self) -> bool {
        !self.is_cavalry()
    }

    /// Check if this troop type is a Chief (can reduce loyalty)
    pub fn is_chief(&self) -> bool {
        matches!(
            self,
            TroopType::RoyalAdvisor | TroopType::HarborMaster | TroopType::ElderChief
        )
    }
}

/// Troop definition from database (base stats)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TroopDefinition {
    pub id: Uuid,
    pub troop_type: TroopType,
    pub tribe: TribeType,
    pub name: String,
    pub description: Option<String>,
    // Combat stats
    pub attack: i32,
    pub defense_infantry: i32,
    pub defense_cavalry: i32,
    pub speed: i32,
    pub carry_capacity: i32,
    pub crop_consumption: i32,
    // Training
    pub training_time_seconds: i32,
    pub wood_cost: i32,
    pub clay_cost: i32,
    pub iron_cost: i32,
    pub crop_cost: i32,
    // Requirements
    pub required_building: BuildingType,
    pub required_building_level: i32,
    // Conquer ability
    pub loyalty_reduction: i32,
    pub created_at: DateTime<Utc>,
}

/// Troops owned by a village
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Troop {
    pub id: Uuid,
    pub village_id: Uuid,
    pub troop_type: TroopType,
    pub count: i32,
    pub in_village: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Training queue entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TroopQueue {
    pub id: Uuid,
    pub village_id: Uuid,
    pub troop_type: TroopType,
    pub count: i32,
    pub each_duration_seconds: i32,
    pub started_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// Request/Response DTOs

#[derive(Debug, Clone, Deserialize)]
pub struct TrainTroopsRequest {
    pub troop_type: TroopType,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrainTroopsResponse {
    pub queue_entry: TroopQueueResponse,
    pub cost: TroopCost,
}

#[derive(Debug, Clone, Serialize)]
pub struct TroopCost {
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    pub time_seconds: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TroopResponse {
    pub troop_type: TroopType,
    pub count: i32,
    pub in_village: i32,
    pub on_mission: i32,
}

impl From<Troop> for TroopResponse {
    fn from(t: Troop) -> Self {
        Self {
            troop_type: t.troop_type,
            count: t.count,
            in_village: t.in_village,
            on_mission: t.count - t.in_village,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TroopQueueResponse {
    pub id: Uuid,
    pub troop_type: TroopType,
    pub count: i32,
    pub each_duration_seconds: i32,
    pub started_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

impl From<TroopQueue> for TroopQueueResponse {
    fn from(q: TroopQueue) -> Self {
        Self {
            id: q.id,
            troop_type: q.troop_type,
            count: q.count,
            each_duration_seconds: q.each_duration_seconds,
            started_at: q.started_at,
            ends_at: q.ends_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TroopDefinitionResponse {
    pub troop_type: TroopType,
    pub tribe: TribeType,
    pub name: String,
    pub description: Option<String>,
    pub attack: i32,
    pub defense_infantry: i32,
    pub defense_cavalry: i32,
    pub speed: i32,
    pub carry_capacity: i32,
    pub crop_consumption: i32,
    pub training_time_seconds: i32,
    pub wood_cost: i32,
    pub clay_cost: i32,
    pub iron_cost: i32,
    pub crop_cost: i32,
    pub required_building: BuildingType,
    pub required_building_level: i32,
    pub loyalty_reduction: i32,
}

impl From<TroopDefinition> for TroopDefinitionResponse {
    fn from(d: TroopDefinition) -> Self {
        Self {
            troop_type: d.troop_type,
            tribe: d.tribe,
            name: d.name,
            description: d.description,
            attack: d.attack,
            defense_infantry: d.defense_infantry,
            defense_cavalry: d.defense_cavalry,
            speed: d.speed,
            carry_capacity: d.carry_capacity,
            crop_consumption: d.crop_consumption,
            training_time_seconds: d.training_time_seconds,
            wood_cost: d.wood_cost,
            clay_cost: d.clay_cost,
            iron_cost: d.iron_cost,
            crop_cost: d.crop_cost,
            required_building: d.required_building,
            required_building_level: d.required_building_level,
            loyalty_reduction: d.loyalty_reduction,
        }
    }
}
