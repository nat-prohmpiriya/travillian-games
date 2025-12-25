use axum::{
    extract::{Path, State},
    Extension, Json,
};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::troop::{
    TrainTroopsRequest, TrainTroopsResponse, TroopDefinitionResponse, TroopQueueResponse,
    TroopResponse,
};
use crate::repositories::user_repo::UserRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::troop_service::TroopService;
use crate::AppState;

// GET /api/troops/definitions - Get all troop definitions (public endpoint)
pub async fn get_definitions(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<TroopDefinitionResponse>>> {
    let definitions = TroopService::get_definitions(&state.db).await?;

    Ok(Json(definitions.into_iter().map(|d| d.into()).collect()))
}

// GET /api/villages/:village_id/troops - Get troops in a village
pub async fn list_troops(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<TroopResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let troops = TroopService::get_village_troops(&state.db, village_id).await?;

    Ok(Json(troops.into_iter().map(|t| t.into()).collect()))
}

// GET /api/villages/:village_id/troops/queue - Get training queue
pub async fn get_training_queue(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<TroopQueueResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let queue = TroopService::get_training_queue(&state.db, village_id).await?;

    Ok(Json(queue.into_iter().map(|q| q.into()).collect()))
}

// POST /api/villages/:village_id/troops/train - Train troops
pub async fn train_troops(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
    Json(body): Json<TrainTroopsRequest>,
) -> AppResult<Json<TrainTroopsResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden("Access denied".into()));
    }

    let response = TroopService::train_troops(&state.db, village_id, body.troop_type, body.count).await?;

    info!(
        "Training {} {:?} in village {}",
        body.count, body.troop_type, village_id
    );

    Ok(Json(response))
}

// DELETE /api/villages/:village_id/troops/queue/:queue_id - Cancel training
pub async fn cancel_training(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path((village_id, queue_id)): Path<(Uuid, Uuid)>,
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

    TroopService::cancel_training(&state.db, village_id, queue_id).await?;

    info!("Training cancelled in village {}", village_id);

    Ok(Json(serde_json::json!({
        "message": "Training cancelled successfully"
    })))
}
