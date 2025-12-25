use chrono::{Duration, Utc};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::army::{
    Army, ArmyResponse, ArmyTroops, BattleReport, CarriedResources, MissionType, ScoutReport,
    SendArmyRequest,
};
use crate::models::troop::TroopDefinition;
use crate::models::village::Village;
use crate::repositories::army_repo::ArmyRepository;
use crate::repositories::troop_repo::TroopRepository;
use crate::repositories::village_repo::VillageRepository;

/// Internal struct for battle calculation results
struct BattleResult {
    attacker_wins: bool,
    attacker_survivors: ArmyTroops,
    defender_survivors: ArmyTroops,
    attacker_losses: ArmyTroops,
    defender_losses: ArmyTroops,
}

pub struct ArmyService;

impl ArmyService {
    /// Send an army from a village to target coordinates
    pub async fn send_army(
        pool: &PgPool,
        player_id: Uuid,
        from_village_id: Uuid,
        request: SendArmyRequest,
    ) -> AppResult<ArmyResponse> {
        // Validate mission type
        if !matches!(
            request.mission,
            MissionType::Raid | MissionType::Attack | MissionType::Scout | MissionType::Support | MissionType::Conquer
        ) {
            return Err(AppError::BadRequest(
                "Only Raid, Attack, Scout, Support, and Conquer missions are currently supported".into(),
            ));
        }

        // Conquer mission requires at least one Chief troop
        if request.mission == MissionType::Conquer {
            let has_chief = request.troops.iter().any(|(troop_type, count)| {
                *count > 0 && troop_type.is_chief()
            });
            if !has_chief {
                return Err(AppError::BadRequest(
                    "Conquer mission requires at least one Chief unit (Royal Advisor, Harbor Master, or Elder Chief)".into(),
                ));
            }
        }

        // Get source village
        let from_village = VillageRepository::find_by_id(pool, from_village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Source village not found".into()))?;

        // Validate ownership
        if from_village.user_id != player_id {
            return Err(AppError::Forbidden);
        }

        // Validate troops are available
        let village_troops = TroopRepository::find_by_village(pool, from_village_id).await?;
        for (troop_type, count) in &request.troops {
            if *count <= 0 {
                continue;
            }
            let available = village_troops
                .iter()
                .find(|t| t.troop_type == *troop_type)
                .map(|t| t.in_village)
                .unwrap_or(0);
            if available < *count {
                return Err(AppError::BadRequest(format!(
                    "Not enough {:?}: have {}, need {}",
                    troop_type, available, count
                )));
            }
        }

        // Get total troops being sent
        let total_troops: i32 = request.troops.values().sum();
        if total_troops <= 0 {
            return Err(AppError::BadRequest("Must send at least one troop".into()));
        }

        // Get target village (if exists)
        let target_village = VillageRepository::find_by_coordinates(pool, request.to_x, request.to_y).await?;

        // Can't attack own village (but can support own village)
        if let Some(ref target) = target_village {
            if target.user_id == player_id && request.mission.is_hostile() {
                return Err(AppError::BadRequest("Cannot attack your own village".into()));
            }
        }

        // Support mission requires a target village
        if request.mission == MissionType::Support && target_village.is_none() {
            return Err(AppError::BadRequest("Support mission requires a target village".into()));
        }

        // Get troop definitions for travel time calculation
        let definitions = TroopRepository::get_all_definitions(pool).await?;

        // Calculate travel time
        let distance = Self::calculate_distance(
            from_village.x,
            from_village.y,
            request.to_x,
            request.to_y,
        );
        let travel_duration = Self::calculate_travel_time(distance, &request.troops, &definitions);

        // Calculate timestamps
        let now = Utc::now();
        let arrives_at = now + travel_duration;
        let returns_at = if request.mission.returns() {
            Some(arrives_at + travel_duration)
        } else {
            None
        };

        // Remove troops from village
        for (troop_type, count) in &request.troops {
            if *count > 0 {
                TroopRepository::remove_troops_from_village(pool, from_village_id, *troop_type, *count)
                    .await?;
            }
        }

        // Create army record
        let army = ArmyRepository::create(
            pool,
            player_id,
            from_village_id,
            request.to_x,
            request.to_y,
            target_village.as_ref().map(|v| v.id),
            request.mission,
            &request.troops,
            &request.resources,
            now,
            arrives_at,
            returns_at,
        )
        .await?;

        info!(
            "Army sent from village {} to ({}, {}) with {} troops, arrives at {}",
            from_village_id, request.to_x, request.to_y, total_troops, arrives_at
        );

        Ok(army.into())
    }

    /// Process all armies that have arrived at their destination
    pub async fn process_arrived_armies(pool: &PgPool) -> AppResult<i32> {
        let arrived = ArmyRepository::find_arrived(pool).await?;
        let mut processed = 0;

        for army in arrived {
            let result = if army.is_returning {
                Self::handle_returning_army(pool, &army).await
            } else {
                match army.mission {
                    MissionType::Raid | MissionType::Attack => {
                        Self::handle_hostile_arrival(pool, &army).await
                    }
                    MissionType::Scout => {
                        Self::handle_scout_arrival(pool, &army).await
                    }
                    MissionType::Support => {
                        Self::handle_support_arrival(pool, &army).await
                    }
                    MissionType::Conquer => {
                        Self::handle_conquer_arrival(pool, &army).await
                    }
                    _ => {
                        // Other mission types not implemented yet
                        error!("Unhandled mission type: {:?}", army.mission);
                        continue;
                    }
                }
            };

            match result {
                Ok(_) => processed += 1,
                Err(e) => {
                    error!("Failed to process army {}: {:?}", army.id, e);
                }
            }
        }

        Ok(processed)
    }

    /// Handle raid/attack arrival at target
    async fn handle_hostile_arrival(pool: &PgPool, army: &Army) -> AppResult<()> {
        let definitions = TroopRepository::get_all_definitions(pool).await?;

        // Get target village
        let target_village = if let Some(village_id) = army.to_village_id {
            VillageRepository::find_by_id(pool, village_id).await?
        } else {
            // Check if village was built at these coordinates since army was sent
            VillageRepository::find_by_coordinates(pool, army.to_x, army.to_y).await?
        };

        // If no target village, army just returns
        let Some(target) = target_village else {
            info!("Army {} arrived at empty tile, returning home", army.id);
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        };

        // Get defender troops (village's own troops)
        let defender_troops_list = TroopRepository::find_by_village(pool, target.id).await?;
        let village_troops: ArmyTroops = defender_troops_list
            .iter()
            .filter(|t| t.in_village > 0)
            .map(|t| (t.troop_type, t.in_village))
            .collect();

        // Get stationed support troops at this village
        let stationed_armies = ArmyRepository::find_stationed_at_village(pool, target.id).await?;

        // Combine village troops with stationed support troops for total defense
        let mut total_defender_troops = village_troops.clone();
        for stationed in &stationed_armies {
            for (troop_type, count) in stationed.troops.0.iter() {
                *total_defender_troops.entry(*troop_type).or_insert(0) += count;
            }
        }

        // Calculate battle with combined defense
        let battle = Self::calculate_battle(
            &army.troops.0,
            &total_defender_troops,
            &definitions,
            army.mission,
        );

        // Apply losses to village's own troops
        let village_loss_ratio = if total_defender_troops.values().sum::<i32>() > 0 {
            village_troops.values().sum::<i32>() as f64
                / total_defender_troops.values().sum::<i32>() as f64
        } else {
            1.0
        };

        for (troop_type, total_losses) in &battle.defender_losses {
            // Calculate village's share of losses
            let village_count = village_troops.get(troop_type).copied().unwrap_or(0);
            let total_count = total_defender_troops.get(troop_type).copied().unwrap_or(0);

            if total_count > 0 && village_count > 0 {
                let village_losses = ((*total_losses as f64) * (village_count as f64 / total_count as f64)).ceil() as i32;
                let actual_losses = village_losses.min(village_count);
                if actual_losses > 0 {
                    TroopRepository::kill_troops(pool, target.id, *troop_type, actual_losses)
                        .await?;
                }
            }
        }

        // Apply losses to stationed support troops
        for stationed in &stationed_armies {
            let mut stationed_survivors = stationed.troops.0.clone();
            let mut had_losses = false;

            for (troop_type, total_losses) in &battle.defender_losses {
                let stationed_count = stationed.troops.0.get(troop_type).copied().unwrap_or(0);
                let total_count = total_defender_troops.get(troop_type).copied().unwrap_or(0);

                if total_count > 0 && stationed_count > 0 {
                    let stationed_losses = ((*total_losses as f64) * (stationed_count as f64 / total_count as f64)).ceil() as i32;
                    let actual_losses = stationed_losses.min(stationed_count);
                    if actual_losses > 0 {
                        had_losses = true;
                        let remaining = stationed_count - actual_losses;
                        if remaining > 0 {
                            stationed_survivors.insert(*troop_type, remaining);
                        } else {
                            stationed_survivors.remove(troop_type);
                        }
                    }
                }
            }

            // Update stationed army with survivors (or delete if all dead)
            if had_losses {
                ArmyRepository::update_stationed_troops(pool, stationed.id, &stationed_survivors)
                    .await?;
            }
        }

        // Calculate stolen resources if attacker won
        let stolen_resources = if battle.attacker_wins {
            Self::calculate_stolen_resources(&target, &battle.attacker_survivors, &definitions, army.mission)
        } else {
            CarriedResources::default()
        };

        // Deduct stolen resources from target village
        if stolen_resources.total() > 0 {
            VillageRepository::deduct_resources(
                pool,
                target.id,
                stolen_resources.wood,
                stolen_resources.clay,
                stolen_resources.iron,
                stolen_resources.crop,
            )
            .await?;
        }

        // Create battle report (show total defender troops including support)
        let winner = if battle.attacker_wins {
            "attacker"
        } else if battle.defender_survivors.values().sum::<i32>() > 0 {
            "defender"
        } else {
            "draw"
        };

        let report = ArmyRepository::create_battle_report(
            pool,
            army.player_id,
            Some(target.user_id),
            army.from_village_id,
            Some(target.id),
            army.mission,
            &army.troops.0,
            &total_defender_troops,
            &battle.attacker_losses,
            &battle.defender_losses,
            &stolen_resources,
            winner,
            Utc::now(),
        )
        .await?;

        info!(
            "Battle at ({}, {}): {} wins! Attacker lost {:?}, Defender lost {:?} (including {} support armies)",
            army.to_x, army.to_y, winner,
            battle.attacker_losses.values().sum::<i32>(),
            battle.defender_losses.values().sum::<i32>(),
            stationed_armies.len()
        );

        // Initiate return journey if there are survivors
        let total_survivors: i32 = battle.attacker_survivors.values().sum();
        if total_survivors > 0 && army.mission.returns() {
            Self::initiate_return(
                pool,
                army,
                battle.attacker_survivors,
                stolen_resources,
                Some(report.id),
            )
            .await?;
        } else {
            // All troops dead or non-returning mission
            ArmyRepository::delete(pool, army.id).await?;
        }

        Ok(())
    }

    /// Handle scout mission arrival at target
    async fn handle_scout_arrival(pool: &PgPool, army: &Army) -> AppResult<()> {
        let definitions = TroopRepository::get_all_definitions(pool).await?;

        // Get target village
        let target_village = if let Some(village_id) = army.to_village_id {
            VillageRepository::find_by_id(pool, village_id).await?
        } else {
            VillageRepository::find_by_coordinates(pool, army.to_x, army.to_y).await?
        };

        // If no target village, scouts just return with no info
        let Some(target) = target_village else {
            info!("Scout {} arrived at empty tile, returning home", army.id);
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        };

        // Get defender troops
        let defender_troops_list = TroopRepository::find_by_village(pool, target.id).await?;
        let defender_troops: ArmyTroops = defender_troops_list
            .iter()
            .filter(|t| t.in_village > 0)
            .map(|t| (t.troop_type, t.in_village))
            .collect();

        // Calculate scout power (using speed as scout effectiveness)
        let attacker_scout_power = Self::calculate_scout_power(&army.troops.0, &definitions);
        let defender_scout_power = Self::calculate_scout_power(&defender_troops, &definitions);

        // Count scouts sent
        let attacker_scout_count: i32 = army.troops.0.values().sum();
        let defender_scout_count: i32 = defender_troops.values().sum();

        // Scout combat: attacker needs > defender's power to succeed
        // Ratio determines success and losses
        let total_power = attacker_scout_power + defender_scout_power;
        let attacker_ratio = if total_power > 0.0 {
            attacker_scout_power / total_power
        } else {
            1.0
        };

        let success = attacker_ratio > 0.4; // Attacker needs at least 40% power ratio to succeed

        // Calculate losses
        let (attacker_losses, defender_losses) = if defender_scout_power > 0.0 {
            // Scout combat - both sides lose scouts
            let attacker_loss_ratio = if success {
                (1.0 - attacker_ratio) * 0.8 // Winner loses less
            } else {
                0.9 + (1.0 - attacker_ratio) * 0.1 // Loser loses 90-100%
            };
            let defender_loss_ratio = if success {
                attacker_ratio * 0.5 // Defender loses based on attacker power
            } else {
                0.1 // Defender barely loses if they win
            };

            let attacker_lost = (attacker_scout_count as f64 * attacker_loss_ratio).ceil() as i32;
            let defender_lost = (defender_scout_count as f64 * defender_loss_ratio).ceil() as i32;

            (attacker_lost.min(attacker_scout_count), defender_lost.min(defender_scout_count))
        } else {
            // No defender scouts - perfect scouting, no losses
            (0, 0)
        };

        // Kill defender scouts
        if defender_losses > 0 {
            // Distribute losses proportionally across troop types
            for (troop_type, count) in &defender_troops {
                let ratio = *count as f64 / defender_scout_count as f64;
                let losses = (defender_losses as f64 * ratio).ceil() as i32;
                if losses > 0 {
                    TroopRepository::kill_troops(pool, target.id, *troop_type, losses.min(*count))
                        .await?;
                }
            }
        }

        // Prepare scouted info (only if successful)
        let (scouted_resources, scouted_troops) = if success {
            let resources = CarriedResources {
                wood: target.wood,
                clay: target.clay,
                iron: target.iron,
                crop: target.crop,
            };
            (Some(resources), Some(defender_troops.clone()))
        } else {
            (None, None)
        };

        // Create scout report
        let _report = ArmyRepository::create_scout_report(
            pool,
            army.player_id,
            Some(target.user_id),
            army.from_village_id,
            Some(target.id),
            attacker_scout_count,
            defender_scout_count,
            attacker_losses,
            defender_losses,
            success,
            scouted_resources.as_ref(),
            scouted_troops.as_ref(),
            Utc::now(),
        )
        .await?;

        info!(
            "Scout at ({}, {}): {}! Attacker lost {}/{}, Defender lost {}/{}",
            army.to_x, army.to_y,
            if success { "SUCCESS" } else { "FAILED" },
            attacker_losses, attacker_scout_count,
            defender_losses, defender_scout_count
        );

        // Calculate survivors and initiate return
        let survivors = Self::calculate_scout_survivors(&army.troops.0, attacker_losses, attacker_scout_count);
        let total_survivors: i32 = survivors.values().sum();

        if total_survivors > 0 {
            Self::initiate_return(
                pool,
                army,
                survivors,
                CarriedResources::default(),
                None,
            )
            .await?;
        } else {
            // All scouts dead
            ArmyRepository::delete(pool, army.id).await?;
        }

        Ok(())
    }

    /// Handle support mission arrival at target village
    async fn handle_support_arrival(pool: &PgPool, army: &Army) -> AppResult<()> {
        // Get target village
        let target_village = if let Some(village_id) = army.to_village_id {
            VillageRepository::find_by_id(pool, village_id).await?
        } else {
            VillageRepository::find_by_coordinates(pool, army.to_x, army.to_y).await?
        };

        // If no target village exists, troops return home
        let Some(_target) = target_village else {
            info!(
                "Support army {} arrived at empty tile ({}, {}), returning home",
                army.id, army.to_x, army.to_y
            );
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        };

        // Mark army as stationed at target village
        ArmyRepository::set_stationed(pool, army.id).await?;

        info!(
            "Support army {} is now stationed at ({}, {}) with {} troops",
            army.id,
            army.to_x,
            army.to_y,
            army.troops.0.values().sum::<i32>()
        );

        Ok(())
    }

    /// Handle conquer mission arrival at target village
    /// Similar to attack, but also reduces loyalty if attacker wins with surviving Chiefs
    async fn handle_conquer_arrival(pool: &PgPool, army: &Army) -> AppResult<()> {
        let definitions = TroopRepository::get_all_definitions(pool).await?;

        // Get target village
        let target_village = if let Some(village_id) = army.to_village_id {
            VillageRepository::find_by_id(pool, village_id).await?
        } else {
            VillageRepository::find_by_coordinates(pool, army.to_x, army.to_y).await?
        };

        // If no target village, army just returns
        let Some(target) = target_village else {
            info!("Conquer army {} arrived at empty tile, returning home", army.id);
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        };

        // Can't conquer own village
        if target.user_id == army.player_id {
            info!("Conquer army {} cannot conquer own village, returning home", army.id);
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        }

        // Can't conquer capital
        if target.is_capital {
            info!("Conquer army {} cannot conquer capital, returning home", army.id);
            return Self::initiate_return(
                pool,
                army,
                army.troops.0.clone(),
                CarriedResources::default(),
                None,
            )
            .await;
        }

        // Get defender troops (village's own troops + stationed support)
        let defender_troops_list = TroopRepository::find_by_village(pool, target.id).await?;
        let village_troops: ArmyTroops = defender_troops_list
            .iter()
            .filter(|t| t.in_village > 0)
            .map(|t| (t.troop_type, t.in_village))
            .collect();

        let stationed_armies = ArmyRepository::find_stationed_at_village(pool, target.id).await?;
        let mut total_defender_troops = village_troops.clone();
        for stationed in &stationed_armies {
            for (troop_type, count) in stationed.troops.0.iter() {
                *total_defender_troops.entry(*troop_type).or_insert(0) += count;
            }
        }

        // Calculate battle (similar to Attack mission)
        let battle = Self::calculate_battle(
            &army.troops.0,
            &total_defender_troops,
            &definitions,
            MissionType::Attack, // Use Attack calculation for combat
        );

        // Apply defender losses (same as handle_hostile_arrival)
        for (troop_type, total_losses) in &battle.defender_losses {
            let village_count = village_troops.get(troop_type).copied().unwrap_or(0);
            let total_count = total_defender_troops.get(troop_type).copied().unwrap_or(0);

            if total_count > 0 && village_count > 0 {
                let village_losses = ((*total_losses as f64) * (village_count as f64 / total_count as f64)).ceil() as i32;
                let actual_losses = village_losses.min(village_count);
                if actual_losses > 0 {
                    TroopRepository::kill_troops(pool, target.id, *troop_type, actual_losses)
                        .await?;
                }
            }
        }

        // Apply losses to stationed support troops
        for stationed in &stationed_armies {
            let mut stationed_survivors = stationed.troops.0.clone();
            let mut had_losses = false;

            for (troop_type, total_losses) in &battle.defender_losses {
                let stationed_count = stationed.troops.0.get(troop_type).copied().unwrap_or(0);
                let total_count = total_defender_troops.get(troop_type).copied().unwrap_or(0);

                if total_count > 0 && stationed_count > 0 {
                    let stationed_losses = ((*total_losses as f64) * (stationed_count as f64 / total_count as f64)).ceil() as i32;
                    let actual_losses = stationed_losses.min(stationed_count);
                    if actual_losses > 0 {
                        had_losses = true;
                        let remaining = stationed_count - actual_losses;
                        if remaining > 0 {
                            stationed_survivors.insert(*troop_type, remaining);
                        } else {
                            stationed_survivors.remove(troop_type);
                        }
                    }
                }
            }

            if had_losses {
                ArmyRepository::update_stationed_troops(pool, stationed.id, &stationed_survivors)
                    .await?;
            }
        }

        // Calculate loyalty reduction if attacker won and has surviving Chiefs
        let mut loyalty_reduced = 0;
        let mut village_conquered = false;

        if battle.attacker_wins {
            // Calculate loyalty reduction from surviving Chiefs
            for (troop_type, count) in &battle.attacker_survivors {
                if *count > 0 && troop_type.is_chief() {
                    if let Some(def) = definitions.iter().find(|d| d.troop_type == *troop_type) {
                        loyalty_reduced += def.loyalty_reduction * count;
                    }
                }
            }

            if loyalty_reduced > 0 {
                let new_loyalty = (target.loyalty - loyalty_reduced).max(0);
                VillageRepository::update_loyalty(pool, target.id, new_loyalty).await?;

                info!(
                    "Conquer at ({}, {}): Loyalty reduced by {} (was {}, now {})",
                    army.to_x, army.to_y, loyalty_reduced, target.loyalty, new_loyalty
                );

                // Check if village is conquered (loyalty <= 0)
                if new_loyalty <= 0 {
                    // Transfer village ownership
                    VillageRepository::transfer_ownership(pool, target.id, army.player_id).await?;
                    // Reset loyalty to 25% (so it can be defended)
                    VillageRepository::update_loyalty(pool, target.id, 25).await?;
                    village_conquered = true;

                    info!(
                        "Village {} at ({}, {}) conquered by player {}!",
                        target.name, army.to_x, army.to_y, army.player_id
                    );
                }
            }
        }

        // Create battle report
        let winner = if battle.attacker_wins {
            "attacker"
        } else if battle.defender_survivors.values().sum::<i32>() > 0 {
            "defender"
        } else {
            "draw"
        };

        let report = ArmyRepository::create_battle_report(
            pool,
            army.player_id,
            Some(target.user_id),
            army.from_village_id,
            Some(target.id),
            MissionType::Conquer,
            &army.troops.0,
            &total_defender_troops,
            &battle.attacker_losses,
            &battle.defender_losses,
            &CarriedResources::default(), // No resources stolen in conquer
            winner,
            Utc::now(),
        )
        .await?;

        info!(
            "Conquer battle at ({}, {}): {} wins! Loyalty: -{}, Conquered: {}",
            army.to_x, army.to_y, winner, loyalty_reduced, village_conquered
        );

        // Initiate return journey if there are survivors
        let total_survivors: i32 = battle.attacker_survivors.values().sum();
        if total_survivors > 0 {
            Self::initiate_return(
                pool,
                army,
                battle.attacker_survivors,
                CarriedResources::default(),
                Some(report.id),
            )
            .await?;
        } else {
            ArmyRepository::delete(pool, army.id).await?;
        }

        Ok(())
    }

    /// Calculate scout power based on troop speed (faster troops = better scouts)
    fn calculate_scout_power(troops: &ArmyTroops, definitions: &[TroopDefinition]) -> f64 {
        troops
            .iter()
            .filter_map(|(troop_type, count)| {
                definitions
                    .iter()
                    .find(|d| d.troop_type == *troop_type)
                    .map(|d| d.speed as f64 * *count as f64)
            })
            .sum()
    }

    /// Calculate scout survivors after losses
    fn calculate_scout_survivors(troops: &ArmyTroops, total_losses: i32, total_count: i32) -> ArmyTroops {
        if total_count <= 0 || total_losses <= 0 {
            return troops.clone();
        }

        let loss_ratio = total_losses as f64 / total_count as f64;
        troops
            .iter()
            .map(|(troop_type, count)| {
                let losses = (*count as f64 * loss_ratio).ceil() as i32;
                (*troop_type, (*count - losses).max(0))
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }

    /// Handle army returning to home village
    async fn handle_returning_army(pool: &PgPool, army: &Army) -> AppResult<()> {
        // Return troops to village
        for (troop_type, count) in army.troops.0.iter() {
            if *count > 0 {
                TroopRepository::return_troops_to_village(pool, army.from_village_id, *troop_type, *count)
                    .await?;
            }
        }

        // Add carried resources to village
        let resources = &army.resources.0;
        if resources.total() > 0 {
            VillageRepository::add_resources(
                pool,
                army.from_village_id,
                resources.wood,
                resources.clay,
                resources.iron,
                resources.crop,
            )
            .await?;
        }

        info!(
            "Army {} returned to village {} with {} resources",
            army.id,
            army.from_village_id,
            resources.total()
        );

        // Delete army record
        ArmyRepository::delete(pool, army.id).await?;

        Ok(())
    }

    /// Initiate return journey for an army
    async fn initiate_return(
        pool: &PgPool,
        army: &Army,
        survivors: ArmyTroops,
        resources: CarriedResources,
        battle_report_id: Option<Uuid>,
    ) -> AppResult<()> {
        // Calculate return travel time based on survivors
        let definitions = TroopRepository::get_all_definitions(pool).await?;
        let from_village = VillageRepository::find_by_id(pool, army.from_village_id).await?;

        let distance = if let Some(village) = from_village {
            Self::calculate_distance(army.to_x, army.to_y, village.x, village.y)
        } else {
            Self::calculate_distance(army.to_x, army.to_y, 0, 0) // Fallback
        };

        let travel_duration = Self::calculate_travel_time(distance, &survivors, &definitions);
        let returns_at = Utc::now() + travel_duration;

        ArmyRepository::set_returning(
            pool,
            army.id,
            returns_at,
            &resources,
            &survivors,
            battle_report_id,
        )
        .await?;

        Ok(())
    }

    /// Calculate Euclidean distance between two points
    fn calculate_distance(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> f64 {
        let dx = (to_x - from_x) as f64;
        let dy = (to_y - from_y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate travel time based on distance and slowest troop
    fn calculate_travel_time(
        distance: f64,
        troops: &ArmyTroops,
        definitions: &[TroopDefinition],
    ) -> Duration {
        // Find slowest troop speed
        let slowest_speed = troops
            .iter()
            .filter(|(_, count)| **count > 0)
            .filter_map(|(troop_type, _)| {
                definitions
                    .iter()
                    .find(|d| d.troop_type == *troop_type)
                    .map(|d| d.speed)
            })
            .min()
            .unwrap_or(6); // Default speed if no troops

        // Speed is fields per hour, calculate hours needed
        let hours = distance / slowest_speed as f64;
        let seconds = (hours * 3600.0) as i64;

        // Minimum 1 minute travel time
        Duration::seconds(seconds.max(60))
    }

    /// Calculate battle using Travian-style formula
    fn calculate_battle(
        attacker_troops: &ArmyTroops,
        defender_troops: &ArmyTroops,
        definitions: &[TroopDefinition],
        mission: MissionType,
    ) -> BattleResult {
        // Calculate attack power
        let attack_power = Self::calculate_attack_power(attacker_troops, definitions);

        // Calculate infantry/cavalry ratio for defense calculation
        let (infantry_attack, cavalry_attack) =
            Self::calculate_attack_by_type(attacker_troops, definitions);
        let total_attack = infantry_attack + cavalry_attack;
        let infantry_ratio = if total_attack > 0.0 {
            infantry_attack / total_attack
        } else {
            0.5
        };

        // Calculate defense power
        let defense_power =
            Self::calculate_defense_power(defender_troops, definitions, infantry_ratio);

        // Determine winner and calculate losses
        let (attacker_wins, attacker_loss_ratio, defender_loss_ratio) =
            if attack_power > defense_power && defense_power > 0.0 {
                // Attacker wins
                let ratio = defense_power / attack_power;
                let attacker_losses = ratio.powf(1.5);
                (true, attacker_losses, 1.0)
            } else if defense_power > 0.0 {
                // Defender wins
                let ratio = attack_power / defense_power;
                let defender_losses = ratio.powf(1.5);
                // Raid: attackers can flee with reduced losses
                let attacker_losses = if mission == MissionType::Raid {
                    0.66_f64.max(1.0 - ratio * 0.5)
                } else {
                    1.0
                };
                (false, attacker_losses, defender_losses)
            } else {
                // No defenders - attacker wins with no losses
                (true, 0.0, 0.0)
            };

        // Calculate actual losses
        let attacker_losses = Self::apply_losses(attacker_troops, attacker_loss_ratio);
        let defender_losses = Self::apply_losses(defender_troops, defender_loss_ratio);

        // Calculate survivors
        let attacker_survivors = Self::calculate_survivors(attacker_troops, &attacker_losses);
        let defender_survivors = Self::calculate_survivors(defender_troops, &defender_losses);

        BattleResult {
            attacker_wins,
            attacker_survivors,
            defender_survivors,
            attacker_losses,
            defender_losses,
        }
    }

    /// Calculate total attack power
    fn calculate_attack_power(troops: &ArmyTroops, definitions: &[TroopDefinition]) -> f64 {
        troops
            .iter()
            .filter_map(|(troop_type, count)| {
                definitions
                    .iter()
                    .find(|d| d.troop_type == *troop_type)
                    .map(|d| d.attack as f64 * *count as f64)
            })
            .sum()
    }

    /// Calculate attack power split by infantry/cavalry
    fn calculate_attack_by_type(
        troops: &ArmyTroops,
        definitions: &[TroopDefinition],
    ) -> (f64, f64) {
        let mut infantry = 0.0;
        let mut cavalry = 0.0;

        for (troop_type, count) in troops {
            if let Some(def) = definitions.iter().find(|d| d.troop_type == *troop_type) {
                let attack = def.attack as f64 * *count as f64;
                if troop_type.is_cavalry() {
                    cavalry += attack;
                } else {
                    infantry += attack;
                }
            }
        }

        (infantry, cavalry)
    }

    /// Calculate total defense power based on attacker composition
    fn calculate_defense_power(
        troops: &ArmyTroops,
        definitions: &[TroopDefinition],
        infantry_ratio: f64,
    ) -> f64 {
        let cavalry_ratio = 1.0 - infantry_ratio;

        troops
            .iter()
            .filter_map(|(troop_type, count)| {
                definitions.iter().find(|d| d.troop_type == *troop_type).map(|d| {
                    let effective_defense = (d.defense_infantry as f64 * infantry_ratio)
                        + (d.defense_cavalry as f64 * cavalry_ratio);
                    effective_defense * *count as f64
                })
            })
            .sum()
    }

    /// Apply loss ratio to troops
    fn apply_losses(troops: &ArmyTroops, loss_ratio: f64) -> ArmyTroops {
        troops
            .iter()
            .map(|(troop_type, count)| {
                let losses = (*count as f64 * loss_ratio).floor() as i32;
                (*troop_type, losses.min(*count))
            })
            .filter(|(_, losses)| *losses > 0)
            .collect()
    }

    /// Calculate survivors after losses
    fn calculate_survivors(troops: &ArmyTroops, losses: &ArmyTroops) -> ArmyTroops {
        troops
            .iter()
            .map(|(troop_type, count)| {
                let loss = losses.get(troop_type).copied().unwrap_or(0);
                (*troop_type, (*count - loss).max(0))
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }

    /// Calculate resources that can be stolen
    fn calculate_stolen_resources(
        target: &Village,
        survivors: &ArmyTroops,
        definitions: &[TroopDefinition],
        mission: MissionType,
    ) -> CarriedResources {
        // Calculate total carry capacity
        let total_capacity: i32 = survivors
            .iter()
            .filter_map(|(troop_type, count)| {
                definitions
                    .iter()
                    .find(|d| d.troop_type == *troop_type)
                    .map(|d| d.carry_capacity * count)
            })
            .sum();

        if total_capacity <= 0 {
            return CarriedResources::default();
        }

        // Raid takes 50% of available, Attack takes 100%
        let raid_percent = match mission {
            MissionType::Raid => 0.5,
            MissionType::Attack | MissionType::Conquer => 1.0,
            _ => 0.0,
        };

        // Calculate available resources
        let available_wood = (target.wood as f64 * raid_percent) as i32;
        let available_clay = (target.clay as f64 * raid_percent) as i32;
        let available_iron = (target.iron as f64 * raid_percent) as i32;
        let available_crop = (target.crop as f64 * raid_percent) as i32;
        let total_available = available_wood + available_clay + available_iron + available_crop;

        if total_available <= 0 {
            return CarriedResources::default();
        }

        // Distribute proportionally up to capacity
        let factor = if total_available <= total_capacity {
            1.0
        } else {
            total_capacity as f64 / total_available as f64
        };

        CarriedResources {
            wood: (available_wood as f64 * factor) as i32,
            clay: (available_clay as f64 * factor) as i32,
            iron: (available_iron as f64 * factor) as i32,
            crop: (available_crop as f64 * factor) as i32,
        }
    }

    /// Get armies sent from a village
    pub async fn get_outgoing_armies(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<Vec<ArmyResponse>> {
        let armies = ArmyRepository::find_outgoing_from_village(pool, village_id).await?;
        Ok(armies.into_iter().map(|a| a.into()).collect())
    }

    /// Get armies incoming to a village
    pub async fn get_incoming_armies(pool: &PgPool, village_id: Uuid) -> AppResult<Vec<Army>> {
        ArmyRepository::find_incoming_to_village(pool, village_id).await
    }

    /// Get battle reports for a player
    pub async fn get_reports(pool: &PgPool, player_id: Uuid) -> AppResult<Vec<BattleReport>> {
        ArmyRepository::find_reports_by_player(pool, player_id).await
    }

    /// Get a single battle report
    pub async fn get_report(pool: &PgPool, report_id: Uuid) -> AppResult<Option<BattleReport>> {
        ArmyRepository::find_report_by_id(pool, report_id).await
    }

    /// Mark report as read
    pub async fn mark_report_read(
        pool: &PgPool,
        report_id: Uuid,
        player_id: Uuid,
    ) -> AppResult<()> {
        let report = ArmyRepository::find_report_by_id(pool, report_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Report not found".into()))?;

        let is_attacker = report.attacker_player_id == player_id;
        let is_defender = report.defender_player_id == Some(player_id);

        if !is_attacker && !is_defender {
            return Err(AppError::Forbidden);
        }

        ArmyRepository::mark_report_read(pool, report_id, is_attacker).await
    }

    // ==================== Scout Reports ====================

    /// Get scout reports for a player
    pub async fn get_scout_reports(pool: &PgPool, player_id: Uuid) -> AppResult<Vec<ScoutReport>> {
        ArmyRepository::find_scout_reports_by_player(pool, player_id).await
    }

    /// Get a single scout report
    pub async fn get_scout_report(
        pool: &PgPool,
        report_id: Uuid,
    ) -> AppResult<Option<ScoutReport>> {
        ArmyRepository::find_scout_report_by_id(pool, report_id).await
    }

    /// Mark scout report as read
    pub async fn mark_scout_report_read(
        pool: &PgPool,
        report_id: Uuid,
        player_id: Uuid,
    ) -> AppResult<()> {
        let report = ArmyRepository::find_scout_report_by_id(pool, report_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Scout report not found".into()))?;

        let is_attacker = report.attacker_player_id == player_id;
        let is_defender = report.defender_player_id == Some(player_id);

        if !is_attacker && !is_defender {
            return Err(AppError::Forbidden);
        }

        ArmyRepository::mark_scout_report_read(pool, report_id, is_attacker).await
    }

    /// Get total unread count (battle + scout reports)
    pub async fn get_total_unread_count(pool: &PgPool, player_id: Uuid) -> AppResult<i64> {
        let battle_count = ArmyRepository::count_unread_reports(pool, player_id).await?;
        let scout_count = ArmyRepository::count_unread_scout_reports(pool, player_id).await?;
        Ok(battle_count + scout_count)
    }

    // ==================== Support/Stationed Troops ====================

    /// Get troops stationed at a village (support from allies)
    pub async fn get_stationed_at_village(
        pool: &PgPool,
        village_id: Uuid,
    ) -> AppResult<Vec<ArmyResponse>> {
        let armies = ArmyRepository::find_stationed_at_village(pool, village_id).await?;
        Ok(armies.into_iter().map(|a| a.into()).collect())
    }

    /// Get support troops sent by player to other villages
    pub async fn get_support_sent(
        pool: &PgPool,
        player_id: Uuid,
    ) -> AppResult<Vec<ArmyResponse>> {
        let armies = ArmyRepository::find_support_sent_by_player(pool, player_id).await?;
        Ok(armies.into_iter().map(|a| a.into()).collect())
    }

    /// Recall stationed support troops back to home village
    pub async fn recall_support(
        pool: &PgPool,
        army_id: Uuid,
        player_id: Uuid,
    ) -> AppResult<ArmyResponse> {
        // Get the army
        let army = ArmyRepository::find_by_id(pool, army_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Army not found".into()))?;

        // Verify ownership
        if army.player_id != player_id {
            return Err(AppError::Forbidden);
        }

        // Must be stationed
        if !army.is_stationed {
            return Err(AppError::BadRequest("Army is not stationed".into()));
        }

        // Calculate return travel time
        let definitions = TroopRepository::get_all_definitions(pool).await?;
        let from_village = VillageRepository::find_by_id(pool, army.from_village_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Home village not found".into()))?;

        let distance =
            Self::calculate_distance(army.to_x, army.to_y, from_village.x, from_village.y);
        let travel_duration = Self::calculate_travel_time(distance, &army.troops.0, &definitions);
        let returns_at = Utc::now() + travel_duration;

        // Start recall
        let updated = ArmyRepository::start_recall(pool, army_id, returns_at).await?;

        info!(
            "Support army {} recalled, returning to village {} at {}",
            army_id, army.from_village_id, returns_at
        );

        Ok(updated.into())
    }
}
