use axum::{
    extract::{Path, State},
    Extension, Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::building::{BuildingCost, BuildingResponse, BuildingType, CreateBuilding};
use crate::repositories::building_repo::BuildingRepository;
use crate::repositories::user_repo::UserRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::building_service::BuildingService;
use crate::AppState;

// GET /api/villages/:village_id/buildings - List buildings in a village
pub async fn list_buildings(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<BuildingResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let buildings = BuildingRepository::find_by_village_id(&state.db, village_id).await?;

    Ok(Json(buildings.into_iter().map(|b| b.into()).collect()))
}

#[derive(Debug, Deserialize)]
pub struct BuildRequest {
    pub building_type: BuildingType,
}

#[derive(Debug, Serialize)]
pub struct BuildResponse {
    pub building: BuildingResponse,
    pub cost: BuildingCost,
}

// POST /api/villages/:village_id/buildings/:slot - Build new building
pub async fn build(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path((village_id, slot)): Path<(Uuid, i32)>,
    Json(body): Json<BuildRequest>,
) -> AppResult<Json<BuildResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    // Check if slot is empty
    if BuildingRepository::find_by_village_and_slot(&state.db, village_id, slot)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("Slot already occupied".to_string()));
    }

    // Check prerequisites
    BuildingService::validate_can_build(&state.db, village_id, &body.building_type).await?;

    // Get cost for level 1
    let cost = body.building_type.cost_at_level(1);

    // Check resources
    if village.wood < cost.wood
        || village.clay < cost.clay
        || village.iron < cost.iron
        || village.crop < cost.crop
    {
        return Err(AppError::BadRequest("Not enough resources".to_string()));
    }

    // Deduct resources
    VillageRepository::deduct_resources(
        &state.db,
        village_id,
        cost.wood,
        cost.clay,
        cost.iron,
        cost.crop,
    )
    .await?;

    // Create building
    let create = CreateBuilding {
        village_id,
        building_type: body.building_type.clone(),
        slot,
    };
    let building = BuildingRepository::create(&state.db, create).await?;

    // Start upgrade timer
    let upgrade_ends_at = Utc::now() + chrono::Duration::seconds(cost.time_seconds as i64);
    let building = BuildingRepository::start_upgrade(&state.db, building.id, upgrade_ends_at).await?;

    info!(
        "Building {:?} started at slot {} in village {}",
        body.building_type, slot, village_id
    );

    Ok(Json(BuildResponse {
        building: building.into(),
        cost,
    }))
}

#[derive(Debug, Serialize)]
pub struct UpgradeResponse {
    pub building: BuildingResponse,
    pub cost: BuildingCost,
}

// POST /api/villages/:village_id/buildings/:slot/upgrade - Upgrade building
pub async fn upgrade(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path((village_id, slot)): Path<(Uuid, i32)>,
) -> AppResult<Json<UpgradeResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let building = BuildingRepository::find_by_village_and_slot(&state.db, village_id, slot)
        .await?
        .ok_or_else(|| AppError::NotFound("Building not found".to_string()))?;

    if building.is_upgrading {
        return Err(AppError::Conflict("Building is already upgrading".to_string()));
    }

    let next_level = building.level + 1;
    if next_level > building.building_type.max_level() {
        return Err(AppError::BadRequest("Building is at max level".to_string()));
    }

    let cost = building.building_type.cost_at_level(next_level);

    // Check resources
    if village.wood < cost.wood
        || village.clay < cost.clay
        || village.iron < cost.iron
        || village.crop < cost.crop
    {
        return Err(AppError::BadRequest("Not enough resources".to_string()));
    }

    // Deduct resources
    VillageRepository::deduct_resources(
        &state.db,
        village_id,
        cost.wood,
        cost.clay,
        cost.iron,
        cost.crop,
    )
    .await?;

    // Start upgrade
    let upgrade_ends_at = Utc::now() + chrono::Duration::seconds(cost.time_seconds as i64);
    let building = BuildingRepository::start_upgrade(&state.db, building.id, upgrade_ends_at).await?;

    info!(
        "Upgrading {:?} to level {} in village {}",
        building.building_type, next_level, village_id
    );

    Ok(Json(UpgradeResponse {
        building: building.into(),
        cost,
    }))
}

// DELETE /api/villages/:village_id/buildings/:slot - Demolish building
pub async fn demolish(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path((village_id, slot)): Path<(Uuid, i32)>,
) -> AppResult<Json<serde_json::Value>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let building = BuildingRepository::find_by_village_and_slot(&state.db, village_id, slot)
        .await?
        .ok_or_else(|| AppError::NotFound("Building not found".to_string()))?;

    // Some buildings cannot be demolished
    if building.building_type == BuildingType::MainBuilding && building.level > 0 {
        return Err(AppError::BadRequest(
            "Cannot demolish Main Building".to_string(),
        ));
    }

    BuildingRepository::demolish(&state.db, building.id).await?;

    info!(
        "Building {:?} demolished at slot {} in village {}",
        building.building_type, slot, village_id
    );

    Ok(Json(serde_json::json!({
        "message": "Building demolished successfully"
    })))
}

// GET /api/villages/:village_id/buildings/queue - Get build queue
pub async fn get_build_queue(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<BuildingResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let buildings = BuildingRepository::find_upgrading_by_village(&state.db, village_id).await?;

    Ok(Json(buildings.into_iter().map(|b| b.into()).collect()))
}
