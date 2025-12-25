use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::hero::{
    AssignAttributesRequest, AvailableAdventureResponse, ChangeHomeVillageRequest,
    CreateHeroRequest, EquipItemRequest, HeroAdventureResponse, HeroItemResponse, HeroListResponse,
    HeroResponse, HeroSlotPurchaseResponse, InventoryResponse, ItemSlot, ReviveHeroRequest,
    ReviveInfoResponse, StartAdventureRequest, UnequipItemRequest, UseItemRequest,
};
use crate::repositories::user_repo::UserRepository;
use crate::services::hero_service::HeroService;
use crate::AppState;

// ==================== Hero CRUD ====================

/// GET /api/heroes - Get all user's heroes
pub async fn list_heroes(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<HeroListResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let heroes = HeroService::get_user_heroes(&state.db, db_user.id).await?;
    Ok(Json(heroes))
}

/// GET /api/heroes/{id} - Get hero by ID
pub async fn get_hero(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero = HeroService::get_hero(&state.db, db_user.id, hero_id).await?;
    Ok(Json(hero))
}

/// POST /api/heroes - Create new hero
pub async fn create_hero(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateHeroRequest>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero = HeroService::create_hero(&state.db, db_user.id, request).await?;
    Ok(Json(hero))
}

/// PUT /api/heroes/{id}/home - Change home village
pub async fn change_home_village(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<ChangeHomeVillageRequest>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero =
        HeroService::change_home_village(&state.db, db_user.id, hero_id, request.village_id).await?;
    Ok(Json(hero))
}

/// PUT /api/heroes/{id}/attributes - Assign attribute points
pub async fn assign_attributes(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<AssignAttributesRequest>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero = HeroService::assign_attributes(&state.db, db_user.id, hero_id, request).await?;
    Ok(Json(hero))
}

// ==================== Hero Slots ====================

/// POST /api/heroes/slots/buy - Buy additional hero slot
pub async fn buy_hero_slot(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<HeroSlotPurchaseResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let result = HeroService::buy_hero_slot(&state.db, db_user.id).await?;
    Ok(Json(result))
}

// ==================== Inventory ====================

/// GET /api/heroes/{id}/inventory - Get hero's inventory
pub async fn get_inventory(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
) -> AppResult<Json<InventoryResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let inventory = HeroService::get_inventory(&state.db, db_user.id, hero_id).await?;
    Ok(Json(inventory))
}

/// POST /api/heroes/{id}/equip - Equip item
pub async fn equip_item(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<EquipItemRequest>,
) -> AppResult<Json<HeroItemResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let item = HeroService::equip_item(&state.db, db_user.id, hero_id, request.item_id).await?;
    Ok(Json(item))
}

/// POST /api/heroes/{id}/unequip - Unequip item from slot
pub async fn unequip_item(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<UnequipItemRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    HeroService::unequip_slot(&state.db, db_user.id, hero_id, request.slot).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

/// POST /api/heroes/{id}/use-item - Use consumable item
pub async fn use_item(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<UseItemRequest>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero = HeroService::use_item(&state.db, db_user.id, hero_id, request.item_id).await?;
    Ok(Json(hero))
}

/// DELETE /api/heroes/{hero_id}/items/{item_id} - Sell/delete item
pub async fn sell_item(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((hero_id, item_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<serde_json::Value>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let sell_value = HeroService::sell_item(&state.db, db_user.id, hero_id, item_id).await?;
    Ok(Json(serde_json::json!({
        "success": true,
        "silver_gained": sell_value
    })))
}

// ==================== Adventures ====================

/// GET /api/heroes/adventures/available - Get available adventures
pub async fn get_available_adventures(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<AvailableAdventureResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let adventures = HeroService::get_available_adventures(&state.db, db_user.id).await?;
    Ok(Json(adventures))
}

/// POST /api/heroes/{id}/adventures - Start adventure
pub async fn start_adventure(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<StartAdventureRequest>,
) -> AppResult<Json<HeroAdventureResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let adventure =
        HeroService::start_adventure(&state.db, db_user.id, hero_id, request.adventure_id).await?;
    Ok(Json(adventure))
}

/// GET /api/heroes/{id}/adventures/active - Get active adventure
pub async fn get_active_adventure(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
) -> AppResult<Json<Option<HeroAdventureResponse>>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let adventure = HeroService::get_active_adventure(&state.db, db_user.id, hero_id).await?;
    Ok(Json(adventure))
}

// ==================== Revive ====================

/// GET /api/heroes/{id}/revive-info - Get revive info for dead hero
pub async fn get_revive_info(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
) -> AppResult<Json<ReviveInfoResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let info = HeroService::get_revive_info(&state.db, db_user.id, hero_id).await?;
    Ok(Json(info))
}

/// POST /api/heroes/{id}/revive - Revive dead hero
pub async fn revive_hero(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(hero_id): Path<Uuid>,
    Json(request): Json<ReviveHeroRequest>,
) -> AppResult<Json<HeroResponse>> {
    let db_user = UserRepository::find_by_firebase_uid(&state.db, &user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let hero = HeroService::revive_hero(&state.db, db_user.id, hero_id, request.use_gold).await?;
    Ok(Json(hero))
}
