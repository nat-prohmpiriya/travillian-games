use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::troop::{Troop, TroopDefinition, TroopQueue, TroopType};

pub struct TroopRepository;

impl TroopRepository {
    // ==================== Troop Definitions ====================

    pub async fn get_all_definitions(pool: &PgPool) -> AppResult<Vec<TroopDefinition>> {
        let definitions = sqlx::query_as::<_, TroopDefinition>(
            r#"
            SELECT id, troop_type, tribe, name, description,
                   attack, defense_infantry, defense_cavalry, speed,
                   carry_capacity, crop_consumption, training_time_seconds,
                   wood_cost, clay_cost, iron_cost, crop_cost,
                   required_building, required_building_level, loyalty_reduction, created_at
            FROM troop_definitions
            ORDER BY tribe, troop_type
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(definitions)
    }

    pub async fn get_definition(pool: &PgPool, troop_type: TroopType) -> AppResult<Option<TroopDefinition>> {
        let definition = sqlx::query_as::<_, TroopDefinition>(
            r#"
            SELECT id, troop_type, tribe, name, description,
                   attack, defense_infantry, defense_cavalry, speed,
                   carry_capacity, crop_consumption, training_time_seconds,
                   wood_cost, clay_cost, iron_cost, crop_cost,
                   required_building, required_building_level, loyalty_reduction, created_at
            FROM troop_definitions
            WHERE troop_type = $1
            "#,
        )
        .bind(&troop_type)
        .fetch_optional(pool)
        .await?;

        Ok(definition)
    }

    // ==================== Troops ====================

    pub async fn find_by_village(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Troop>> {
        let troops = sqlx::query_as::<_, Troop>(
            r#"
            SELECT id, village_id, troop_type, count, in_village, created_at, updated_at
            FROM troops
            WHERE village_id = $1 AND count > 0
            ORDER BY troop_type
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(troops)
    }

    pub async fn find_by_village_and_type(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
    ) -> AppResult<Option<Troop>> {
        let troop = sqlx::query_as::<_, Troop>(
            r#"
            SELECT id, village_id, troop_type, count, in_village, created_at, updated_at
            FROM troops
            WHERE village_id = $1 AND troop_type = $2
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .fetch_optional(pool)
        .await?;

        Ok(troop)
    }

    pub async fn add_troops(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
    ) -> AppResult<Troop> {
        // Upsert: create or update troop count
        let troop = sqlx::query_as::<_, Troop>(
            r#"
            INSERT INTO troops (village_id, troop_type, count, in_village)
            VALUES ($1, $2, $3, $3)
            ON CONFLICT (village_id, troop_type)
            DO UPDATE SET
                count = troops.count + $3,
                in_village = troops.in_village + $3,
                updated_at = NOW()
            RETURNING id, village_id, troop_type, count, in_village, created_at, updated_at
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .bind(count)
        .fetch_one(pool)
        .await?;

        Ok(troop)
    }

    pub async fn remove_troops_from_village(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
    ) -> AppResult<Troop> {
        // Reduce in_village count (troops sent on mission)
        let troop = sqlx::query_as::<_, Troop>(
            r#"
            UPDATE troops
            SET in_village = in_village - $3,
                updated_at = NOW()
            WHERE village_id = $1 AND troop_type = $2
            RETURNING id, village_id, troop_type, count, in_village, created_at, updated_at
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .bind(count)
        .fetch_one(pool)
        .await?;

        Ok(troop)
    }

    pub async fn return_troops_to_village(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
    ) -> AppResult<Troop> {
        // Return troops from mission to village
        let troop = sqlx::query_as::<_, Troop>(
            r#"
            UPDATE troops
            SET in_village = in_village + $3,
                updated_at = NOW()
            WHERE village_id = $1 AND troop_type = $2
            RETURNING id, village_id, troop_type, count, in_village, created_at, updated_at
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .bind(count)
        .fetch_one(pool)
        .await?;

        Ok(troop)
    }

    pub async fn kill_troops(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
    ) -> AppResult<()> {
        // Permanently remove troops (killed in battle)
        sqlx::query(
            r#"
            UPDATE troops
            SET count = count - $3,
                in_village = GREATEST(0, in_village - $3),
                updated_at = NOW()
            WHERE village_id = $1 AND troop_type = $2
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .bind(count)
        .execute(pool)
        .await?;

        Ok(())
    }

    // ==================== Training Queue ====================

    pub async fn get_queue_by_village(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<TroopQueue>> {
        let queue = sqlx::query_as::<_, TroopQueue>(
            r#"
            SELECT id, village_id, troop_type, count, each_duration_seconds,
                   started_at, ends_at, created_at
            FROM troop_queue
            WHERE village_id = $1
            ORDER BY ends_at ASC
            "#,
        )
        .bind(village_id)
        .fetch_all(pool)
        .await?;

        Ok(queue)
    }

    pub async fn add_to_queue(
        pool: &PgPool,
        village_id: Uuid,
        troop_type: TroopType,
        count: i32,
        each_duration_seconds: i32,
        started_at: DateTime<Utc>,
        ends_at: DateTime<Utc>,
    ) -> AppResult<TroopQueue> {
        let queue_entry = sqlx::query_as::<_, TroopQueue>(
            r#"
            INSERT INTO troop_queue (village_id, troop_type, count, each_duration_seconds, started_at, ends_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, village_id, troop_type, count, each_duration_seconds, started_at, ends_at, created_at
            "#,
        )
        .bind(village_id)
        .bind(&troop_type)
        .bind(count)
        .bind(each_duration_seconds)
        .bind(started_at)
        .bind(ends_at)
        .fetch_one(pool)
        .await?;

        Ok(queue_entry)
    }

    pub async fn remove_from_queue(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM troop_queue WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn find_completed_training(pool: &PgPool) -> AppResult<Vec<TroopQueue>> {
        let completed = sqlx::query_as::<_, TroopQueue>(
            r#"
            SELECT id, village_id, troop_type, count, each_duration_seconds,
                   started_at, ends_at, created_at
            FROM troop_queue
            WHERE ends_at <= NOW()
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(completed)
    }

    pub async fn get_last_queue_end_time(pool: &PgPool, village_id: Uuid) -> AppResult<Option<DateTime<Utc>>> {
        let result: Option<(DateTime<Utc>,)> = sqlx::query_as(
            r#"
            SELECT MAX(ends_at) FROM troop_queue WHERE village_id = $1
            "#,
        )
        .bind(village_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|r| r.0))
    }

    // ==================== Crop Consumption ====================

    pub async fn get_total_crop_consumption(pool: &PgPool, village_id: Uuid) -> AppResult<i32> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(SUM(t.count * td.crop_consumption), 0)
            FROM troops t
            JOIN troop_definitions td ON t.troop_type = td.troop_type
            WHERE t.village_id = $1
            "#,
        )
        .bind(village_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0 as i32)
    }
}
