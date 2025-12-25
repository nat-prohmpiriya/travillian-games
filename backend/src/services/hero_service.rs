use chrono::{Duration, Utc};
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::hero::{
    AdventureDifficulty, AssignAttributesRequest, AvailableAdventureResponse, CreateHeroRequest,
    EquippedItemsResponse, Hero, HeroAdventureResponse, HeroItemResponse, HeroListResponse,
    HeroResponse, HeroSlotPurchaseResponse, HeroStatus, InventoryResponse, ItemDefinitionResponse,
    ItemRarity, ItemSlot, ReviveInfoResponse, ReviveResourceCost,
};
use crate::repositories::hero_repo::HeroRepository;
use crate::repositories::shop_repo::ShopRepository;
use crate::repositories::village_repo::VillageRepository;

pub struct HeroService;

impl HeroService {
    // ==================== Hero CRUD ====================

    /// Get all heroes for a user
    pub async fn get_user_heroes(pool: &PgPool, user_id: Uuid) -> AppResult<HeroListResponse> {
        let heroes = HeroRepository::get_user_heroes(pool, user_id).await?;
        let total_slots = HeroRepository::get_user_slots(pool, user_id).await?;
        let used_slots = heroes.len() as i32;

        // Get next slot cost
        let next_slot = used_slots + 1;
        let next_slot_cost = if next_slot <= 5 {
            HeroRepository::get_slot_price(pool, next_slot)
                .await?
                .map(|p| p.gold_cost)
        } else {
            None
        };

        Ok(HeroListResponse {
            heroes: heroes.into_iter().map(|h| h.into()).collect(),
            total_slots,
            used_slots,
            next_slot_cost,
        })
    }

    /// Get hero by ID
    pub async fn get_hero(pool: &PgPool, user_id: Uuid, hero_id: Uuid) -> AppResult<HeroResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        Ok(hero.into())
    }

    /// Create a new hero
    pub async fn create_hero(
        pool: &PgPool,
        user_id: Uuid,
        request: CreateHeroRequest,
    ) -> AppResult<HeroResponse> {
        // Check available slots
        let total_slots = HeroRepository::get_user_slots(pool, user_id).await?;
        let used_slots = HeroRepository::count_user_heroes(pool, user_id).await?;

        if used_slots >= total_slots {
            return Err(AppError::BadRequest("No hero slots available".into()));
        }

        // Verify village ownership
        let village = VillageRepository::find_by_id(pool, request.home_village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Village does not belong to you".into()));
        }

        // Find next available slot
        let slot_number = used_slots + 1;

        // Create hero
        let hero = HeroRepository::create(
            pool,
            user_id,
            slot_number,
            &request.name,
            request.tribe,
            request.home_village_id,
        )
        .await?;

        // Generate initial adventures
        Self::generate_adventures(pool, user_id).await?;

        Ok(hero.into())
    }

    /// Change hero's home village
    pub async fn change_home_village(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        village_id: Uuid,
    ) -> AppResult<HeroResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Verify village ownership
        let village = VillageRepository::find_by_id(pool, village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Village not found".into()))?;

        if village.user_id != user_id {
            return Err(AppError::Forbidden("Village does not belong to you".into()));
        }

        let hero = HeroRepository::update_home_village(pool, hero_id, village_id).await?;
        Ok(hero.into())
    }

    /// Assign attribute points
    pub async fn assign_attributes(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        request: AssignAttributesRequest,
    ) -> AppResult<HeroResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Calculate total points being spent
        let total_points =
            request.fighting_strength + request.off_bonus + request.def_bonus + request.resources_bonus;

        if total_points <= 0 {
            return Err(AppError::BadRequest("Must assign at least 1 point".into()));
        }

        if total_points > hero.unassigned_points {
            return Err(AppError::BadRequest("Not enough unassigned points".into()));
        }

        // Validate no negative values
        if request.fighting_strength < 0
            || request.off_bonus < 0
            || request.def_bonus < 0
            || request.resources_bonus < 0
        {
            return Err(AppError::BadRequest("Cannot assign negative points".into()));
        }

        let hero = HeroRepository::assign_attributes(
            pool,
            hero_id,
            request.fighting_strength,
            request.off_bonus,
            request.def_bonus,
            request.resources_bonus,
            total_points,
        )
        .await?;

        Ok(hero.into())
    }

    // ==================== Hero Slots ====================

    /// Buy additional hero slot with gold
    pub async fn buy_hero_slot(pool: &PgPool, user_id: Uuid) -> AppResult<HeroSlotPurchaseResponse> {
        let current_slots = HeroRepository::get_user_slots(pool, user_id).await?;
        let next_slot = current_slots + 1;

        if next_slot > 5 {
            return Err(AppError::BadRequest("Maximum hero slots reached".into()));
        }

        // Get price
        let price = HeroRepository::get_slot_price(pool, next_slot)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid slot".into()))?;

        // Check gold balance
        let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
        if balance < price.gold_cost {
            return Err(AppError::BadRequest("Insufficient gold".into()));
        }

        // Deduct gold
        let new_balance = ShopRepository::deduct_gold(pool, user_id, price.gold_cost).await?;

        // Add slot
        let total_slots = HeroRepository::add_user_slot(pool, user_id).await?;

        // Record transaction
        ShopRepository::create_transaction(
            pool,
            user_id,
            crate::models::shop::TransactionType::GoldSpend,
            -price.gold_cost,
            None,
            None,
            None,
            None,
            Some(&format!("Hero Slot #{}", next_slot)),
        )
        .await?;

        // Record gold usage
        ShopRepository::record_gold_usage(
            pool,
            user_id,
            crate::models::shop::GoldFeature::HeroSlot,
            price.gold_cost,
            None,
            None,
            Some(serde_json::json!({ "slot_number": next_slot })),
            None,
        )
        .await?;

        Ok(HeroSlotPurchaseResponse {
            success: true,
            new_slot_number: next_slot,
            gold_spent: price.gold_cost,
            new_balance,
            total_slots,
        })
    }

    // ==================== Inventory ====================

    /// Get hero's inventory
    pub async fn get_inventory(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
    ) -> AppResult<InventoryResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        let items = HeroRepository::get_hero_items(pool, hero_id).await?;

        // Separate equipped and unequipped items
        let mut equipped = EquippedItemsResponse {
            helmet: None,
            weapon: None,
            armor_left: None,
            armor_right: None,
            boots: None,
            horse: None,
            bag: None,
            bandage: None,
        };

        let mut inventory_items: Vec<HeroItemResponse> = Vec::new();
        let mut extra_slots = 0;

        for (hero_item, item_def) in items {
            let item_response = HeroItemResponse {
                id: hero_item.id,
                item: item_def.clone().into(),
                is_equipped: hero_item.is_equipped,
                equipped_slot: hero_item.equipped_slot,
                quantity: hero_item.quantity,
                obtained_at: hero_item.obtained_at,
            };

            if hero_item.is_equipped {
                if let Some(slot) = &hero_item.equipped_slot {
                    match slot {
                        ItemSlot::Helmet => equipped.helmet = Some(item_response.clone()),
                        ItemSlot::Weapon => equipped.weapon = Some(item_response.clone()),
                        ItemSlot::ArmorLeft => equipped.armor_left = Some(item_response.clone()),
                        ItemSlot::ArmorRight => equipped.armor_right = Some(item_response.clone()),
                        ItemSlot::Boots => equipped.boots = Some(item_response.clone()),
                        ItemSlot::Horse => equipped.horse = Some(item_response.clone()),
                        ItemSlot::Bag => {
                            extra_slots += item_def.extra_inventory_slots;
                            equipped.bag = Some(item_response.clone());
                        }
                        ItemSlot::Bandage => equipped.bandage = Some(item_response.clone()),
                        ItemSlot::Consumable => {}
                    }
                }
            } else {
                inventory_items.push(item_response);
            }
        }

        let base_slots = 10;
        let total_slots = base_slots + extra_slots;
        let used_slots = inventory_items.len() as i32;

        Ok(InventoryResponse {
            equipped,
            items: inventory_items,
            total_slots,
            used_slots,
        })
    }

    /// Equip item
    pub async fn equip_item(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        item_id: Uuid,
    ) -> AppResult<HeroItemResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Get item
        let (hero_item, item_def) = HeroRepository::get_hero_item(pool, item_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;

        if hero_item.hero_id != hero_id {
            return Err(AppError::Forbidden("Item does not belong to this hero".into()));
        }

        if hero_item.is_equipped {
            return Err(AppError::BadRequest("Item is already equipped".into()));
        }

        // Check level requirement
        if hero.level < item_def.required_level {
            return Err(AppError::BadRequest(format!(
                "Requires level {}",
                item_def.required_level
            )));
        }

        // Consumables cannot be equipped
        if item_def.is_consumable {
            return Err(AppError::BadRequest("Consumables cannot be equipped".into()));
        }

        // Unequip existing item in same slot
        HeroRepository::unequip_slot(pool, hero_id, item_def.slot).await?;

        // Equip new item
        let equipped = HeroRepository::equip_item(pool, item_id, item_def.slot).await?;

        Ok(HeroItemResponse {
            id: equipped.id,
            item: item_def.into(),
            is_equipped: true,
            equipped_slot: equipped.equipped_slot,
            quantity: equipped.quantity,
            obtained_at: equipped.obtained_at,
        })
    }

    /// Unequip item from slot
    pub async fn unequip_slot(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        slot: ItemSlot,
    ) -> AppResult<()> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        HeroRepository::unequip_slot(pool, hero_id, slot).await?;
        Ok(())
    }

    /// Use consumable item
    pub async fn use_item(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        item_id: Uuid,
    ) -> AppResult<HeroResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Get item
        let (hero_item, item_def) = HeroRepository::get_hero_item(pool, item_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;

        if hero_item.hero_id != hero_id {
            return Err(AppError::Forbidden("Item does not belong to this hero".into()));
        }

        if !item_def.is_consumable {
            return Err(AppError::BadRequest("Item is not consumable".into()));
        }

        // Apply item effect
        let mut updated_hero = hero.clone();
        if item_def.health_restore > 0 {
            let new_health = (hero.health + item_def.health_restore).min(100);
            updated_hero = HeroRepository::update_health(pool, hero_id, new_health).await?;
        }

        // Use (consume) the item
        HeroRepository::use_item(pool, item_id).await?;

        Ok(updated_hero.into())
    }

    /// Sell/delete item
    pub async fn sell_item(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        item_id: Uuid,
    ) -> AppResult<i32> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        // Get item
        let (hero_item, item_def) = HeroRepository::get_hero_item(pool, item_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Item not found".into()))?;

        if hero_item.hero_id != hero_id {
            return Err(AppError::Forbidden("Item does not belong to this hero".into()));
        }

        if hero_item.is_equipped {
            return Err(AppError::BadRequest("Cannot sell equipped items".into()));
        }

        let sell_value = item_def.sell_value * hero_item.quantity;

        // Delete item
        HeroRepository::delete_item(pool, item_id).await?;

        // TODO: Add silver to user's balance

        Ok(sell_value)
    }

    // ==================== Adventures ====================

    /// Get available adventures
    pub async fn get_available_adventures(
        pool: &PgPool,
        user_id: Uuid,
    ) -> AppResult<Vec<AvailableAdventureResponse>> {
        let adventures = HeroRepository::get_available_adventures(pool, user_id).await?;
        Ok(adventures.into_iter().map(|a| a.into()).collect())
    }

    /// Generate new adventures for user
    pub async fn generate_adventures(pool: &PgPool, user_id: Uuid) -> AppResult<()> {
        let now = Utc::now();

        // Pre-generate all random values (scope RNG so it's dropped before await)
        struct AdventureParams {
            difficulty: AdventureDifficulty,
            min_duration: i32,
            max_duration: i32,
            reward_type: Option<&'static str>,
            item_rarity: Option<ItemRarity>,
            expires_at: chrono::DateTime<Utc>,
        }

        let adventures: Vec<AdventureParams> = {
            let mut rng = rand::thread_rng();
            let mut params = Vec::new();

            // Generate 3-5 short adventures
            let short_count = rng.gen_range(3..=5);
            for _ in 0..short_count {
                let min_duration = rng.gen_range(30..60) * 60;
                let max_duration = min_duration + rng.gen_range(30..90) * 60;
                let expires_at = now + Duration::hours(rng.gen_range(6..24));

                let reward_type = match rng.gen_range(0..3) {
                    0 => Some("resources"),
                    1 => Some("silver"),
                    _ => Some("item"),
                };

                let item_rarity = if reward_type == Some("item") {
                    Some(match rng.gen_range(0..100) {
                        0..=60 => ItemRarity::Common,
                        61..=85 => ItemRarity::Uncommon,
                        86..=95 => ItemRarity::Rare,
                        _ => ItemRarity::Epic,
                    })
                } else {
                    None
                };

                params.push(AdventureParams {
                    difficulty: AdventureDifficulty::Short,
                    min_duration,
                    max_duration,
                    reward_type,
                    item_rarity,
                    expires_at,
                });
            }

            // Generate 1-2 long adventures
            let long_count = rng.gen_range(1..=2);
            for _ in 0..long_count {
                let min_duration = rng.gen_range(8..12) * 3600;
                let max_duration = min_duration + rng.gen_range(2..4) * 3600;
                let expires_at = now + Duration::hours(rng.gen_range(12..48));

                let item_rarity = Some(match rng.gen_range(0..100) {
                    0..=30 => ItemRarity::Uncommon,
                    31..=60 => ItemRarity::Rare,
                    61..=85 => ItemRarity::Epic,
                    _ => ItemRarity::Legendary,
                });

                params.push(AdventureParams {
                    difficulty: AdventureDifficulty::Long,
                    min_duration,
                    max_duration,
                    reward_type: Some("item"),
                    item_rarity,
                    expires_at,
                });
            }

            params
        };

        // Now create adventures (RNG is dropped, safe to await)
        for adv in adventures {
            HeroRepository::create_available_adventure(
                pool,
                user_id,
                adv.difficulty,
                adv.min_duration,
                adv.max_duration,
                adv.reward_type,
                adv.item_rarity,
                adv.expires_at,
            )
            .await?;
        }

        Ok(())
    }

    /// Start adventure
    pub async fn start_adventure(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        adventure_id: Uuid,
    ) -> AppResult<HeroAdventureResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        if !hero.is_available() {
            return Err(AppError::BadRequest("Hero is not available".into()));
        }

        // Check if hero already has active adventure
        if let Some(_) = HeroRepository::get_active_adventure(pool, hero_id).await? {
            return Err(AppError::BadRequest("Hero already has an active adventure".into()));
        }

        // Get available adventure
        let adventure = HeroRepository::get_available_adventure(pool, adventure_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Adventure not found".into()))?;

        if adventure.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        if adventure.is_taken {
            return Err(AppError::BadRequest("Adventure already taken".into()));
        }

        if adventure.expires_at < Utc::now() {
            return Err(AppError::BadRequest("Adventure has expired".into()));
        }

        // Calculate random duration within range (scope RNG so it's dropped before await)
        let duration = {
            let mut rng = rand::thread_rng();
            rng.gen_range(adventure.min_duration_seconds..=adventure.max_duration_seconds)
        };

        // Mark available adventure as taken
        HeroRepository::mark_adventure_taken(pool, adventure_id).await?;

        // Start adventure
        let hero_adventure = HeroRepository::start_adventure(
            pool,
            hero_id,
            adventure.difficulty,
            duration,
        )
        .await?;

        // Update hero status
        HeroRepository::update_status(pool, hero_id, HeroStatus::InAdventure).await?;

        Ok(HeroAdventureResponse {
            id: hero_adventure.id,
            hero_id: hero_adventure.hero_id,
            difficulty: hero_adventure.difficulty,
            started_at: hero_adventure.started_at,
            ends_at: hero_adventure.ends_at,
            is_completed: false,
            reward_experience: None,
            reward_silver: None,
            reward_resources: None,
            reward_item: None,
            health_lost: None,
        })
    }

    /// Get active adventure for hero
    pub async fn get_active_adventure(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
    ) -> AppResult<Option<HeroAdventureResponse>> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        let adventure = HeroRepository::get_active_adventure(pool, hero_id).await?;
        Ok(adventure.map(|a| HeroAdventureResponse {
            id: a.id,
            hero_id: a.hero_id,
            difficulty: a.difficulty,
            started_at: a.started_at,
            ends_at: a.ends_at,
            is_completed: a.is_completed,
            reward_experience: a.reward_experience,
            reward_silver: a.reward_silver,
            reward_resources: a.reward_resources,
            reward_item: None, // Would need to fetch item definition
            health_lost: a.health_lost,
        }))
    }

    /// Process completed adventures (called by background job)
    pub async fn process_completed_adventures(pool: &PgPool) -> AppResult<i32> {
        let completed = HeroRepository::find_completed_adventures(pool).await?;
        let mut count = 0;

        for adventure in completed {
            if let Err(e) = Self::complete_adventure(pool, adventure.id).await {
                tracing::error!("Failed to complete adventure {}: {}", adventure.id, e);
            } else {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Complete a single adventure
    async fn complete_adventure(pool: &PgPool, adventure_id: Uuid) -> AppResult<()> {
        let adventure = HeroRepository::get_active_adventure(pool, adventure_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Adventure not found".into()))?;

        // Pre-generate all random values (scope RNG so it's dropped before await)
        struct RewardParams {
            base_exp: i32,
            base_silver: i32,
            health_damage: i32,
            resources: serde_json::Value,
            should_drop_item: bool,
            item_rarity: Option<ItemRarity>,
            item_index_seed: usize,
        }

        let params: RewardParams = {
            let mut rng = rand::thread_rng();

            let (base_exp, base_silver, health_damage) = match adventure.difficulty {
                AdventureDifficulty::Short => (
                    rng.gen_range(50..150),
                    rng.gen_range(10..50),
                    rng.gen_range(5..20),
                ),
                AdventureDifficulty::Long => (
                    rng.gen_range(200..500),
                    rng.gen_range(50..200),
                    rng.gen_range(15..40),
                ),
            };

            let resources = serde_json::json!({
                "wood": rng.gen_range(50..200),
                "clay": rng.gen_range(50..200),
                "iron": rng.gen_range(50..200),
                "crop": rng.gen_range(50..200),
            });

            let drop_chance = match adventure.difficulty {
                AdventureDifficulty::Short => 30,
                AdventureDifficulty::Long => 60,
            };

            let should_drop_item = rng.gen_range(0..100) < drop_chance;

            let item_rarity = if should_drop_item {
                Some(match adventure.difficulty {
                    AdventureDifficulty::Short => match rng.gen_range(0..100) {
                        0..=60 => ItemRarity::Common,
                        61..=85 => ItemRarity::Uncommon,
                        86..=95 => ItemRarity::Rare,
                        _ => ItemRarity::Epic,
                    },
                    AdventureDifficulty::Long => match rng.gen_range(0..100) {
                        0..=20 => ItemRarity::Uncommon,
                        21..=50 => ItemRarity::Rare,
                        51..=80 => ItemRarity::Epic,
                        _ => ItemRarity::Legendary,
                    },
                })
            } else {
                None
            };

            RewardParams {
                base_exp,
                base_silver,
                health_damage,
                resources,
                should_drop_item,
                item_rarity,
                item_index_seed: rng.gen_range(0..1000),
            }
        };

        // Now do async operations (RNG is dropped)
        let item_id = if params.should_drop_item {
            if let Some(rarity) = params.item_rarity {
                let items = HeroRepository::get_items_by_rarity(pool, rarity).await?;
                if !items.is_empty() {
                    let item = &items[params.item_index_seed % items.len()];
                    HeroRepository::add_item(pool, adventure.hero_id, item.id, 1).await?;
                    Some(item.id)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Complete adventure
        HeroRepository::complete_adventure(
            pool,
            adventure_id,
            params.base_exp,
            params.base_silver,
            Some(params.resources),
            item_id,
            params.health_damage,
        )
        .await?;

        // Add experience to hero
        HeroRepository::add_experience(pool, adventure.hero_id, params.base_exp).await?;

        // Damage hero
        HeroRepository::damage_hero(pool, adventure.hero_id, params.health_damage).await?;

        // Update hero status back to idle (if not dead)
        let hero = HeroRepository::find_by_id(pool, adventure.hero_id).await?;
        if let Some(hero) = hero {
            if hero.health > 0 {
                HeroRepository::update_status(pool, adventure.hero_id, HeroStatus::Idle).await?;
            }
        }

        Ok(())
    }

    // ==================== Revive ====================

    /// Get revive info for dead hero
    pub async fn get_revive_info(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
    ) -> AppResult<ReviveInfoResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        if !hero.is_dead() {
            return Err(AppError::BadRequest("Hero is not dead".into()));
        }

        let revive_at = hero.revive_at.unwrap_or(Utc::now());
        let remaining = (revive_at - Utc::now()).num_seconds().max(0);

        // Calculate gold cost for instant revive (1 gold per 30 minutes remaining)
        let gold_cost = ((remaining as f64 / 1800.0).ceil() as i32).max(1);

        // Resource cost for natural revive
        let base_cost = hero.level * 100;
        let resource_cost = ReviveResourceCost {
            wood: base_cost,
            clay: base_cost,
            iron: base_cost,
            crop: base_cost * 2,
        };

        Ok(ReviveInfoResponse {
            hero_id,
            revive_at,
            remaining_seconds: remaining,
            gold_cost_instant: gold_cost,
            resource_cost,
        })
    }

    /// Revive hero (with gold or naturally)
    pub async fn revive_hero(
        pool: &PgPool,
        user_id: Uuid,
        hero_id: Uuid,
        use_gold: bool,
    ) -> AppResult<HeroResponse> {
        let hero = HeroRepository::find_by_id(pool, hero_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Hero not found".into()))?;

        if hero.user_id != user_id {
            return Err(AppError::Forbidden("Access denied".into()));
        }

        if !hero.is_dead() {
            return Err(AppError::BadRequest("Hero is not dead".into()));
        }

        if use_gold {
            let revive_info = Self::get_revive_info(pool, user_id, hero_id).await?;

            // Check gold balance
            let balance = ShopRepository::get_gold_balance(pool, user_id).await?;
            if balance < revive_info.gold_cost_instant {
                return Err(AppError::BadRequest("Insufficient gold".into()));
            }

            // Deduct gold
            ShopRepository::deduct_gold(pool, user_id, revive_info.gold_cost_instant).await?;

            // Revive with 50% health
            let hero = HeroRepository::revive_hero(pool, hero_id, 50).await?;
            Ok(hero.into())
        } else {
            // Natural revive - check if time has passed
            let revive_at = hero.revive_at.unwrap_or(Utc::now());
            if Utc::now() < revive_at {
                return Err(AppError::BadRequest("Hero cannot be revived yet".into()));
            }

            // Revive with 25% health
            let hero = HeroRepository::revive_hero(pool, hero_id, 25).await?;
            Ok(hero.into())
        }
    }

    // ==================== Health Regeneration ====================

    /// Process health regeneration for all heroes (called by background job)
    pub async fn process_health_regen(pool: &PgPool) -> AppResult<i32> {
        // Get all heroes that need health regen (health < 100 and not dead)
        let result = sqlx::query(
            r#"
            UPDATE heroes
            SET health = LEAST(100, health + (
                EXTRACT(EPOCH FROM (NOW() - last_health_update)) / 3600.0 * health_regen_rate
            )::INTEGER),
            last_health_update = NOW(),
            updated_at = NOW()
            WHERE health < 100 AND health > 0 AND status != 'dead'
            "#,
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() as i32)
    }
}
