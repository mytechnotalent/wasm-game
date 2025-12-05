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

//! # Enemy Component
//!
//! WebAssembly component implementing enemy mechanics for the Legend of WASM game.
//! Handles enemy spawning, AI behavior, and damage calculation.

#![allow(dead_code)]

mod bindings;

use bindings::exports::docs::enemy::ai::{Guest as AiGuest, Position as AiPosition};
use bindings::exports::docs::enemy::damage::Guest as DamageGuest;
use bindings::exports::docs::enemy::spawn::{Guest as SpawnGuest, Position as SpawnPosition};
use bindings::exports::docs::enemy::types::{Behavior, EnemyKind, EnemyState, Position};

/// Chase distance threshold for AI decisions.
const CHASE_DISTANCE: u32 = 5;

/// Attack range for melee enemies.
const ATTACK_RANGE: u32 = 1;

/// Flee health threshold percentage.
const FLEE_THRESHOLD: u32 = 20;

/// Component structure for enemy functionality.
struct Component;

bindings::export!(Component with_types_in bindings);

/// Get base health for an enemy kind.
///
/// # Arguments
///
/// * `kind` - The type of enemy
///
/// # Returns
///
/// * `u32` - Base health value
fn base_health(kind: &EnemyKind) -> u32 {
    match kind {
        EnemyKind::Slime => 30,
        EnemyKind::Skeleton => 50,
        EnemyKind::Bat => 20,
        EnemyKind::Goblin => 40,
        EnemyKind::DarkKnight => 80,
        EnemyKind::Boss => 200,
    }
}

/// Get base attack for an enemy kind.
///
/// # Arguments
///
/// * `kind` - The type of enemy
///
/// # Returns
///
/// * `u32` - Base attack value
fn base_attack(kind: &EnemyKind) -> u32 {
    match kind {
        EnemyKind::Slime => 5,
        EnemyKind::Skeleton => 12,
        EnemyKind::Bat => 8,
        EnemyKind::Goblin => 10,
        EnemyKind::DarkKnight => 20,
        EnemyKind::Boss => 30,
    }
}

/// Get base defense for an enemy kind.
///
/// # Arguments
///
/// * `kind` - The type of enemy
///
/// # Returns
///
/// * `u32` - Base defense value
fn base_defense(kind: &EnemyKind) -> u32 {
    match kind {
        EnemyKind::Slime => 2,
        EnemyKind::Skeleton => 5,
        EnemyKind::Bat => 1,
        EnemyKind::Goblin => 4,
        EnemyKind::DarkKnight => 15,
        EnemyKind::Boss => 20,
    }
}

/// Get experience reward for an enemy kind.
///
/// # Arguments
///
/// * `kind` - The type of enemy
///
/// # Returns
///
/// * `u32` - Experience reward
fn base_exp_reward(kind: &EnemyKind) -> u32 {
    match kind {
        EnemyKind::Slime => 10,
        EnemyKind::Skeleton => 25,
        EnemyKind::Bat => 15,
        EnemyKind::Goblin => 20,
        EnemyKind::DarkKnight => 50,
        EnemyKind::Boss => 100,
    }
}

/// Get default behavior for an enemy kind.
///
/// # Arguments
///
/// * `kind` - The type of enemy
///
/// # Returns
///
/// * `Behavior` - Default behavior pattern
fn default_behavior(kind: &EnemyKind) -> Behavior {
    match kind {
        EnemyKind::Slime => Behavior::Wander,
        EnemyKind::Skeleton => Behavior::Guard,
        EnemyKind::Bat => Behavior::Chase,
        EnemyKind::Goblin => Behavior::Chase,
        EnemyKind::DarkKnight => Behavior::Guard,
        EnemyKind::Boss => Behavior::BossPattern,
    }
}

/// Create base enemy state from kind and position.
///
/// # Arguments
///
/// * `kind` - Type of enemy
/// * `pos` - Spawn position
///
/// # Returns
///
/// * `EnemyState` - Initial enemy state
fn create_enemy_state(kind: EnemyKind, pos: Position) -> EnemyState {
    let health = base_health(&kind);
    EnemyState {
        kind,
        health,
        max_health: health,
        attack: base_attack(&kind),
        defense: base_defense(&kind),
        exp_reward: base_exp_reward(&kind),
        pos,
        current_behavior: default_behavior(&kind),
        is_alive: true,
    }
}

/// Create boss state with enhanced stats.
///
/// # Arguments
///
/// * `pos` - Spawn position
///
/// # Returns
///
/// * `EnemyState` - Boss enemy state
fn create_boss_state(pos: Position) -> EnemyState {
    EnemyState {
        kind: EnemyKind::Boss,
        health: 200,
        max_health: 200,
        attack: 30,
        defense: 20,
        exp_reward: 100,
        pos,
        current_behavior: Behavior::BossPattern,
        is_alive: true,
    }
}

/// Calculate horizontal distance.
///
/// # Arguments
///
/// * `x1` - First X coordinate
/// * `x2` - Second X coordinate
///
/// # Returns
///
/// * `u32` - Absolute horizontal distance
fn horizontal_distance(x1: i32, x2: i32) -> u32 {
    (x1 - x2).unsigned_abs()
}

/// Calculate vertical distance.
///
/// # Arguments
///
/// * `y1` - First Y coordinate
/// * `y2` - Second Y coordinate
///
/// # Returns
///
/// * `u32` - Absolute vertical distance
fn vertical_distance(y1: i32, y2: i32) -> u32 {
    (y1 - y2).unsigned_abs()
}

/// Calculate Manhattan distance between positions.
///
/// # Arguments
///
/// * `enemy_pos` - Enemy position
/// * `player_pos` - Player position
///
/// # Returns
///
/// * `u32` - Manhattan distance
fn manhattan_distance(enemy_pos: &Position, player_pos: &AiPosition) -> u32 {
    let h_dist = horizontal_distance(enemy_pos.x, player_pos.x);
    let v_dist = vertical_distance(enemy_pos.y, player_pos.y);
    h_dist + v_dist
}

/// Calculate step toward player X coordinate.
///
/// # Arguments
///
/// * `enemy_x` - Enemy X position
/// * `player_x` - Player X position
///
/// # Returns
///
/// * `i32` - Step direction (-1, 0, or 1)
fn step_toward_x(enemy_x: i32, player_x: i32) -> i32 {
    if player_x > enemy_x {
        1
    } else if player_x < enemy_x {
        -1
    } else {
        0
    }
}

/// Calculate step toward player Y coordinate.
///
/// # Arguments
///
/// * `enemy_y` - Enemy Y position
/// * `player_y` - Player Y position
///
/// # Returns
///
/// * `i32` - Step direction (-1, 0, or 1)
fn step_toward_y(enemy_y: i32, player_y: i32) -> i32 {
    if player_y > enemy_y {
        1
    } else if player_y < enemy_y {
        -1
    } else {
        0
    }
}

/// Calculate chase movement position.
///
/// # Arguments
///
/// * `enemy` - Enemy state
/// * `player_pos` - Player position
///
/// # Returns
///
/// * `AiPosition` - New position toward player
fn chase_movement(enemy: &EnemyState, player_pos: &AiPosition) -> AiPosition {
    let dx = step_toward_x(enemy.pos.x, player_pos.x);
    let dy = step_toward_y(enemy.pos.y, player_pos.y);
    AiPosition {
        x: enemy.pos.x + dx,
        y: enemy.pos.y + dy,
    }
}

/// Calculate flee movement position.
///
/// # Arguments
///
/// * `enemy` - Enemy state
/// * `player_pos` - Player position
///
/// # Returns
///
/// * `AiPosition` - New position away from player
fn flee_movement(enemy: &EnemyState, player_pos: &AiPosition) -> AiPosition {
    let dx = -step_toward_x(enemy.pos.x, player_pos.x);
    let dy = -step_toward_y(enemy.pos.y, player_pos.y);
    AiPosition {
        x: enemy.pos.x + dx,
        y: enemy.pos.y + dy,
    }
}

/// Calculate wander movement position.
///
/// # Arguments
///
/// * `enemy` - Enemy state
///
/// # Returns
///
/// * `AiPosition` - Random wander position
fn wander_movement(enemy: &EnemyState) -> AiPosition {
    let offset = (enemy.pos.x + enemy.pos.y) % 4;
    let (dx, dy) = wander_offset(offset);
    AiPosition {
        x: enemy.pos.x + dx,
        y: enemy.pos.y + dy,
    }
}

/// Get wander offset based on pseudo-random value.
///
/// # Arguments
///
/// * `offset` - Pseudo-random offset value
///
/// # Returns
///
/// * `(i32, i32)` - X and Y offset tuple
fn wander_offset(offset: i32) -> (i32, i32) {
    match offset {
        0 => (1, 0),
        1 => (-1, 0),
        2 => (0, 1),
        _ => (0, -1),
    }
}

/// Stay at current position (for guard behavior).
///
/// # Arguments
///
/// * `enemy` - Enemy state
///
/// # Returns
///
/// * `AiPosition` - Current position
fn guard_position(enemy: &EnemyState) -> AiPosition {
    AiPosition {
        x: enemy.pos.x,
        y: enemy.pos.y,
    }
}

/// Calculate movement based on behavior.
///
/// # Arguments
///
/// * `enemy` - Enemy state
/// * `player_pos` - Player position
///
/// # Returns
///
/// * `AiPosition` - New position based on behavior
fn movement_by_behavior(enemy: &EnemyState, player_pos: &AiPosition) -> AiPosition {
    match enemy.current_behavior {
        Behavior::Chase => chase_movement(enemy, player_pos),
        Behavior::Flee => flee_movement(enemy, player_pos),
        Behavior::Wander => wander_movement(enemy),
        Behavior::Guard | Behavior::BossPattern => guard_position(enemy),
    }
}

/// Check if enemy health is below flee threshold.
///
/// # Arguments
///
/// * `enemy` - Enemy state
///
/// # Returns
///
/// * `bool` - True if should flee
fn is_low_health(enemy: &EnemyState) -> bool {
    let health_percent = (enemy.health * 100) / enemy.max_health;
    health_percent < FLEE_THRESHOLD
}

/// Determine new behavior based on state.
///
/// # Arguments
///
/// * `enemy` - Enemy state
///
/// # Returns
///
/// * `Behavior` - Updated behavior
fn determine_behavior(enemy: &EnemyState) -> Behavior {
    if is_low_health(enemy) && enemy.kind != EnemyKind::Boss {
        return Behavior::Flee;
    }
    default_behavior(&enemy.kind)
}

/// Calculate effective damage after defense.
///
/// # Arguments
///
/// * `raw_damage` - Raw incoming damage
/// * `defense` - Defense value
///
/// # Returns
///
/// * `u32` - Damage after defense reduction
fn calculate_effective_damage(raw_damage: u32, defense: u32) -> u32 {
    let reduction = defense / 2;
    raw_damage.saturating_sub(reduction).max(1)
}

/// Apply damage to enemy state.
///
/// # Arguments
///
/// * `enemy` - Enemy state
/// * `damage` - Damage to apply
///
/// # Returns
///
/// * `EnemyState` - Updated enemy state
fn apply_damage(enemy: EnemyState, damage: u32) -> EnemyState {
    let new_health = enemy.health.saturating_sub(damage);
    let is_alive = new_health > 0;
    EnemyState {
        health: new_health,
        is_alive,
        ..enemy
    }
}

/// Convert spawn position to types position.
///
/// # Arguments
///
/// * `pos` - Spawn position
///
/// # Returns
///
/// * `Position` - Types position
fn spawn_to_types_pos(pos: SpawnPosition) -> Position {
    Position { x: pos.x, y: pos.y }
}

impl SpawnGuest for Component {
    /// Spawn a new enemy of specified kind at position.
    ///
    /// # Arguments
    ///
    /// * `kind` - Type of enemy to spawn
    /// * `pos` - Spawn position
    ///
    /// # Returns
    ///
    /// * `EnemyState` - New enemy state
    fn spawn_enemy(kind: EnemyKind, pos: SpawnPosition) -> EnemyState {
        let types_pos = spawn_to_types_pos(pos);
        create_enemy_state(kind, types_pos)
    }

    /// Spawn a boss enemy at position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Spawn position
    ///
    /// # Returns
    ///
    /// * `EnemyState` - Boss enemy state
    fn spawn_boss(pos: SpawnPosition) -> EnemyState {
        let types_pos = spawn_to_types_pos(pos);
        create_boss_state(types_pos)
    }

    /// Get base stats for an enemy kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - Type of enemy
    ///
    /// # Returns
    ///
    /// * `EnemyState` - Base stats at origin
    fn get_base_stats(kind: EnemyKind) -> EnemyState {
        let pos = Position { x: 0, y: 0 };
        create_enemy_state(kind, pos)
    }
}

impl AiGuest for Component {
    /// Calculate enemy's next move based on player position.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    /// * `player_pos` - Player position
    ///
    /// # Returns
    ///
    /// * `Position` - New position for enemy
    fn calculate_move(enemy: EnemyState, player_pos: AiPosition) -> AiPosition {
        movement_by_behavior(&enemy, &player_pos)
    }

    /// Determine if enemy should attack player.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    /// * `player_pos` - Player position
    ///
    /// # Returns
    ///
    /// * `bool` - True if should attack
    fn should_attack(enemy: EnemyState, player_pos: AiPosition) -> bool {
        let distance = manhattan_distance(&enemy.pos, &player_pos);
        distance <= ATTACK_RANGE
    }

    /// Update enemy behavior based on current state.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    ///
    /// # Returns
    ///
    /// * `Behavior` - Updated behavior
    fn update_behavior(enemy: EnemyState) -> Behavior {
        determine_behavior(&enemy)
    }

    /// Get attack damage for this enemy.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    ///
    /// # Returns
    ///
    /// * `u32` - Attack damage value
    fn get_attack_damage(enemy: EnemyState) -> u32 {
        enemy.attack
    }
}

impl DamageGuest for Component {
    /// Apply damage to enemy with defense calculation.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    /// * `raw_damage` - Raw incoming damage
    ///
    /// # Returns
    ///
    /// * `EnemyState` - Updated enemy state
    fn take_damage(enemy: EnemyState, raw_damage: u32) -> EnemyState {
        let effective = calculate_effective_damage(raw_damage, enemy.defense);
        apply_damage(enemy, effective)
    }

    /// Check if enemy is defeated.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    ///
    /// # Returns
    ///
    /// * `bool` - True if defeated
    fn is_defeated(enemy: EnemyState) -> bool {
        !enemy.is_alive || enemy.health == 0
    }

    /// Get experience reward for defeating enemy.
    ///
    /// # Arguments
    ///
    /// * `enemy` - Enemy state
    ///
    /// # Returns
    ///
    /// * `u32` - Experience reward
    fn get_exp_reward(enemy: EnemyState) -> u32 {
        enemy.exp_reward
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test slime base health is correct.
    fn test_base_health_slime() {
        let health = base_health(&EnemyKind::Slime);
        assert_eq!(health, 30);
    }

    #[test]
    /// Test boss base health is correct.
    fn test_base_health_boss() {
        let health = base_health(&EnemyKind::Boss);
        assert_eq!(health, 200);
    }

    #[test]
    /// Test skeleton base attack is correct.
    fn test_base_attack_skeleton() {
        let attack = base_attack(&EnemyKind::Skeleton);
        assert_eq!(attack, 12);
    }

    #[test]
    /// Test dark knight base defense is correct.
    fn test_base_defense_dark_knight() {
        let defense = base_defense(&EnemyKind::DarkKnight);
        assert_eq!(defense, 15);
    }

    #[test]
    /// Test goblin experience reward is correct.
    fn test_base_exp_reward_goblin() {
        let exp = base_exp_reward(&EnemyKind::Goblin);
        assert_eq!(exp, 20);
    }

    #[test]
    /// Test bat default behavior is chase.
    fn test_default_behavior_bat() {
        let behavior = default_behavior(&EnemyKind::Bat);
        assert!(matches!(behavior, Behavior::Chase));
    }

    #[test]
    /// Test skeleton default behavior is guard.
    fn test_default_behavior_skeleton() {
        let behavior = default_behavior(&EnemyKind::Skeleton);
        assert!(matches!(behavior, Behavior::Guard));
    }

    #[test]
    /// Test horizontal distance calculation.
    fn test_horizontal_distance() {
        let dist = horizontal_distance(10, 3);
        assert_eq!(dist, 7);
    }

    #[test]
    /// Test vertical distance calculation.
    fn test_vertical_distance() {
        let dist = vertical_distance(5, 12);
        assert_eq!(dist, 7);
    }

    #[test]
    /// Test step toward player X positive.
    fn test_step_toward_x_positive() {
        let step = step_toward_x(0, 5);
        assert_eq!(step, 1);
    }

    #[test]
    /// Test step toward player X negative.
    fn test_step_toward_x_negative() {
        let step = step_toward_x(5, 0);
        assert_eq!(step, -1);
    }

    #[test]
    /// Test step toward player X zero.
    fn test_step_toward_x_zero() {
        let step = step_toward_x(5, 5);
        assert_eq!(step, 0);
    }

    #[test]
    /// Test step toward player Y positive.
    fn test_step_toward_y_positive() {
        let step = step_toward_y(0, 5);
        assert_eq!(step, 1);
    }

    #[test]
    /// Test wander offset 0 returns right.
    fn test_wander_offset_0() {
        let offset = wander_offset(0);
        assert_eq!(offset, (1, 0));
    }

    #[test]
    /// Test wander offset 1 returns left.
    fn test_wander_offset_1() {
        let offset = wander_offset(1);
        assert_eq!(offset, (-1, 0));
    }

    #[test]
    /// Test effective damage calculation.
    fn test_calculate_effective_damage() {
        let damage = calculate_effective_damage(20, 10);
        assert_eq!(damage, 15);
    }

    #[test]
    /// Test minimum damage is 1.
    fn test_calculate_effective_damage_minimum() {
        let damage = calculate_effective_damage(5, 100);
        assert_eq!(damage, 1);
    }

    #[test]
    /// Test creating enemy state.
    fn test_create_enemy_state() {
        let pos = Position { x: 5, y: 5 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        assert_eq!(enemy.health, 30);
        assert!(enemy.is_alive);
    }

    #[test]
    /// Test creating boss state.
    fn test_create_boss_state() {
        let pos = Position { x: 0, y: 0 };
        let boss = create_boss_state(pos);
        assert_eq!(boss.health, 200);
        assert!(matches!(boss.kind, EnemyKind::Boss));
    }

    #[test]
    /// Test low health detection.
    fn test_is_low_health_true() {
        let pos = Position { x: 0, y: 0 };
        let mut enemy = create_enemy_state(EnemyKind::Slime, pos);
        enemy.health = 5;
        assert!(is_low_health(&enemy));
    }

    #[test]
    /// Test not low health.
    fn test_is_low_health_false() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        assert!(!is_low_health(&enemy));
    }

    #[test]
    /// Test spawn enemy function.
    fn test_spawn_enemy() {
        let pos = SpawnPosition { x: 10, y: 20 };
        let enemy = <Component as SpawnGuest>::spawn_enemy(EnemyKind::Goblin, pos);
        assert_eq!(enemy.pos.x, 10);
        assert_eq!(enemy.pos.y, 20);
    }

    #[test]
    /// Test spawn boss function.
    fn test_spawn_boss() {
        let pos = SpawnPosition { x: 0, y: 0 };
        let boss = <Component as SpawnGuest>::spawn_boss(pos);
        assert!(matches!(boss.kind, EnemyKind::Boss));
    }

    #[test]
    /// Test get base stats function.
    fn test_get_base_stats() {
        let stats = <Component as SpawnGuest>::get_base_stats(EnemyKind::Skeleton);
        assert_eq!(stats.health, 50);
    }

    #[test]
    /// Test should attack in range.
    fn test_should_attack_true() {
        let pos = Position { x: 5, y: 5 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        let player = AiPosition { x: 5, y: 6 };
        assert!(<Component as AiGuest>::should_attack(enemy, player));
    }

    #[test]
    /// Test should not attack out of range.
    fn test_should_attack_false() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        let player = AiPosition { x: 10, y: 10 };
        assert!(!<Component as AiGuest>::should_attack(enemy, player));
    }

    #[test]
    /// Test get attack damage.
    fn test_get_attack_damage() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Skeleton, pos);
        let damage = <Component as AiGuest>::get_attack_damage(enemy);
        assert_eq!(damage, 12);
    }

    #[test]
    /// Test take damage function.
    fn test_take_damage() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        let damaged = <Component as DamageGuest>::take_damage(enemy, 10);
        assert!(damaged.health < 30);
    }

    #[test]
    /// Test is defeated true.
    fn test_is_defeated_true() {
        let pos = Position { x: 0, y: 0 };
        let mut enemy = create_enemy_state(EnemyKind::Slime, pos);
        enemy.health = 0;
        enemy.is_alive = false;
        assert!(<Component as DamageGuest>::is_defeated(enemy));
    }

    #[test]
    /// Test is defeated false.
    fn test_is_defeated_false() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Slime, pos);
        assert!(!<Component as DamageGuest>::is_defeated(enemy));
    }

    #[test]
    /// Test get exp reward.
    fn test_get_exp_reward() {
        let pos = Position { x: 0, y: 0 };
        let enemy = create_enemy_state(EnemyKind::Boss, pos);
        let exp = <Component as DamageGuest>::get_exp_reward(enemy);
        assert_eq!(exp, 100);
    }
}
