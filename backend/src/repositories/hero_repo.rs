use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::hero::{
    AvailableAdventure, Hero, HeroAdventure, HeroItem, HeroItemWithDefinition, HeroSlotPrice,
    HeroStatus, ItemDefinition, ItemRarity, ItemSlot, AdventureDifficulty,
};
use crate::models::troop::TribeType;

pub struct HeroRepository;

impl HeroRepository {
    // ==================== Heroes ====================

    /// Get all heroes for a user
    pub async fn get_user_heroes(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<Hero>> {
        let heroes = sqlx::query_as::<_, Hero>(
            r#"
            SELECT id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                   status, level, experience, experience_to_next, health, health_regen_rate,
                   unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                   base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                   created_at, updated_at
            FROM heroes
            WHERE user_id = $1
            ORDER BY slot_number
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(heroes)
    }

    /// Get hero by ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Hero>> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            SELECT id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                   status, level, experience, experience_to_next, health, health_regen_rate,
                   unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                   base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                   created_at, updated_at
            FROM heroes
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(hero)
    }

    /// Get hero by user and slot
    pub async fn find_by_slot(pool: &PgPool, user_id: Uuid, slot: i32) -> AppResult<Option<Hero>> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            SELECT id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                   status, level, experience, experience_to_next, health, health_regen_rate,
                   unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                   base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                   created_at, updated_at
            FROM heroes
            WHERE user_id = $1 AND slot_number = $2
            "#,
        )
        .bind(user_id)
        .bind(slot)
        .fetch_optional(pool)
        .await?;

        Ok(hero)
    }

    /// Count user's heroes
    pub async fn count_user_heroes(pool: &PgPool, user_id: Uuid) -> AppResult<i32> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM heroes WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0 as i32)
    }

    /// Get user's hero slots count
    pub async fn get_user_slots(pool: &PgPool, user_id: Uuid) -> AppResult<i32> {
        let result: (i32,) = sqlx::query_as(
            "SELECT hero_slots FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Create a new hero
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        slot_number: i32,
        name: &str,
        tribe: TribeType,
        home_village_id: Uuid,
    ) -> AppResult<Hero> {
        // Base stats based on tribe
        let (base_attack, base_defense, base_speed) = match tribe {
            TribeType::Phasuttha => (80, 80, 7.0),  // Balanced
            TribeType::Nava => (70, 90, 8.0),       // Defensive, faster
            TribeType::Kiri => (90, 70, 6.0),       // Offensive, slower
            TribeType::Special => (80, 80, 7.0),
        };

        let hero = sqlx::query_as::<_, Hero>(
            r#"
            INSERT INTO heroes (
                user_id, slot_number, name, tribe, home_village_id, current_village_id,
                base_attack, base_defense, base_speed
            )
            VALUES ($1, $2, $3, $4, $5, $5, $6, $7, $8)
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(slot_number)
        .bind(name)
        .bind(&tribe)
        .bind(home_village_id)
        .bind(base_attack)
        .bind(base_defense)
        .bind(base_speed)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Update hero home village
    pub async fn update_home_village(
        pool: &PgPool,
        hero_id: Uuid,
        village_id: Uuid,
    ) -> AppResult<Hero> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET home_village_id = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(village_id)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Update hero status
    pub async fn update_status(
        pool: &PgPool,
        hero_id: Uuid,
        status: HeroStatus,
    ) -> AppResult<Hero> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(&status)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Assign attribute points
    pub async fn assign_attributes(
        pool: &PgPool,
        hero_id: Uuid,
        fighting_strength: i32,
        off_bonus: i32,
        def_bonus: i32,
        resources_bonus: i32,
        points_spent: i32,
    ) -> AppResult<Hero> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET fighting_strength = fighting_strength + $2,
                off_bonus = off_bonus + $3,
                def_bonus = def_bonus + $4,
                resources_bonus = resources_bonus + $5,
                unassigned_points = unassigned_points - $6,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(fighting_strength)
        .bind(off_bonus)
        .bind(def_bonus)
        .bind(resources_bonus)
        .bind(points_spent)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Add experience to hero
    pub async fn add_experience(pool: &PgPool, hero_id: Uuid, exp: i32) -> AppResult<Hero> {
        // Get current hero
        let hero = Self::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound("Hero not found".into()))?;

        let mut new_exp = hero.experience + exp;
        let mut new_level = hero.level;
        let mut new_points = hero.unassigned_points;
        let mut exp_to_next = hero.experience_to_next;

        // Level up loop
        while new_exp >= exp_to_next {
            new_exp -= exp_to_next;
            new_level += 1;
            new_points += 4; // 4 attribute points per level
            exp_to_next = Hero::exp_for_level(new_level + 1);
        }

        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET experience = $2,
                level = $3,
                unassigned_points = $4,
                experience_to_next = $5,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(new_exp)
        .bind(new_level)
        .bind(new_points)
        .bind(exp_to_next)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Update hero health
    pub async fn update_health(pool: &PgPool, hero_id: Uuid, health: i32) -> AppResult<Hero> {
        let health = health.clamp(0, 100);
        let (status, died_at): (HeroStatus, Option<DateTime<Utc>>) = if health <= 0 {
            (HeroStatus::Dead, Some(Utc::now()))
        } else {
            (HeroStatus::Idle, None)
        };

        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET health = $2,
                status = $3,
                died_at = COALESCE($4, died_at),
                last_health_update = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(health)
        .bind(&status)
        .bind(died_at)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Damage hero (reduce health)
    pub async fn damage_hero(pool: &PgPool, hero_id: Uuid, damage: i32) -> AppResult<Hero> {
        let hero = Self::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| crate::error::AppError::NotFound("Hero not found".into()))?;

        let new_health = (hero.health - damage).max(0);
        Self::update_health(pool, hero_id, new_health).await
    }

    /// Kill hero
    pub async fn kill_hero(pool: &PgPool, hero_id: Uuid) -> AppResult<Hero> {
        let revive_at = Utc::now() + chrono::Duration::hours(24); // 24 hour revive time

        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET health = 0,
                status = 'dead',
                died_at = NOW(),
                revive_at = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(revive_at)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    /// Revive hero
    pub async fn revive_hero(pool: &PgPool, hero_id: Uuid, health: i32) -> AppResult<Hero> {
        let hero = sqlx::query_as::<_, Hero>(
            r#"
            UPDATE heroes
            SET health = $2,
                status = 'idle',
                died_at = NULL,
                revive_at = NULL,
                last_health_update = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, slot_number, name, tribe, home_village_id, current_village_id,
                      status, level, experience, experience_to_next, health, health_regen_rate,
                      unassigned_points, fighting_strength, off_bonus, def_bonus, resources_bonus,
                      base_attack, base_defense, base_speed, last_health_update, died_at, revive_at,
                      created_at, updated_at
            "#,
        )
        .bind(hero_id)
        .bind(health)
        .fetch_one(pool)
        .await?;

        Ok(hero)
    }

    // ==================== Hero Slots ====================

    /// Get hero slot prices
    pub async fn get_slot_prices(pool: &PgPool) -> AppResult<Vec<HeroSlotPrice>> {
        let prices = sqlx::query_as::<_, HeroSlotPrice>(
            "SELECT slot_number, gold_cost FROM hero_slot_prices ORDER BY slot_number"
        )
        .fetch_all(pool)
        .await?;

        Ok(prices)
    }

    /// Get price for specific slot
    pub async fn get_slot_price(pool: &PgPool, slot: i32) -> AppResult<Option<HeroSlotPrice>> {
        let price = sqlx::query_as::<_, HeroSlotPrice>(
            "SELECT slot_number, gold_cost FROM hero_slot_prices WHERE slot_number = $1"
        )
        .bind(slot)
        .fetch_optional(pool)
        .await?;

        Ok(price)
    }

    /// Add hero slot to user
    pub async fn add_user_slot(pool: &PgPool, user_id: Uuid) -> AppResult<i32> {
        let result: (i32,) = sqlx::query_as(
            r#"
            UPDATE users
            SET hero_slots = hero_slots + 1
            WHERE id = $1
            RETURNING hero_slots
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    // ==================== Item Definitions ====================

    /// Get all item definitions
    pub async fn get_all_items(pool: &PgPool) -> AppResult<Vec<ItemDefinition>> {
        let items = sqlx::query_as::<_, ItemDefinition>(
            r#"
            SELECT id, name, description, slot, rarity, required_level,
                   attack_bonus, defense_bonus, speed_bonus, health_regen_bonus,
                   experience_bonus, resource_bonus, carry_bonus,
                   health_restore, is_consumable, extra_inventory_slots,
                   sell_value, can_drop_adventure, can_buy_auction, created_at
            FROM item_definitions
            ORDER BY slot, rarity, name
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    /// Get item definition by ID
    pub async fn get_item_definition(pool: &PgPool, id: Uuid) -> AppResult<Option<ItemDefinition>> {
        let item = sqlx::query_as::<_, ItemDefinition>(
            r#"
            SELECT id, name, description, slot, rarity, required_level,
                   attack_bonus, defense_bonus, speed_bonus, health_regen_bonus,
                   experience_bonus, resource_bonus, carry_bonus,
                   health_restore, is_consumable, extra_inventory_slots,
                   sell_value, can_drop_adventure, can_buy_auction, created_at
            FROM item_definitions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(item)
    }

    /// Get items by rarity for adventure drops
    pub async fn get_items_by_rarity(
        pool: &PgPool,
        rarity: ItemRarity,
    ) -> AppResult<Vec<ItemDefinition>> {
        let items = sqlx::query_as::<_, ItemDefinition>(
            r#"
            SELECT id, name, description, slot, rarity, required_level,
                   attack_bonus, defense_bonus, speed_bonus, health_regen_bonus,
                   experience_bonus, resource_bonus, carry_bonus,
                   health_restore, is_consumable, extra_inventory_slots,
                   sell_value, can_drop_adventure, can_buy_auction, created_at
            FROM item_definitions
            WHERE rarity = $1 AND can_drop_adventure = TRUE
            "#,
        )
        .bind(&rarity)
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    // ==================== Hero Items (Inventory) ====================

    /// Get hero's inventory
    pub async fn get_hero_items(pool: &PgPool, hero_id: Uuid) -> AppResult<Vec<(HeroItem, ItemDefinition)>> {
        let items = sqlx::query_as::<_, HeroItemWithDefinition>(
            r#"
            SELECT hi.id, hi.hero_id, hi.item_definition_id, hi.is_equipped, hi.equipped_slot,
                   hi.quantity, hi.obtained_at, hi.equipped_at,
                   id.id as item_id, id.name as item_name, id.description as item_description,
                   id.slot as item_slot, id.rarity as item_rarity, id.required_level as item_required_level,
                   id.attack_bonus as item_attack_bonus, id.defense_bonus as item_defense_bonus,
                   id.speed_bonus as item_speed_bonus, id.health_regen_bonus as item_health_regen_bonus,
                   id.experience_bonus as item_experience_bonus, id.resource_bonus as item_resource_bonus,
                   id.carry_bonus as item_carry_bonus, id.health_restore as item_health_restore,
                   id.is_consumable as item_is_consumable, id.extra_inventory_slots as item_extra_inventory_slots,
                   id.sell_value as item_sell_value
            FROM hero_items hi
            JOIN item_definitions id ON hi.item_definition_id = id.id
            WHERE hi.hero_id = $1
            ORDER BY hi.is_equipped DESC, id.slot, id.rarity DESC
            "#,
        )
        .bind(hero_id)
        .fetch_all(pool)
        .await?;

        Ok(items.into_iter().map(|i| i.into_parts()).collect())
    }

    /// Get hero's equipped items
    pub async fn get_equipped_items(pool: &PgPool, hero_id: Uuid) -> AppResult<Vec<(HeroItem, ItemDefinition)>> {
        let items = sqlx::query_as::<_, HeroItemWithDefinition>(
            r#"
            SELECT hi.id, hi.hero_id, hi.item_definition_id, hi.is_equipped, hi.equipped_slot,
                   hi.quantity, hi.obtained_at, hi.equipped_at,
                   id.id as item_id, id.name as item_name, id.description as item_description,
                   id.slot as item_slot, id.rarity as item_rarity, id.required_level as item_required_level,
                   id.attack_bonus as item_attack_bonus, id.defense_bonus as item_defense_bonus,
                   id.speed_bonus as item_speed_bonus, id.health_regen_bonus as item_health_regen_bonus,
                   id.experience_bonus as item_experience_bonus, id.resource_bonus as item_resource_bonus,
                   id.carry_bonus as item_carry_bonus, id.health_restore as item_health_restore,
                   id.is_consumable as item_is_consumable, id.extra_inventory_slots as item_extra_inventory_slots,
                   id.sell_value as item_sell_value
            FROM hero_items hi
            JOIN item_definitions id ON hi.item_definition_id = id.id
            WHERE hi.hero_id = $1 AND hi.is_equipped = TRUE
            "#,
        )
        .bind(hero_id)
        .fetch_all(pool)
        .await?;

        Ok(items.into_iter().map(|i| i.into_parts()).collect())
    }

    /// Get hero item by ID
    pub async fn get_hero_item(pool: &PgPool, item_id: Uuid) -> AppResult<Option<(HeroItem, ItemDefinition)>> {
        let item = sqlx::query_as::<_, HeroItemWithDefinition>(
            r#"
            SELECT hi.id, hi.hero_id, hi.item_definition_id, hi.is_equipped, hi.equipped_slot,
                   hi.quantity, hi.obtained_at, hi.equipped_at,
                   id.id as item_id, id.name as item_name, id.description as item_description,
                   id.slot as item_slot, id.rarity as item_rarity, id.required_level as item_required_level,
                   id.attack_bonus as item_attack_bonus, id.defense_bonus as item_defense_bonus,
                   id.speed_bonus as item_speed_bonus, id.health_regen_bonus as item_health_regen_bonus,
                   id.experience_bonus as item_experience_bonus, id.resource_bonus as item_resource_bonus,
                   id.carry_bonus as item_carry_bonus, id.health_restore as item_health_restore,
                   id.is_consumable as item_is_consumable, id.extra_inventory_slots as item_extra_inventory_slots,
                   id.sell_value as item_sell_value
            FROM hero_items hi
            JOIN item_definitions id ON hi.item_definition_id = id.id
            WHERE hi.id = $1
            "#,
        )
        .bind(item_id)
        .fetch_optional(pool)
        .await?;

        Ok(item.map(|i| i.into_parts()))
    }

    /// Add item to hero's inventory
    pub async fn add_item(
        pool: &PgPool,
        hero_id: Uuid,
        item_def_id: Uuid,
        quantity: i32,
    ) -> AppResult<HeroItem> {
        let item = sqlx::query_as::<_, HeroItem>(
            r#"
            INSERT INTO hero_items (hero_id, item_definition_id, quantity)
            VALUES ($1, $2, $3)
            RETURNING id, hero_id, item_definition_id, is_equipped, equipped_slot,
                      quantity, obtained_at, equipped_at
            "#,
        )
        .bind(hero_id)
        .bind(item_def_id)
        .bind(quantity)
        .fetch_one(pool)
        .await?;

        Ok(item)
    }

    /// Equip item
    pub async fn equip_item(pool: &PgPool, item_id: Uuid, slot: ItemSlot) -> AppResult<HeroItem> {
        let item = sqlx::query_as::<_, HeroItem>(
            r#"
            UPDATE hero_items
            SET is_equipped = TRUE, equipped_slot = $2, equipped_at = NOW()
            WHERE id = $1
            RETURNING id, hero_id, item_definition_id, is_equipped, equipped_slot,
                      quantity, obtained_at, equipped_at
            "#,
        )
        .bind(item_id)
        .bind(&slot)
        .fetch_one(pool)
        .await?;

        Ok(item)
    }

    /// Unequip item
    pub async fn unequip_item(pool: &PgPool, item_id: Uuid) -> AppResult<HeroItem> {
        let item = sqlx::query_as::<_, HeroItem>(
            r#"
            UPDATE hero_items
            SET is_equipped = FALSE, equipped_slot = NULL, equipped_at = NULL
            WHERE id = $1
            RETURNING id, hero_id, item_definition_id, is_equipped, equipped_slot,
                      quantity, obtained_at, equipped_at
            "#,
        )
        .bind(item_id)
        .fetch_one(pool)
        .await?;

        Ok(item)
    }

    /// Unequip all items in a slot
    pub async fn unequip_slot(pool: &PgPool, hero_id: Uuid, slot: ItemSlot) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE hero_items
            SET is_equipped = FALSE, equipped_slot = NULL, equipped_at = NULL
            WHERE hero_id = $1 AND equipped_slot = $2
            "#,
        )
        .bind(hero_id)
        .bind(&slot)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Use consumable item (reduce quantity or delete)
    pub async fn use_item(pool: &PgPool, item_id: Uuid) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE hero_items
            SET quantity = quantity - 1
            WHERE id = $1
            "#,
        )
        .bind(item_id)
        .execute(pool)
        .await?;

        // Delete if quantity is 0
        sqlx::query("DELETE FROM hero_items WHERE id = $1 AND quantity <= 0")
            .bind(item_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Delete item
    pub async fn delete_item(pool: &PgPool, item_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM hero_items WHERE id = $1")
            .bind(item_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // ==================== Adventures ====================

    /// Get available adventures for user
    pub async fn get_available_adventures(
        pool: &PgPool,
        user_id: Uuid,
    ) -> AppResult<Vec<AvailableAdventure>> {
        let adventures = sqlx::query_as::<_, AvailableAdventure>(
            r#"
            SELECT id, user_id, difficulty, min_duration_seconds, max_duration_seconds,
                   potential_reward_type, potential_item_rarity, expires_at, is_taken, created_at
            FROM available_adventures
            WHERE user_id = $1 AND is_taken = FALSE AND expires_at > NOW()
            ORDER BY expires_at
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(adventures)
    }

    /// Get available adventure by ID
    pub async fn get_available_adventure(
        pool: &PgPool,
        id: Uuid,
    ) -> AppResult<Option<AvailableAdventure>> {
        let adventure = sqlx::query_as::<_, AvailableAdventure>(
            r#"
            SELECT id, user_id, difficulty, min_duration_seconds, max_duration_seconds,
                   potential_reward_type, potential_item_rarity, expires_at, is_taken, created_at
            FROM available_adventures
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(adventure)
    }

    /// Create available adventure
    pub async fn create_available_adventure(
        pool: &PgPool,
        user_id: Uuid,
        difficulty: AdventureDifficulty,
        min_duration: i32,
        max_duration: i32,
        reward_type: Option<&str>,
        item_rarity: Option<ItemRarity>,
        expires_at: DateTime<Utc>,
    ) -> AppResult<AvailableAdventure> {
        let adventure = sqlx::query_as::<_, AvailableAdventure>(
            r#"
            INSERT INTO available_adventures (
                user_id, difficulty, min_duration_seconds, max_duration_seconds,
                potential_reward_type, potential_item_rarity, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, difficulty, min_duration_seconds, max_duration_seconds,
                      potential_reward_type, potential_item_rarity, expires_at, is_taken, created_at
            "#,
        )
        .bind(user_id)
        .bind(&difficulty)
        .bind(min_duration)
        .bind(max_duration)
        .bind(reward_type)
        .bind(item_rarity.as_ref())
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(adventure)
    }

    /// Mark adventure as taken
    pub async fn mark_adventure_taken(pool: &PgPool, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE available_adventures SET is_taken = TRUE WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Start hero adventure
    pub async fn start_adventure(
        pool: &PgPool,
        hero_id: Uuid,
        difficulty: AdventureDifficulty,
        duration_seconds: i32,
    ) -> AppResult<HeroAdventure> {
        let ends_at = Utc::now() + chrono::Duration::seconds(duration_seconds as i64);

        let adventure = sqlx::query_as::<_, HeroAdventure>(
            r#"
            INSERT INTO hero_adventures (hero_id, difficulty, duration_seconds, ends_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, hero_id, difficulty, started_at, duration_seconds, ends_at,
                      is_completed, completed_at, reward_experience, reward_silver,
                      reward_resources, reward_item_id, health_lost, created_at
            "#,
        )
        .bind(hero_id)
        .bind(&difficulty)
        .bind(duration_seconds)
        .bind(ends_at)
        .fetch_one(pool)
        .await?;

        Ok(adventure)
    }

    /// Get active adventure for hero
    pub async fn get_active_adventure(pool: &PgPool, hero_id: Uuid) -> AppResult<Option<HeroAdventure>> {
        let adventure = sqlx::query_as::<_, HeroAdventure>(
            r#"
            SELECT id, hero_id, difficulty, started_at, duration_seconds, ends_at,
                   is_completed, completed_at, reward_experience, reward_silver,
                   reward_resources, reward_item_id, health_lost, created_at
            FROM hero_adventures
            WHERE hero_id = $1 AND is_completed = FALSE
            ORDER BY ends_at DESC
            LIMIT 1
            "#,
        )
        .bind(hero_id)
        .fetch_optional(pool)
        .await?;

        Ok(adventure)
    }

    /// Complete adventure with rewards
    pub async fn complete_adventure(
        pool: &PgPool,
        adventure_id: Uuid,
        exp: i32,
        silver: i32,
        resources: Option<serde_json::Value>,
        item_id: Option<Uuid>,
        health_lost: i32,
    ) -> AppResult<HeroAdventure> {
        let adventure = sqlx::query_as::<_, HeroAdventure>(
            r#"
            UPDATE hero_adventures
            SET is_completed = TRUE,
                completed_at = NOW(),
                reward_experience = $2,
                reward_silver = $3,
                reward_resources = $4,
                reward_item_id = $5,
                health_lost = $6
            WHERE id = $1
            RETURNING id, hero_id, difficulty, started_at, duration_seconds, ends_at,
                      is_completed, completed_at, reward_experience, reward_silver,
                      reward_resources, reward_item_id, health_lost, created_at
            "#,
        )
        .bind(adventure_id)
        .bind(exp)
        .bind(silver)
        .bind(resources)
        .bind(item_id)
        .bind(health_lost)
        .fetch_one(pool)
        .await?;

        Ok(adventure)
    }

    /// Get completed adventures for hero
    pub async fn get_adventure_history(
        pool: &PgPool,
        hero_id: Uuid,
        limit: i32,
    ) -> AppResult<Vec<HeroAdventure>> {
        let adventures = sqlx::query_as::<_, HeroAdventure>(
            r#"
            SELECT id, hero_id, difficulty, started_at, duration_seconds, ends_at,
                   is_completed, completed_at, reward_experience, reward_silver,
                   reward_resources, reward_item_id, health_lost, created_at
            FROM hero_adventures
            WHERE hero_id = $1 AND is_completed = TRUE
            ORDER BY completed_at DESC
            LIMIT $2
            "#,
        )
        .bind(hero_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(adventures)
    }

    /// Find completed adventures that need processing
    pub async fn find_completed_adventures(pool: &PgPool) -> AppResult<Vec<HeroAdventure>> {
        let adventures = sqlx::query_as::<_, HeroAdventure>(
            r#"
            SELECT id, hero_id, difficulty, started_at, duration_seconds, ends_at,
                   is_completed, completed_at, reward_experience, reward_silver,
                   reward_resources, reward_item_id, health_lost, created_at
            FROM hero_adventures
            WHERE is_completed = FALSE AND ends_at <= NOW()
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(adventures)
    }
}
