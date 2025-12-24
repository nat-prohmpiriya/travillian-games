use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

use super::troop::TroopType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "mission_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MissionType {
    Raid,     // Quick attack, steal resources
    Attack,   // Full attack, kill troops
    Conquer,  // Attack to take over village
    Support,  // Send troops to defend
    Scout,    // Reconnaissance mission
    Settle,   // Found new village with settlers
}

impl MissionType {
    pub fn is_hostile(&self) -> bool {
        matches!(self, MissionType::Raid | MissionType::Attack | MissionType::Conquer | MissionType::Scout)
    }

    pub fn is_support(&self) -> bool {
        matches!(self, MissionType::Support)
    }

    pub fn returns(&self) -> bool {
        // Settle missions don't return
        !matches!(self, MissionType::Settle)
    }
}

/// Troops in an army (serialized as JSON in database)
pub type ArmyTroops = HashMap<TroopType, i32>;

/// Resources carried by army (serialized as JSON in database)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CarriedResources {
    #[serde(default)]
    pub wood: i32,
    #[serde(default)]
    pub clay: i32,
    #[serde(default)]
    pub iron: i32,
    #[serde(default)]
    pub crop: i32,
}

impl CarriedResources {
    pub fn total(&self) -> i32 {
        self.wood + self.clay + self.iron + self.crop
    }
}

/// Army movement record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Army {
    pub id: Uuid,
    pub player_id: Uuid,
    pub from_village_id: Uuid,
    pub to_x: i32,
    pub to_y: i32,
    pub to_village_id: Option<Uuid>,
    pub mission: MissionType,
    pub troops: sqlx::types::Json<ArmyTroops>,
    pub resources: sqlx::types::Json<CarriedResources>,
    pub departed_at: DateTime<Utc>,
    pub arrives_at: DateTime<Utc>,
    pub returns_at: Option<DateTime<Utc>>,
    pub is_returning: bool,
    pub battle_report_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Battle report record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BattleReport {
    pub id: Uuid,
    pub attacker_player_id: Uuid,
    pub defender_player_id: Option<Uuid>,
    pub attacker_village_id: Uuid,
    pub defender_village_id: Option<Uuid>,
    pub mission: MissionType,
    pub attacker_troops: sqlx::types::Json<ArmyTroops>,
    pub defender_troops: sqlx::types::Json<ArmyTroops>,
    pub attacker_losses: sqlx::types::Json<ArmyTroops>,
    pub defender_losses: sqlx::types::Json<ArmyTroops>,
    pub resources_stolen: sqlx::types::Json<CarriedResources>,
    pub winner: String, // "attacker", "defender", "draw"
    pub occurred_at: DateTime<Utc>,
    pub read_by_attacker: bool,
    pub read_by_defender: bool,
    pub created_at: DateTime<Utc>,
}

// Request/Response DTOs

#[derive(Debug, Clone, Deserialize)]
pub struct SendArmyRequest {
    pub to_x: i32,
    pub to_y: i32,
    pub mission: MissionType,
    pub troops: HashMap<TroopType, i32>,
    #[serde(default)]
    pub resources: CarriedResources,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArmyResponse {
    pub id: Uuid,
    pub from_village_id: Uuid,
    pub to_x: i32,
    pub to_y: i32,
    pub to_village_id: Option<Uuid>,
    pub mission: MissionType,
    pub troops: ArmyTroops,
    pub resources: CarriedResources,
    pub departed_at: DateTime<Utc>,
    pub arrives_at: DateTime<Utc>,
    pub returns_at: Option<DateTime<Utc>>,
    pub is_returning: bool,
}

impl From<Army> for ArmyResponse {
    fn from(a: Army) -> Self {
        Self {
            id: a.id,
            from_village_id: a.from_village_id,
            to_x: a.to_x,
            to_y: a.to_y,
            to_village_id: a.to_village_id,
            mission: a.mission,
            troops: a.troops.0,
            resources: a.resources.0,
            departed_at: a.departed_at,
            arrives_at: a.arrives_at,
            returns_at: a.returns_at,
            is_returning: a.is_returning,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BattleReportResponse {
    pub id: Uuid,
    pub attacker_player_id: Uuid,
    pub defender_player_id: Option<Uuid>,
    pub attacker_village_id: Uuid,
    pub defender_village_id: Option<Uuid>,
    pub mission: MissionType,
    pub attacker_troops: ArmyTroops,
    pub defender_troops: ArmyTroops,
    pub attacker_losses: ArmyTroops,
    pub defender_losses: ArmyTroops,
    pub resources_stolen: CarriedResources,
    pub winner: String,
    pub occurred_at: DateTime<Utc>,
    pub is_read: bool,
}

impl BattleReport {
    pub fn to_response(&self, is_attacker: bool) -> BattleReportResponse {
        BattleReportResponse {
            id: self.id,
            attacker_player_id: self.attacker_player_id,
            defender_player_id: self.defender_player_id,
            attacker_village_id: self.attacker_village_id,
            defender_village_id: self.defender_village_id,
            mission: self.mission,
            attacker_troops: self.attacker_troops.0.clone(),
            defender_troops: self.defender_troops.0.clone(),
            attacker_losses: self.attacker_losses.0.clone(),
            defender_losses: self.defender_losses.0.clone(),
            resources_stolen: self.resources_stolen.0.clone(),
            winner: self.winner.clone(),
            occurred_at: self.occurred_at,
            is_read: if is_attacker { self.read_by_attacker } else { self.read_by_defender },
        }
    }
}
