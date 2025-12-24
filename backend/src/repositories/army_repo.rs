use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::army::{Army, ArmyTroops, BattleReport, CarriedResources, MissionType};

pub struct ArmyRepository;

impl ArmyRepository {
    // ==================== Armies ====================

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Army>> {
        let army = sqlx::query_as::<_, Army>(
            r#"
            SELECT id, player_id, from_village_id, to_x, to_y, to_village_id,
                   mission, troops, resources, departed_at, arrives_at,
                   returns_at, is_returning, battle_report_id, created_at
            FROM armies
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(army)
    }

    pub async fn find_by_player(pool: &PgPool, player_id: Uuid) -> AppResult<Vec<Army>> {
        let armies = sqlx::query_as::<_, Army>(
            r#"
            SELECT id, player_id, from_village_id, to_x, to_y, to_village_id,
                   mission, troops, resources, departed_at, arrives_at,
                   returns_at, is_returning, battle_report_id, created_at
            FROM armies
            WHERE player_id = $1
            ORDER BY arrives_at ASC
            "#,
        )
        .bind(player_id)
        .fetch_all(pool)
        .await?;

        Ok(armies)
    }

    pub async fn find_outgoing_from_village(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Army>> {
        let armies = sqlx::query_as::<_, Army>(
            r#"
            SELECT id, player_id, from_village_id, to_x, to_y, to_village_id,
                   mission, troops, resources, departed_at, arrives_at,
                   returns_at, is_returning, battle_report_id, created_at
            FROM armies
            WHERE from_village_id = $1
            ORDER BY arrives_at ASC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(armies)
    }

    pub async fn find_incoming_to_village(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Army>> {
        let armies = sqlx::query_as::<_, Army>(
            r#"
            SELECT id, player_id, from_village_id, to_x, to_y, to_village_id,
                   mission, troops, resources, departed_at, arrives_at,
                   returns_at, is_returning, battle_report_id, created_at
            FROM armies
            WHERE to_village_id = $1 AND is_returning = FALSE
            ORDER BY arrives_at ASC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(armies)
    }

    pub async fn create(
        pool: &PgPool,
        player_id: Uuid,
        from_village_id: Uuid,
        to_x: i32,
        to_y: i32,
        to_village_id: Option<Uuid>,
        mission: MissionType,
        troops: &ArmyTroops,
        resources: &CarriedResources,
        departed_at: DateTime<Utc>,
        arrives_at: DateTime<Utc>,
        returns_at: Option<DateTime<Utc>>,
    ) -> AppResult<Army> {
        let army = sqlx::query_as::<_, Army>(
            r#"
            INSERT INTO armies (player_id, from_village_id, to_x, to_y, to_village_id,
                               mission, troops, resources, departed_at, arrives_at, returns_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, player_id, from_village_id, to_x, to_y, to_village_id,
                      mission, troops, resources, departed_at, arrives_at,
                      returns_at, is_returning, battle_report_id, created_at
            "#,
        )
        .bind(player_id)
        .bind(from_village_id)
        .bind(to_x)
        .bind(to_y)
        .bind(to_village_id)
        .bind(&mission)
        .bind(sqlx::types::Json(troops))
        .bind(sqlx::types::Json(resources))
        .bind(departed_at)
        .bind(arrives_at)
        .bind(returns_at)
        .fetch_one(pool)
        .await?;

        Ok(army)
    }

    pub async fn set_returning(
        pool: &PgPool,
        id: Uuid,
        returns_at: DateTime<Utc>,
        resources: &CarriedResources,
        surviving_troops: &ArmyTroops,
        battle_report_id: Option<Uuid>,
    ) -> AppResult<Army> {
        let army = sqlx::query_as::<_, Army>(
            r#"
            UPDATE armies
            SET is_returning = TRUE,
                arrives_at = $2,
                resources = $3,
                troops = $4,
                battle_report_id = $5
            WHERE id = $1
            RETURNING id, player_id, from_village_id, to_x, to_y, to_village_id,
                      mission, troops, resources, departed_at, arrives_at,
                      returns_at, is_returning, battle_report_id, created_at
            "#,
        )
        .bind(id)
        .bind(returns_at)
        .bind(sqlx::types::Json(resources))
        .bind(sqlx::types::Json(surviving_troops))
        .bind(battle_report_id)
        .fetch_one(pool)
        .await?;

        Ok(army)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM armies WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn find_arrived(pool: &PgPool) -> AppResult<Vec<Army>> {
        let armies = sqlx::query_as::<_, Army>(
            r#"
            SELECT id, player_id, from_village_id, to_x, to_y, to_village_id,
                   mission, troops, resources, departed_at, arrives_at,
                   returns_at, is_returning, battle_report_id, created_at
            FROM armies
            WHERE arrives_at <= NOW()
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(armies)
    }

    // ==================== Battle Reports ====================

    pub async fn create_battle_report(
        pool: &PgPool,
        attacker_player_id: Uuid,
        defender_player_id: Option<Uuid>,
        attacker_village_id: Uuid,
        defender_village_id: Option<Uuid>,
        mission: MissionType,
        attacker_troops: &ArmyTroops,
        defender_troops: &ArmyTroops,
        attacker_losses: &ArmyTroops,
        defender_losses: &ArmyTroops,
        resources_stolen: &CarriedResources,
        winner: &str,
        occurred_at: DateTime<Utc>,
    ) -> AppResult<BattleReport> {
        let report = sqlx::query_as::<_, BattleReport>(
            r#"
            INSERT INTO battle_reports (
                attacker_player_id, defender_player_id, attacker_village_id, defender_village_id,
                mission, attacker_troops, defender_troops, attacker_losses, defender_losses,
                resources_stolen, winner, occurred_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id, attacker_player_id, defender_player_id, attacker_village_id, defender_village_id,
                      mission, attacker_troops, defender_troops, attacker_losses, defender_losses,
                      resources_stolen, winner, occurred_at, read_by_attacker, read_by_defender, created_at
            "#,
        )
        .bind(attacker_player_id)
        .bind(defender_player_id)
        .bind(attacker_village_id)
        .bind(defender_village_id)
        .bind(&mission)
        .bind(sqlx::types::Json(attacker_troops))
        .bind(sqlx::types::Json(defender_troops))
        .bind(sqlx::types::Json(attacker_losses))
        .bind(sqlx::types::Json(defender_losses))
        .bind(sqlx::types::Json(resources_stolen))
        .bind(winner)
        .bind(occurred_at)
        .fetch_one(pool)
        .await?;

        Ok(report)
    }

    pub async fn find_reports_by_player(pool: &PgPool, player_id: Uuid) -> AppResult<Vec<BattleReport>> {
        let reports = sqlx::query_as::<_, BattleReport>(
            r#"
            SELECT id, attacker_player_id, defender_player_id, attacker_village_id, defender_village_id,
                   mission, attacker_troops, defender_troops, attacker_losses, defender_losses,
                   resources_stolen, winner, occurred_at, read_by_attacker, read_by_defender, created_at
            FROM battle_reports
            WHERE attacker_player_id = $1 OR defender_player_id = $1
            ORDER BY occurred_at DESC
            LIMIT 100
            "#,
        )
        .bind(player_id)
        .fetch_all(pool)
        .await?;

        Ok(reports)
    }

    pub async fn find_report_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<BattleReport>> {
        let report = sqlx::query_as::<_, BattleReport>(
            r#"
            SELECT id, attacker_player_id, defender_player_id, attacker_village_id, defender_village_id,
                   mission, attacker_troops, defender_troops, attacker_losses, defender_losses,
                   resources_stolen, winner, occurred_at, read_by_attacker, read_by_defender, created_at
            FROM battle_reports
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(report)
    }

    pub async fn mark_report_read(pool: &PgPool, id: Uuid, is_attacker: bool) -> AppResult<()> {
        let query = if is_attacker {
            "UPDATE battle_reports SET read_by_attacker = TRUE WHERE id = $1"
        } else {
            "UPDATE battle_reports SET read_by_defender = TRUE WHERE id = $1"
        };

        sqlx::query(query).bind(id).execute(pool).await?;

        Ok(())
    }

    pub async fn count_unread_reports(pool: &PgPool, player_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM battle_reports
            WHERE (attacker_player_id = $1 AND read_by_attacker = FALSE)
               OR (defender_player_id = $1 AND read_by_defender = FALSE)
            "#,
        )
        .bind(player_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }
}
