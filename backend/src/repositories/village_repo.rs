use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::village::{CreateVillage, UpdateVillage, Village, VillageMapInfo};

pub struct VillageRepository;

impl VillageRepository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Village>> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            SELECT id, user_id, name, x, y, is_capital,
                   wood, clay, iron, crop,
                   warehouse_capacity, granary_capacity,
                   population, culture_points, loyalty,
                   resources_updated_at, created_at, updated_at
            FROM villages
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(village)
    }

    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<Village>> {
        let villages = sqlx::query_as::<_, Village>(
            r#"
            SELECT id, user_id, name, x, y, is_capital,
                   wood, clay, iron, crop,
                   warehouse_capacity, granary_capacity,
                   population, culture_points, loyalty,
                   resources_updated_at, created_at, updated_at
            FROM villages
            WHERE user_id = $1
            ORDER BY is_capital DESC, created_at ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(villages)
    }

    pub async fn find_by_coordinates(pool: &PgPool, x: i32, y: i32) -> AppResult<Option<Village>> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            SELECT id, user_id, name, x, y, is_capital,
                   wood, clay, iron, crop,
                   warehouse_capacity, granary_capacity,
                   population, culture_points, loyalty,
                   resources_updated_at, created_at, updated_at
            FROM villages
            WHERE x = $1 AND y = $2
            "#,
        )
        .bind(x)
        .bind(y)
        .fetch_optional(pool)
        .await?;

        Ok(village)
    }

    pub async fn find_in_range(
        pool: &PgPool,
        center_x: i32,
        center_y: i32,
        range: i32,
    ) -> AppResult<Vec<VillageMapInfo>> {
        let villages = sqlx::query_as::<_, VillageMapInfo>(
            r#"
            SELECT v.id, v.user_id, v.name, v.x, v.y, v.population,
                   u.display_name as player_name
            FROM villages v
            LEFT JOIN users u ON v.user_id = u.id
            WHERE v.x BETWEEN $1 AND $2
              AND v.y BETWEEN $3 AND $4
            "#,
        )
        .bind(center_x - range)
        .bind(center_x + range)
        .bind(center_y - range)
        .bind(center_y + range)
        .fetch_all(pool)
        .await?;

        Ok(villages)
    }

    pub async fn create(pool: &PgPool, input: CreateVillage) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            INSERT INTO villages (user_id, name, x, y, is_capital)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(&input.user_id)
        .bind(&input.name)
        .bind(input.x)
        .bind(input.y)
        .bind(input.is_capital)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn update(pool: &PgPool, id: Uuid, input: UpdateVillage) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET name = COALESCE($2, name),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&input.name)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn update_resources(
        pool: &PgPool,
        id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET wood = $2, clay = $3, iron = $4, crop = $5,
                resources_updated_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn deduct_resources(
        pool: &PgPool,
        id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET wood = wood - $2,
                clay = clay - $3,
                iron = iron - $4,
                crop = crop - $5,
                updated_at = NOW()
            WHERE id = $1
              AND wood >= $2
              AND clay >= $3
              AND iron >= $4
              AND crop >= $5
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn update_storage_capacity(
        pool: &PgPool,
        id: Uuid,
        warehouse_capacity: i32,
        granary_capacity: i32,
    ) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET warehouse_capacity = $2,
                granary_capacity = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(warehouse_capacity)
        .bind(granary_capacity)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn update_population(pool: &PgPool, id: Uuid, population: i32) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET population = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(population)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn count_by_user_id(pool: &PgPool, user_id: Uuid) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM villages WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(count.0)
    }

    pub async fn is_coordinate_available(pool: &PgPool, x: i32, y: i32) -> AppResult<bool> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(SELECT 1 FROM villages WHERE x = $1 AND y = $2)
            "#,
        )
        .bind(x)
        .bind(y)
        .fetch_one(pool)
        .await?;

        Ok(!exists.0)
    }

    pub async fn add_resources(
        pool: &PgPool,
        id: Uuid,
        wood: i32,
        clay: i32,
        iron: i32,
        crop: i32,
    ) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET wood = LEAST(wood + $2, warehouse_capacity),
                clay = LEAST(clay + $3, warehouse_capacity),
                iron = LEAST(iron + $4, warehouse_capacity),
                crop = LEAST(crop + $5, granary_capacity),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(wood)
        .bind(clay)
        .bind(iron)
        .bind(crop)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    // ==================== Conquer-related ====================

    pub async fn update_loyalty(pool: &PgPool, id: Uuid, loyalty: i32) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET loyalty = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(loyalty)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }

    pub async fn transfer_ownership(pool: &PgPool, id: Uuid, new_owner_id: Uuid) -> AppResult<Village> {
        let village = sqlx::query_as::<_, Village>(
            r#"
            UPDATE villages
            SET user_id = $2,
                is_capital = false,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, name, x, y, is_capital,
                      wood, clay, iron, crop,
                      warehouse_capacity, granary_capacity,
                      population, culture_points, loyalty,
                      resources_updated_at, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(new_owner_id)
        .fetch_one(pool)
        .await?;

        Ok(village)
    }
}
