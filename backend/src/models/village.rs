use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Village {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub is_capital: bool,
    // Resources
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    // Storage limits
    pub warehouse_capacity: i32,
    pub granary_capacity: i32,
    // Stats
    pub population: i32,
    pub culture_points: i32,
    pub loyalty: i32,
    // Timestamps
    pub resources_updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVillage {
    pub user_id: Uuid,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub is_capital: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVillage {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRates {
    pub wood_per_hour: i32,
    pub clay_per_hour: i32,
    pub iron_per_hour: i32,
    pub crop_per_hour: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VillageResponse {
    pub id: Uuid,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub is_capital: bool,
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
    pub warehouse_capacity: i32,
    pub granary_capacity: i32,
    pub population: i32,
    pub culture_points: i32,
    pub loyalty: i32,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<ProductionRates>,
}

impl From<Village> for VillageResponse {
    fn from(v: Village) -> Self {
        Self {
            id: v.id,
            name: v.name,
            x: v.x,
            y: v.y,
            is_capital: v.is_capital,
            wood: v.wood,
            clay: v.clay,
            iron: v.iron,
            crop: v.crop,
            warehouse_capacity: v.warehouse_capacity,
            granary_capacity: v.granary_capacity,
            population: v.population,
            culture_points: v.culture_points,
            loyalty: v.loyalty,
            created_at: v.created_at,
            production: None,
        }
    }
}

impl VillageResponse {
    pub fn with_production(mut self, production: ProductionRates) -> Self {
        self.production = Some(production);
        self
    }
}

// For map display - lightweight version
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VillageMapInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub population: i32,
    pub player_name: Option<String>,
}
