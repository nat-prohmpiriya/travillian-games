use axum::{
    extract::{Path, State},
    Extension, Json,
};
use tracing::info;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::middleware::AuthenticatedUser;
use crate::models::army::{ArmyResponse, BattleReportResponse, ScoutReportResponse, SendArmyRequest};
use crate::repositories::army_repo::ArmyRepository;
use crate::repositories::user_repo::UserRepository;
use crate::repositories::village_repo::VillageRepository;
use crate::services::army_service::ArmyService;
use crate::AppState;

// POST /api/villages/:village_id/armies - Send army
pub async fn send_army(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
    Json(body): Json<SendArmyRequest>,
) -> AppResult<Json<ArmyResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden);
    }

    let response = ArmyService::send_army(&state.db, user.id, village_id, body).await?;

    info!(
        "Army sent from village {} to ({}, {})",
        village_id, response.to_x, response.to_y
    );

    Ok(Json(response))
}

// GET /api/villages/:village_id/armies/outgoing - List outgoing armies
pub async fn list_outgoing(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<ArmyResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden);
    }

    let armies = ArmyService::get_outgoing_armies(&state.db, village_id).await?;

    Ok(Json(armies))
}

// GET /api/villages/:village_id/armies/incoming - List incoming armies
pub async fn list_incoming(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(village_id): Path<Uuid>,
) -> AppResult<Json<Vec<ArmyResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let village = VillageRepository::find_by_id(&state.db, village_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Village not found".to_string()))?;

    if village.user_id != user.id {
        return Err(AppError::Forbidden);
    }

    let armies = ArmyService::get_incoming_armies(&state.db, village_id).await?;

    Ok(Json(armies.into_iter().map(|a| a.into()).collect()))
}

// GET /api/reports - List battle reports
pub async fn list_reports(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<BattleReportResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let reports = ArmyService::get_reports(&state.db, user.id).await?;

    let responses: Vec<BattleReportResponse> = reports
        .into_iter()
        .map(|r| {
            let is_attacker = r.attacker_player_id == user.id;
            r.to_response(is_attacker)
        })
        .collect();

    Ok(Json(responses))
}

// GET /api/reports/:report_id - Get single report
pub async fn get_report(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(report_id): Path<Uuid>,
) -> AppResult<Json<BattleReportResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let report = ArmyService::get_report(&state.db, report_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Report not found".to_string()))?;

    // Check if user is involved in this battle
    let is_attacker = report.attacker_player_id == user.id;
    let is_defender = report.defender_player_id == Some(user.id);

    if !is_attacker && !is_defender {
        return Err(AppError::Forbidden);
    }

    Ok(Json(report.to_response(is_attacker)))
}

// POST /api/reports/:report_id/read - Mark report as read
pub async fn mark_report_read(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(report_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    ArmyService::mark_report_read(&state.db, report_id, user.id).await?;

    Ok(Json(serde_json::json!({
        "message": "Report marked as read"
    })))
}

// GET /api/reports/unread-count - Get unread report count (battle + scout)
pub async fn get_unread_count(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<serde_json::Value>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let count = ArmyService::get_total_unread_count(&state.db, user.id).await?;

    Ok(Json(serde_json::json!({
        "unread_count": count
    })))
}

// ==================== Scout Reports ====================

// GET /api/scout-reports - List scout reports
pub async fn list_scout_reports(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
) -> AppResult<Json<Vec<ScoutReportResponse>>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let reports = ArmyService::get_scout_reports(&state.db, user.id).await?;

    let responses: Vec<ScoutReportResponse> = reports
        .into_iter()
        .map(|r| {
            let is_attacker = r.attacker_player_id == user.id;
            r.to_response(is_attacker)
        })
        .collect();

    Ok(Json(responses))
}

// GET /api/scout-reports/:report_id - Get single scout report
pub async fn get_scout_report(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(report_id): Path<Uuid>,
) -> AppResult<Json<ScoutReportResponse>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let report = ArmyService::get_scout_report(&state.db, report_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Scout report not found".to_string()))?;

    // Check if user is involved
    let is_attacker = report.attacker_player_id == user.id;
    let is_defender = report.defender_player_id == Some(user.id);

    if !is_attacker && !is_defender {
        return Err(AppError::Forbidden);
    }

    Ok(Json(report.to_response(is_attacker)))
}

// POST /api/scout-reports/:report_id/read - Mark scout report as read
pub async fn mark_scout_report_read(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthenticatedUser>,
    Path(report_id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let user = UserRepository::find_by_firebase_uid(&state.db, &auth_user.firebase_uid)
        .await?
        .ok_or(AppError::Unauthorized)?;

    ArmyService::mark_scout_report_read(&state.db, report_id, user.id).await?;

    Ok(Json(serde_json::json!({
        "message": "Scout report marked as read"
    })))
}
