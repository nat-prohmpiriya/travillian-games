use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "building_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BuildingType {
    // Village buildings
    MainBuilding,
    Warehouse,
    Granary,
    Barracks,
    Stable,
    Workshop,
    Academy,
    Smithy,
    RallyPoint,
    Market,
    Embassy,
    TownHall,
    Residence,
    Palace,
    Treasury,
    TradeOffice,
    Wall,
    // Resource fields
    Woodcutter,
    ClayPit,
    IronMine,
    CropField,
}

impl BuildingType {
    pub fn is_resource_field(&self) -> bool {
        matches!(
            self,
            BuildingType::Woodcutter
                | BuildingType::ClayPit
                | BuildingType::IronMine
                | BuildingType::CropField
        )
    }

    pub fn max_level(&self) -> i32 {
        match self {
            BuildingType::Wall => 20,
            BuildingType::Palace | BuildingType::Residence => 20,
            _ if self.is_resource_field() => 20,
            _ => 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Building {
    pub id: Uuid,
    pub village_id: Uuid,
    pub building_type: BuildingType,
    pub slot: i32,
    pub level: i32,
    pub is_upgrading: bool,
    pub upgrade_ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuilding {
    pub village_id: Uuid,
    pub building_type: BuildingType,
    pub slot: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingResponse {
    pub id: Uuid,
    pub building_type: BuildingType,
    pub slot: i32,
    pub level: i32,
    pub is_upgrading: bool,
    pub upgrade_ends_at: Option<DateTime<Utc>>,
}

impl From<Building> for BuildingResponse {
    fn from(b: Building) -> Self {
        Self {
            id: b.id,
            building_type: b.building_type,
            slot: b.slot,
            level: b.level,
            is_upgrading: b.is_upgrading,
            upgrade_ends_at: b.upgrade_ends_at,
        }
    }
}

// Building costs and production rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingCost {
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    pub time_seconds: i32,
}

impl BuildingType {
    pub fn base_cost(&self) -> BuildingCost {
        match self {
            BuildingType::MainBuilding => BuildingCost {
                wood: 70,
                clay: 40,
                iron: 60,
                crop: 20,
                time_seconds: 300,
            },
            BuildingType::Warehouse => BuildingCost {
                wood: 130,
                clay: 160,
                iron: 90,
                crop: 40,
                time_seconds: 400,
            },
            BuildingType::Granary => BuildingCost {
                wood: 80,
                clay: 100,
                iron: 70,
                crop: 20,
                time_seconds: 350,
            },
            BuildingType::Barracks => BuildingCost {
                wood: 210,
                clay: 140,
                iron: 260,
                crop: 120,
                time_seconds: 600,
            },
            BuildingType::RallyPoint => BuildingCost {
                wood: 110,
                clay: 160,
                iron: 90,
                crop: 70,
                time_seconds: 250,
            },
            BuildingType::Market => BuildingCost {
                wood: 80,
                clay: 70,
                iron: 120,
                crop: 70,
                time_seconds: 400,
            },
            // Resource fields
            BuildingType::Woodcutter => BuildingCost {
                wood: 40,
                clay: 100,
                iron: 50,
                crop: 60,
                time_seconds: 260,
            },
            BuildingType::ClayPit => BuildingCost {
                wood: 80,
                clay: 40,
                iron: 80,
                crop: 50,
                time_seconds: 220,
            },
            BuildingType::IronMine => BuildingCost {
                wood: 100,
                clay: 80,
                iron: 30,
                crop: 60,
                time_seconds: 450,
            },
            BuildingType::CropField => BuildingCost {
                wood: 70,
                clay: 90,
                iron: 70,
                crop: 20,
                time_seconds: 150,
            },
            // Default for others
            _ => BuildingCost {
                wood: 100,
                clay: 100,
                iron: 100,
                crop: 50,
                time_seconds: 300,
            },
        }
    }

    pub fn cost_at_level(&self, level: i32) -> BuildingCost {
        let base = self.base_cost();
        let multiplier = (1.28_f64).powi(level - 1);
        BuildingCost {
            wood: (base.wood as f64 * multiplier) as i32,
            clay: (base.clay as f64 * multiplier) as i32,
            iron: (base.iron as f64 * multiplier) as i32,
            crop: (base.crop as f64 * multiplier) as i32,
            time_seconds: (base.time_seconds as f64 * multiplier) as i32,
        }
    }

    pub fn production_per_hour(&self, level: i32) -> i32 {
        if !self.is_resource_field() {
            return 0;
        }
        // Base production formula similar to Travian
        let base = 3;
        (base as f64 * (1.63_f64).powi(level - 1) * 1.0034_f64.powi((level - 1) * (level - 1))) as i32
    }

    /// Storage capacity for Warehouse/Granary at given level
    /// Based on Travian formula: base * 1.2^level
    pub fn storage_capacity(&self, level: i32) -> i32 {
        if level == 0 {
            return 800; // Base capacity
        }
        let base = match self {
            BuildingType::Warehouse => 400,
            BuildingType::Granary => 400,
            _ => return 0,
        };
        (base as f64 * (1.2_f64).powi(level)) as i32
    }
}
