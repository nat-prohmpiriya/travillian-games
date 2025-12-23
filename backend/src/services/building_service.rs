use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::building::{Building, BuildingType};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct BuildingService;

impl BuildingService {
    /// Complete a building upgrade and handle side effects
    pub async fn complete_upgrade(pool: &PgPool, building_id: Uuid) -> AppResult<Building> {
        // Complete the upgrade
        let building = BuildingRepository::complete_upgrade(pool, building_id).await?;

        // Handle side effects based on building type
        match building.building_type {
            BuildingType::Warehouse | BuildingType::Granary => {
                Self::update_village_storage(pool, building.village_id).await?;
            }
            _ => {}
        }

        Ok(building)
    }

    /// Recalculate and update village storage capacity based on all Warehouse/Granary buildings
    pub async fn update_village_storage(pool: &PgPool, village_id: Uuid) -> AppResult<()> {
        let buildings = BuildingRepository::find_by_village_id(pool, village_id).await?;

        let mut warehouse_capacity = 800; // Base capacity
        let mut granary_capacity = 800; // Base capacity

        for building in buildings {
            match building.building_type {
                BuildingType::Warehouse => {
                    warehouse_capacity += building.building_type.storage_capacity(building.level);
                }
                BuildingType::Granary => {
                    granary_capacity += building.building_type.storage_capacity(building.level);
                }
                _ => {}
            }
        }

        VillageRepository::update_storage_capacity(pool, village_id, warehouse_capacity, granary_capacity)
            .await?;

        Ok(())
    }
}
