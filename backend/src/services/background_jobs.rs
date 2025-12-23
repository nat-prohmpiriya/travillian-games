use sqlx::PgPool;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

use crate::repositories::building_repo::BuildingRepository;
use crate::services::building_service::BuildingService;
use crate::services::resource_service::ResourceService;

/// Start all background jobs
pub async fn start_background_jobs(pool: PgPool) {
    // Spawn building completion job
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        run_building_completion_job(pool_clone).await;
    });

    // Spawn resource production job
    let pool_clone = pool.clone();
    tokio::spawn(async move {
        run_resource_production_job(pool_clone).await;
    });

    info!("Background jobs started");
}

/// Check and complete building upgrades every 10 seconds
async fn run_building_completion_job(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(10));

    loop {
        ticker.tick().await;

        match complete_building_upgrades(&pool).await {
            Ok(count) => {
                if count > 0 {
                    info!("Completed {} building upgrades", count);
                }
            }
            Err(e) => {
                error!("Error completing building upgrades: {:?}", e);
            }
        }
    }
}

/// Complete all buildings that have finished upgrading
async fn complete_building_upgrades(pool: &PgPool) -> anyhow::Result<i32> {
    let buildings = BuildingRepository::find_completed_upgrades(pool).await?;
    let mut completed = 0;

    for building in buildings {
        // Use BuildingService to handle upgrade completion with side effects
        match BuildingService::complete_upgrade(pool, building.id).await {
            Ok(updated) => {
                info!(
                    "Building {:?} upgraded to level {} in village {}",
                    updated.building_type, updated.level, updated.village_id
                );
                completed += 1;
            }
            Err(e) => {
                error!("Error completing upgrade for building {}: {:?}", building.id, e);
            }
        }
    }

    Ok(completed)
}

/// Update resource production every 5 minutes
async fn run_resource_production_job(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(300)); // 5 minutes

    loop {
        ticker.tick().await;

        match ResourceService::update_all_village_resources(&pool).await {
            Ok(count) => {
                if count > 0 {
                    info!("Updated resources for {} villages", count);
                }
            }
            Err(e) => {
                error!("Error updating village resources: {:?}", e);
            }
        }
    }
}
