// MIT License
//
// Copyright (c) 2025 Kevin Thomas
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Combat Component
//!
//! WebAssembly component implementing combat mechanics for the Legend of WASM game.
//! Handles damage calculation, combat actions, and battle management.

#![allow(dead_code)]

mod bindings;

use bindings::exports::docs::combat::actions::{
    CombatResult as ActionsCombatResult, CombatantStats as ActionsCombatantStats,
    Guest as ActionsGuest,
};
use bindings::exports::docs::combat::battle::{BattleState, Guest as BattleGuest};
use bindings::exports::docs::combat::damage::{
    AttackType, CombatantStats as DamageCombatantStats, Guest as DamageGuest,
};
use bindings::exports::docs::combat::types::CombatResult;

/// Critical hit multiplier.
const CRITICAL_MULTIPLIER: u32 = 2;

/// Minimum damage dealt per attack.
const MINIMUM_DAMAGE: u32 = 1;

/// Attack boost per 10 points of attack.
const ATTACK_DIVISOR: u32 = 10;

/// Component structure for combat functionality.
struct Component;

bindings::export!(Component with_types_in bindings);

/// Get base damage for an attack type.
///
/// # Arguments
///
/// * `attack` - Attack type
///
/// # Returns
///
/// * `u32` - Base damage value
fn attack_base_damage(attack: &AttackType) -> u32 {
    match attack {
        AttackType::SwordSlash => 10,
        AttackType::SpinAttack => 20,
        AttackType::BowShot => 8,
        AttackType::MagicAttack => 15,
        AttackType::ShieldBash => 5,
    }
}

/// Calculate stat multiplier for damage.
///
/// # Arguments
///
/// * `attack_stat` - Attacker's attack value
/// * `equipment` - Equipment bonus
///
/// # Returns
///
/// * `u32` - Multiplier value
fn calculate_multiplier(attack_stat: u32, equipment: u32) -> u32 {
    1 + (attack_stat + equipment) / ATTACK_DIVISOR
}

/// Calculate raw damage with multiplier.
///
/// # Arguments
///
/// * `base` - Base damage
/// * `multiplier` - Stat multiplier
///
/// # Returns
///
/// * `u32` - Raw damage
fn calculate_raw_damage(base: u32, multiplier: u32) -> u32 {
    base * multiplier
}

/// Calculate defense reduction.
///
/// # Arguments
///
/// * `defense` - Defense value
///
/// # Returns
///
/// * `u32` - Damage reduction
fn calculate_defense_reduction(defense: u32) -> u32 {
    defense / 2
}

/// Apply defense to raw damage.
///
/// # Arguments
///
/// * `raw` - Raw damage
/// * `reduction` - Defense reduction
///
/// # Returns
///
/// * `u32` - Final damage
fn apply_defense_reduction(raw: u32, reduction: u32) -> u32 {
    raw.saturating_sub(reduction).max(MINIMUM_DAMAGE)
}

/// Check if critical hit based on attack stat.
///
/// # Arguments
///
/// * `attack` - Attacker's attack value
///
/// # Returns
///
/// * `bool` - True if critical
fn is_critical_hit(attack: u32) -> bool {
    attack % 10 == 7
}

/// Apply critical multiplier to damage.
///
/// # Arguments
///
/// * `damage` - Base damage
/// * `is_crit` - Whether it's a critical hit
///
/// # Returns
///
/// * `u32` - Damage after crit multiplier
fn apply_critical_multiplier(damage: u32, is_crit: bool) -> u32 {
    if is_crit {
        damage * CRITICAL_MULTIPLIER
    } else {
        damage
    }
}

/// Check if target is defeated.
///
/// # Arguments
///
/// * `target_health` - Target's current health
/// * `damage` - Damage dealt
///
/// # Returns
///
/// * `bool` - True if target is defeated
fn check_defeat(target_health: u32, damage: u32) -> bool {
    damage >= target_health
}

/// Generate combat message.
///
/// # Arguments
///
/// * `damage` - Damage dealt
/// * `is_crit` - Whether it was critical
///
/// # Returns
///
/// * `String` - Combat message
fn generate_combat_message(damage: u32, is_crit: bool) -> String {
    if is_crit {
        format!("Critical hit! {} damage!", damage)
    } else {
        format!("Hit for {} damage!", damage)
    }
}

/// Create combat result.
///
/// # Arguments
///
/// * `damage` - Damage dealt
/// * `is_crit` - Whether critical
/// * `defeated` - Whether target defeated
/// * `exp` - Experience gained
///
/// # Returns
///
/// * `CombatResult` - Combat result
fn create_combat_result(damage: u32, is_crit: bool, defeated: bool, exp: u32) -> CombatResult {
    CombatResult {
        damage_dealt: damage,
        is_critical: is_crit,
        target_defeated: defeated,
        exp_gained: if defeated { exp } else { 0 },
        message: generate_combat_message(damage, is_crit),
    }
}

/// Calculate flee chance.
///
/// # Arguments
///
/// * `player_speed` - Player speed stat
/// * `enemy_speed` - Enemy speed stat
///
/// # Returns
///
/// * `bool` - True if flee successful
fn calculate_flee_success(player_speed: u32, enemy_speed: u32) -> bool {
    player_speed > enemy_speed
}

/// Check if special attack available.
///
/// # Arguments
///
/// * `attack` - Attack type
/// * `health` - Current health
/// * `max_health` - Maximum health
///
/// # Returns
///
/// * `bool` - True if can use special
fn can_use_special(attack: &AttackType, health: u32, max_health: u32) -> bool {
    match attack {
        AttackType::SpinAttack => health >= max_health / 2,
        AttackType::MagicAttack => health >= max_health / 4,
        _ => true,
    }
}

/// Create initial battle state.
///
/// # Arguments
///
/// * `player_hp` - Player health
/// * `enemy_hp` - Enemy health
///
/// # Returns
///
/// * `BattleState` - Initial battle state
fn create_battle_state(player_hp: u32, enemy_hp: u32) -> BattleState {
    BattleState {
        is_active: true,
        turn_count: 0,
        player_health: player_hp,
        enemy_health: enemy_hp,
        is_player_turn: true,
    }
}

/// End battle state.
///
/// # Arguments
///
/// * `state` - Current battle state
///
/// # Returns
///
/// * `BattleState` - Ended battle state
fn end_battle_state(state: BattleState) -> BattleState {
    BattleState {
        is_active: false,
        ..state
    }
}

/// Advance turn in battle.
///
/// # Arguments
///
/// * `state` - Current battle state
///
/// # Returns
///
/// * `BattleState` - Next turn state
fn advance_turn(state: BattleState) -> BattleState {
    BattleState {
        turn_count: state.turn_count + 1,
        is_player_turn: !state.is_player_turn,
        ..state
    }
}

/// Update battle health values.
///
/// # Arguments
///
/// * `state` - Current battle state
/// * `player_hp` - New player health
/// * `enemy_hp` - New enemy health
///
/// # Returns
///
/// * `BattleState` - Updated battle state
fn update_battle_health(state: BattleState, player_hp: u32, enemy_hp: u32) -> BattleState {
    BattleState {
        player_health: player_hp,
        enemy_health: enemy_hp,
        ..state
    }
}

/// Check if battle is over.
///
/// # Arguments
///
/// * `state` - Current battle state
///
/// # Returns
///
/// * `bool` - True if battle is over
fn check_battle_over(state: &BattleState) -> bool {
    !state.is_active || state.player_health == 0 || state.enemy_health == 0
}

/// Determine if player won.
///
/// # Arguments
///
/// * `state` - Current battle state
///
/// # Returns
///
/// * `bool` - True if player won
fn determine_player_won(state: &BattleState) -> bool {
    state.enemy_health == 0 && state.player_health > 0
}

/// Convert actions combat result.
///
/// # Arguments
///
/// * `result` - Source result
///
/// # Returns
///
/// * `ActionsCombatResult` - Converted result
fn to_actions_result(result: CombatResult) -> ActionsCombatResult {
    ActionsCombatResult {
        damage_dealt: result.damage_dealt,
        is_critical: result.is_critical,
        target_defeated: result.target_defeated,
        exp_gained: result.exp_gained,
        message: result.message,
    }
}

/// Convert actions attack type.
///
/// # Arguments
///
/// * `attack` - Source attack type
///
/// # Returns
///
/// * `AttackType` - Converted attack type
fn from_actions_attack(attack: bindings::exports::docs::combat::actions::AttackType) -> AttackType {
    match attack {
        bindings::exports::docs::combat::actions::AttackType::SwordSlash => AttackType::SwordSlash,
        bindings::exports::docs::combat::actions::AttackType::SpinAttack => AttackType::SpinAttack,
        bindings::exports::docs::combat::actions::AttackType::BowShot => AttackType::BowShot,
        bindings::exports::docs::combat::actions::AttackType::MagicAttack => {
            AttackType::MagicAttack
        }
        bindings::exports::docs::combat::actions::AttackType::ShieldBash => AttackType::ShieldBash,
    }
}

impl DamageGuest for Component {
    /// Calculate base damage for an attack type.
    ///
    /// # Arguments
    ///
    /// * `attack` - Attack type
    /// * `attacker_stats` - Attacker stats
    ///
    /// # Returns
    ///
    /// * `u32` - Base damage value
    fn calculate_base_damage(attack: AttackType, attacker_stats: DamageCombatantStats) -> u32 {
        let base = attack_base_damage(&attack);
        let mult = calculate_multiplier(attacker_stats.attack, attacker_stats.equipment_bonus);
        calculate_raw_damage(base, mult)
    }

    /// Apply defense reduction to damage.
    ///
    /// # Arguments
    ///
    /// * `raw_damage` - Raw damage value
    /// * `defender_defense` - Defender's defense
    ///
    /// # Returns
    ///
    /// * `u32` - Damage after defense
    fn apply_defense(raw_damage: u32, defender_defense: u32) -> u32 {
        let reduction = calculate_defense_reduction(defender_defense);
        apply_defense_reduction(raw_damage, reduction)
    }

    /// Roll for critical hit.
    ///
    /// # Arguments
    ///
    /// * `attacker_attack` - Attacker's attack stat
    ///
    /// # Returns
    ///
    /// * `u32` - 1 if critical, 0 otherwise
    fn roll_critical(attacker_attack: u32) -> u32 {
        if is_critical_hit(attacker_attack) {
            1
        } else {
            0
        }
    }

    /// Apply critical hit multiplier.
    ///
    /// # Arguments
    ///
    /// * `damage` - Base damage
    /// * `is_critical` - Critical flag (1 or 0)
    ///
    /// # Returns
    ///
    /// * `u32` - Damage after crit
    fn apply_critical(damage: u32, is_critical: u32) -> u32 {
        apply_critical_multiplier(damage, is_critical == 1)
    }

    /// Calculate final damage with all modifiers.
    ///
    /// # Arguments
    ///
    /// * `attack` - Attack type
    /// * `attacker` - Attacker stats
    /// * `defender` - Defender stats
    ///
    /// # Returns
    ///
    /// * `u32` - Final damage
    fn calculate_final_damage(
        attack: AttackType,
        attacker: DamageCombatantStats,
        defender: DamageCombatantStats,
    ) -> u32 {
        let base = Self::calculate_base_damage(attack, attacker);
        let after_def = Self::apply_defense(base, defender.defense);
        let crit = Self::roll_critical(attacker.attack);
        Self::apply_critical(after_def, crit)
    }
}

impl ActionsGuest for Component {
    /// Execute a player attack against an enemy.
    ///
    /// # Arguments
    ///
    /// * `attack` - Attack type
    /// * `player_stats` - Player stats
    /// * `enemy_stats` - Enemy stats
    /// * `enemy_exp` - Enemy experience reward
    ///
    /// # Returns
    ///
    /// * `CombatResult` - Result of attack
    fn player_attack(
        attack: bindings::exports::docs::combat::actions::AttackType,
        player_stats: ActionsCombatantStats,
        enemy_stats: ActionsCombatantStats,
        enemy_exp: u32,
    ) -> ActionsCombatResult {
        let att = from_actions_attack(attack);
        let p_stats = to_damage_stats(&player_stats);
        let e_stats = to_damage_stats(&enemy_stats);
        let damage = <Component as DamageGuest>::calculate_final_damage(att, p_stats, e_stats);
        let is_crit = is_critical_hit(player_stats.attack);
        let defeated = check_defeat(enemy_stats.health, damage);
        let result = create_combat_result(damage, is_crit, defeated, enemy_exp);
        to_actions_result(result)
    }

    /// Execute an enemy attack against the player.
    ///
    /// # Arguments
    ///
    /// * `enemy_attack` - Enemy attack value
    /// * `enemy_stats` - Enemy stats
    /// * `player_stats` - Player stats
    ///
    /// # Returns
    ///
    /// * `CombatResult` - Result of attack
    fn enemy_attack(
        _enemy_attack: u32,
        enemy_stats: ActionsCombatantStats,
        player_stats: ActionsCombatantStats,
    ) -> ActionsCombatResult {
        let e_stats = to_damage_stats(&enemy_stats);
        let p_stats = to_damage_stats(&player_stats);
        let damage = <Component as DamageGuest>::calculate_final_damage(
            AttackType::SwordSlash,
            e_stats,
            p_stats,
        );
        let is_crit = is_critical_hit(enemy_stats.attack);
        let defeated = check_defeat(player_stats.health, damage);
        let result = create_combat_result(damage, is_crit, defeated, 0);
        to_actions_result(result)
    }

    /// Check if player can perform special attack.
    ///
    /// # Arguments
    ///
    /// * `attack` - Attack type
    /// * `player_stats` - Player stats
    ///
    /// # Returns
    ///
    /// * `bool` - True if can use special
    fn can_special_attack(
        attack: bindings::exports::docs::combat::actions::AttackType,
        player_stats: ActionsCombatantStats,
    ) -> bool {
        let att = from_actions_attack(attack);
        can_use_special(&att, player_stats.health, player_stats.max_health)
    }

    /// Attempt to flee from combat.
    ///
    /// # Arguments
    ///
    /// * `player_speed` - Player speed
    /// * `enemy_speed` - Enemy speed
    ///
    /// # Returns
    ///
    /// * `bool` - True if successful
    fn attempt_flee(player_speed: u32, enemy_speed: u32) -> bool {
        calculate_flee_success(player_speed, enemy_speed)
    }
}

/// Convert actions stats to damage stats.
///
/// # Arguments
///
/// * `stats` - Actions stats
///
/// # Returns
///
/// * `DamageCombatantStats` - Converted stats
fn to_damage_stats(stats: &ActionsCombatantStats) -> DamageCombatantStats {
    DamageCombatantStats {
        attack: stats.attack,
        defense: stats.defense,
        health: stats.health,
        max_health: stats.max_health,
        equipment_bonus: stats.equipment_bonus,
    }
}

impl BattleGuest for Component {
    /// Start a new battle.
    ///
    /// # Arguments
    ///
    /// * `player_health` - Player starting health
    /// * `enemy_health` - Enemy starting health
    ///
    /// # Returns
    ///
    /// * `BattleState` - Initial battle state
    fn start_battle(player_health: u32, enemy_health: u32) -> BattleState {
        create_battle_state(player_health, enemy_health)
    }

    /// End the current battle.
    ///
    /// # Arguments
    ///
    /// * `state` - Current battle state
    ///
    /// # Returns
    ///
    /// * `BattleState` - Ended battle state
    fn end_battle(state: BattleState) -> BattleState {
        end_battle_state(state)
    }

    /// Advance to the next turn.
    ///
    /// # Arguments
    ///
    /// * `state` - Current battle state
    ///
    /// # Returns
    ///
    /// * `BattleState` - Next turn state
    fn next_turn(state: BattleState) -> BattleState {
        advance_turn(state)
    }

    /// Update battle state after combat action.
    ///
    /// # Arguments
    ///
    /// * `state` - Current battle state
    /// * `player_health` - Updated player health
    /// * `enemy_health` - Updated enemy health
    ///
    /// # Returns
    ///
    /// * `BattleState` - Updated battle state
    fn update_health(state: BattleState, player_health: u32, enemy_health: u32) -> BattleState {
        update_battle_health(state, player_health, enemy_health)
    }

    /// Check if the battle is over.
    ///
    /// # Arguments
    ///
    /// * `state` - Current battle state
    ///
    /// # Returns
    ///
    /// * `bool` - True if battle over
    fn is_battle_over(state: BattleState) -> bool {
        check_battle_over(&state)
    }

    /// Determine if player won the battle.
    ///
    /// # Arguments
    ///
    /// * `state` - Current battle state
    ///
    /// # Returns
    ///
    /// * `bool` - True if player won
    fn player_won(state: BattleState) -> bool {
        determine_player_won(&state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test sword slash base damage.
    fn test_attack_base_damage_sword() {
        let damage = attack_base_damage(&AttackType::SwordSlash);
        assert_eq!(damage, 10);
    }

    #[test]
    /// Test spin attack base damage.
    fn test_attack_base_damage_spin() {
        let damage = attack_base_damage(&AttackType::SpinAttack);
        assert_eq!(damage, 20);
    }

    #[test]
    /// Test bow shot base damage.
    fn test_attack_base_damage_bow() {
        let damage = attack_base_damage(&AttackType::BowShot);
        assert_eq!(damage, 8);
    }

    #[test]
    /// Test multiplier calculation.
    fn test_calculate_multiplier() {
        let mult = calculate_multiplier(20, 5);
        assert_eq!(mult, 3);
    }

    #[test]
    /// Test raw damage calculation.
    fn test_calculate_raw_damage() {
        let damage = calculate_raw_damage(10, 2);
        assert_eq!(damage, 20);
    }

    #[test]
    /// Test defense reduction calculation.
    fn test_calculate_defense_reduction() {
        let reduction = calculate_defense_reduction(10);
        assert_eq!(reduction, 5);
    }

    #[test]
    /// Test apply defense reduction.
    fn test_apply_defense_reduction() {
        let damage = apply_defense_reduction(20, 5);
        assert_eq!(damage, 15);
    }

    #[test]
    /// Test minimum damage applied.
    fn test_apply_defense_reduction_minimum() {
        let damage = apply_defense_reduction(5, 100);
        assert_eq!(damage, MINIMUM_DAMAGE);
    }

    #[test]
    /// Test critical hit detection true.
    fn test_is_critical_hit_true() {
        assert!(is_critical_hit(17));
    }

    #[test]
    /// Test critical hit detection false.
    fn test_is_critical_hit_false() {
        assert!(!is_critical_hit(15));
    }

    #[test]
    /// Test critical multiplier applied.
    fn test_apply_critical_multiplier_crit() {
        let damage = apply_critical_multiplier(10, true);
        assert_eq!(damage, 20);
    }

    #[test]
    /// Test no critical multiplier.
    fn test_apply_critical_multiplier_no_crit() {
        let damage = apply_critical_multiplier(10, false);
        assert_eq!(damage, 10);
    }

    #[test]
    /// Test defeat check true.
    fn test_check_defeat_true() {
        assert!(check_defeat(10, 15));
    }

    #[test]
    /// Test defeat check false.
    fn test_check_defeat_false() {
        assert!(!check_defeat(100, 15));
    }

    #[test]
    /// Test combat message crit.
    fn test_generate_combat_message_crit() {
        let msg = generate_combat_message(50, true);
        assert!(msg.contains("Critical"));
    }

    #[test]
    /// Test combat message normal.
    fn test_generate_combat_message_normal() {
        let msg = generate_combat_message(25, false);
        assert!(msg.contains("25"));
    }

    #[test]
    /// Test flee success.
    fn test_calculate_flee_success_true() {
        assert!(calculate_flee_success(10, 5));
    }

    #[test]
    /// Test flee failure.
    fn test_calculate_flee_success_false() {
        assert!(!calculate_flee_success(5, 10));
    }

    #[test]
    /// Test can use spin attack.
    fn test_can_use_special_spin_true() {
        assert!(can_use_special(&AttackType::SpinAttack, 60, 100));
    }

    #[test]
    /// Test cannot use spin attack.
    fn test_can_use_special_spin_false() {
        assert!(!can_use_special(&AttackType::SpinAttack, 40, 100));
    }

    #[test]
    /// Test create battle state.
    fn test_create_battle_state() {
        let state = create_battle_state(100, 50);
        assert!(state.is_active);
        assert!(state.is_player_turn);
    }

    #[test]
    /// Test end battle state.
    fn test_end_battle_state() {
        let state = create_battle_state(100, 50);
        let ended = end_battle_state(state);
        assert!(!ended.is_active);
    }

    #[test]
    /// Test advance turn.
    fn test_advance_turn() {
        let state = create_battle_state(100, 50);
        let next = advance_turn(state);
        assert!(!next.is_player_turn);
        assert_eq!(next.turn_count, 1);
    }

    #[test]
    /// Test update battle health.
    fn test_update_battle_health() {
        let state = create_battle_state(100, 50);
        let updated = update_battle_health(state, 80, 30);
        assert_eq!(updated.player_health, 80);
        assert_eq!(updated.enemy_health, 30);
    }

    #[test]
    /// Test battle over when not active.
    fn test_check_battle_over_inactive() {
        let mut state = create_battle_state(100, 50);
        state.is_active = false;
        assert!(check_battle_over(&state));
    }

    #[test]
    /// Test battle over when player dead.
    fn test_check_battle_over_player_dead() {
        let mut state = create_battle_state(100, 50);
        state.player_health = 0;
        assert!(check_battle_over(&state));
    }

    #[test]
    /// Test player won.
    fn test_determine_player_won_true() {
        let mut state = create_battle_state(100, 50);
        state.enemy_health = 0;
        assert!(determine_player_won(&state));
    }

    #[test]
    /// Test player lost.
    fn test_determine_player_won_false() {
        let mut state = create_battle_state(100, 50);
        state.player_health = 0;
        assert!(!determine_player_won(&state));
    }

    #[test]
    /// Test start battle function.
    fn test_start_battle() {
        let state = <Component as BattleGuest>::start_battle(100, 50);
        assert!(state.is_active);
    }

    #[test]
    /// Test end battle function.
    fn test_end_battle() {
        let state = create_battle_state(100, 50);
        let ended = <Component as BattleGuest>::end_battle(state);
        assert!(!ended.is_active);
    }

    #[test]
    /// Test next turn function.
    fn test_next_turn() {
        let state = create_battle_state(100, 50);
        let next = <Component as BattleGuest>::next_turn(state);
        assert_eq!(next.turn_count, 1);
    }

    #[test]
    /// Test is battle over function.
    fn test_is_battle_over() {
        let mut state = create_battle_state(100, 50);
        state.enemy_health = 0;
        assert!(<Component as BattleGuest>::is_battle_over(state));
    }

    #[test]
    /// Test player won function.
    fn test_player_won() {
        let mut state = create_battle_state(100, 50);
        state.enemy_health = 0;
        assert!(<Component as BattleGuest>::player_won(state));
    }

    #[test]
    /// Test attempt flee success.
    fn test_attempt_flee_success() {
        assert!(<Component as ActionsGuest>::attempt_flee(15, 10));
    }

    #[test]
    /// Test attempt flee failure.
    fn test_attempt_flee_failure() {
        assert!(!<Component as ActionsGuest>::attempt_flee(5, 10));
    }
}
