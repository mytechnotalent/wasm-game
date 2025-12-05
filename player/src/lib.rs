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

//! # Player Component
//!
//! WebAssembly component implementing player mechanics for the Legend of WASM game.
//! Handles player movement, health management, experience, and leveling.

#![allow(dead_code)]

mod bindings;

use bindings::exports::docs::player::movement::{Direction, Guest as MovementGuest, Position};
use bindings::exports::docs::player::stats::{Guest as StatsGuest, PlayerStats};

/// Default starting health for new players.
const STARTING_HEALTH: u32 = 100;

/// Default starting attack power for new players.
const STARTING_ATTACK: u32 = 10;

/// Default starting defense for new players.
const STARTING_DEFENSE: u32 = 5;

/// Base experience required for level 2.
const BASE_EXP_REQUIREMENT: u32 = 100;

/// Experience multiplier per level.
const EXP_MULTIPLIER: f32 = 1.5;

/// Health bonus per level up.
const HEALTH_PER_LEVEL: u32 = 20;

/// Attack bonus per level up.
const ATTACK_PER_LEVEL: u32 = 3;

/// Defense bonus per level up.
const DEFENSE_PER_LEVEL: u32 = 2;

/// Component structure for player functionality.
struct Component;

bindings::export!(Component with_types_in bindings);

/// Calculate delta X for a given direction.
///
/// # Arguments
///
/// * `dir` - The direction to calculate delta for
///
/// # Returns
///
/// * `i32` - The X coordinate change (-1, 0, or 1)
fn delta_x(dir: &Direction) -> i32 {
    match dir {
        Direction::West => -1,
        Direction::East => 1,
        _ => 0,
    }
}

/// Calculate delta Y for a given direction.
///
/// # Arguments
///
/// * `dir` - The direction to calculate delta for
///
/// # Returns
///
/// * `i32` - The Y coordinate change (-1, 0, or 1)
fn delta_y(dir: &Direction) -> i32 {
    match dir {
        Direction::North => -1,
        Direction::South => 1,
        _ => 0,
    }
}

/// Calculate new X position after movement.
///
/// # Arguments
///
/// * `current` - Current X coordinate
/// * `dx` - Delta X value
///
/// # Returns
///
/// * `i32` - New X coordinate
fn new_x(current: i32, dx: i32) -> i32 {
    current + dx
}

/// Calculate new Y position after movement.
///
/// # Arguments
///
/// * `current` - Current Y coordinate
/// * `dy` - Delta Y value
///
/// # Returns
///
/// * `i32` - New Y coordinate
fn new_y(current: i32, dy: i32) -> i32 {
    current + dy
}

/// Calculate horizontal distance component.
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

/// Calculate vertical distance component.
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

/// Create default player stats for a new game.
///
/// # Returns
///
/// * `PlayerStats` - Fresh player stats
fn create_default_stats() -> PlayerStats {
    PlayerStats {
        health: STARTING_HEALTH,
        max_health: STARTING_HEALTH,
        attack: STARTING_ATTACK,
        defense: STARTING_DEFENSE,
        experience: 0,
        level: 1,
    }
}

/// Calculate effective damage after defense.
///
/// # Arguments
///
/// * `raw_damage` - Damage before defense reduction
/// * `defense` - Defense value to apply
///
/// # Returns
///
/// * `u32` - Damage after defense reduction
fn calculate_effective_damage(raw_damage: u32, defense: u32) -> u32 {
    let reduction = defense / 2;
    raw_damage.saturating_sub(reduction).max(1)
}

/// Apply damage to health value.
///
/// # Arguments
///
/// * `current_health` - Current health points
/// * `damage` - Damage to apply
///
/// # Returns
///
/// * `u32` - Health after damage
fn apply_damage_to_health(current_health: u32, damage: u32) -> u32 {
    current_health.saturating_sub(damage)
}

/// Calculate healed health value.
///
/// # Arguments
///
/// * `current` - Current health
/// * `amount` - Amount to heal
/// * `max` - Maximum health cap
///
/// # Returns
///
/// * `u32` - Health after healing
fn calculate_healed_health(current: u32, amount: u32, max: u32) -> u32 {
    (current + amount).min(max)
}

/// Check if player should level up.
///
/// # Arguments
///
/// * `experience` - Current experience points
/// * `level` - Current level
///
/// # Returns
///
/// * `bool` - True if should level up
fn should_level_up(experience: u32, level: u32) -> bool {
    let required = calculate_exp_requirement(level);
    experience >= required
}

/// Calculate experience requirement for next level.
///
/// # Arguments
///
/// * `level` - Current level
///
/// # Returns
///
/// * `u32` - Experience needed for next level
fn calculate_exp_requirement(level: u32) -> u32 {
    let multiplier = EXP_MULTIPLIER.powi(level as i32 - 1);
    (BASE_EXP_REQUIREMENT as f32 * multiplier) as u32
}

/// Calculate new max health after level up.
///
/// # Arguments
///
/// * `current_max` - Current max health
///
/// # Returns
///
/// * `u32` - New max health
fn level_up_max_health(current_max: u32) -> u32 {
    current_max + HEALTH_PER_LEVEL
}

/// Calculate new attack after level up.
///
/// # Arguments
///
/// * `current` - Current attack
///
/// # Returns
///
/// * `u32` - New attack value
fn level_up_attack(current: u32) -> u32 {
    current + ATTACK_PER_LEVEL
}

/// Calculate new defense after level up.
///
/// # Arguments
///
/// * `current` - Current defense
///
/// # Returns
///
/// * `u32` - New defense value
fn level_up_defense(current: u32) -> u32 {
    current + DEFENSE_PER_LEVEL
}

/// Apply level up bonuses to stats.
///
/// # Arguments
///
/// * `stats` - Current player stats
///
/// # Returns
///
/// * `PlayerStats` - Stats after level up
fn apply_level_up(stats: PlayerStats) -> PlayerStats {
    PlayerStats {
        health: level_up_max_health(stats.max_health),
        max_health: level_up_max_health(stats.max_health),
        attack: level_up_attack(stats.attack),
        defense: level_up_defense(stats.defense),
        experience: stats.experience,
        level: stats.level + 1,
    }
}

/// Process experience gain with level up check.
///
/// # Arguments
///
/// * `stats` - Current player stats
/// * `exp` - Experience gained
///
/// # Returns
///
/// * `PlayerStats` - Updated stats
fn process_experience_gain(stats: PlayerStats, exp: u32) -> PlayerStats {
    let new_exp = stats.experience + exp;
    let updated = create_stats_with_exp(stats, new_exp);
    check_and_apply_level_up(updated)
}

/// Create stats with updated experience.
///
/// # Arguments
///
/// * `stats` - Original stats
/// * `new_exp` - New experience value
///
/// # Returns
///
/// * `PlayerStats` - Stats with updated experience
fn create_stats_with_exp(stats: PlayerStats, new_exp: u32) -> PlayerStats {
    PlayerStats {
        health: stats.health,
        max_health: stats.max_health,
        attack: stats.attack,
        defense: stats.defense,
        experience: new_exp,
        level: stats.level,
    }
}

/// Check level up condition and apply if needed.
///
/// # Arguments
///
/// * `stats` - Stats to check
///
/// # Returns
///
/// * `PlayerStats` - Potentially leveled up stats
fn check_and_apply_level_up(stats: PlayerStats) -> PlayerStats {
    if should_level_up(stats.experience, stats.level) {
        apply_level_up(stats)
    } else {
        stats
    }
}

impl MovementGuest for Component {
    /// Move the player one tile in the specified direction.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The player's current position
    /// * `dir` - The direction to move
    ///
    /// # Returns
    ///
    /// * `Position` - The new position after movement
    fn move_player(current_pos: Position, dir: Direction) -> Position {
        let dx = delta_x(&dir);
        let dy = delta_y(&dir);
        Position {
            x: new_x(current_pos.x, dx),
            y: new_y(current_pos.y, dy),
        }
    }

    /// Calculate Manhattan distance between two positions.
    ///
    /// # Arguments
    ///
    /// * `start` - Starting position
    /// * `end` - Ending position
    ///
    /// # Returns
    ///
    /// * `u32` - Manhattan distance
    fn calculate_distance(start: Position, end: Position) -> u32 {
        let h_dist = horizontal_distance(start.x, end.x);
        let v_dist = vertical_distance(start.y, end.y);
        h_dist + v_dist
    }
}

impl StatsGuest for Component {
    /// Create a new player with default starting stats.
    ///
    /// # Returns
    ///
    /// * `PlayerStats` - New player stats
    fn create_player() -> PlayerStats {
        create_default_stats()
    }

    /// Apply damage to the player considering defense.
    ///
    /// # Arguments
    ///
    /// * `stats` - Current player stats
    /// * `raw_damage` - Incoming damage before defense
    ///
    /// # Returns
    ///
    /// * `PlayerStats` - Updated stats after damage
    fn take_damage(stats: PlayerStats, raw_damage: u32) -> PlayerStats {
        let effective = calculate_effective_damage(raw_damage, stats.defense);
        let new_health = apply_damage_to_health(stats.health, effective);
        PlayerStats {
            health: new_health,
            ..stats
        }
    }

    /// Heal the player by the specified amount.
    ///
    /// # Arguments
    ///
    /// * `stats` - Current player stats
    /// * `amount` - Amount to heal
    ///
    /// # Returns
    ///
    /// * `PlayerStats` - Updated stats after healing
    fn heal(stats: PlayerStats, amount: u32) -> PlayerStats {
        let new_health = calculate_healed_health(stats.health, amount, stats.max_health);
        PlayerStats {
            health: new_health,
            ..stats
        }
    }

    /// Add experience and handle potential level up.
    ///
    /// # Arguments
    ///
    /// * `stats` - Current player stats
    /// * `exp` - Experience to add
    ///
    /// # Returns
    ///
    /// * `PlayerStats` - Updated stats with experience
    fn gain_experience(stats: PlayerStats, exp: u32) -> PlayerStats {
        process_experience_gain(stats, exp)
    }

    /// Check if the player is defeated.
    ///
    /// # Arguments
    ///
    /// * `stats` - Current player stats
    ///
    /// # Returns
    ///
    /// * `bool` - True if health is zero
    fn is_defeated(stats: PlayerStats) -> bool {
        stats.health == 0
    }

    /// Get experience required for the next level.
    ///
    /// # Arguments
    ///
    /// * `current_level` - The player's current level
    ///
    /// # Returns
    ///
    /// * `u32` - Experience needed for next level
    fn exp_to_next_level(current_level: u32) -> u32 {
        calculate_exp_requirement(current_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that delta_x returns -1 for West direction.
    fn test_delta_x_west() {
        let result = delta_x(&Direction::West);
        assert_eq!(result, -1);
    }

    #[test]
    /// Test that delta_x returns 1 for East direction.
    fn test_delta_x_east() {
        let result = delta_x(&Direction::East);
        assert_eq!(result, 1);
    }

    #[test]
    /// Test that delta_x returns 0 for North direction.
    fn test_delta_x_north() {
        let result = delta_x(&Direction::North);
        assert_eq!(result, 0);
    }

    #[test]
    /// Test that delta_y returns -1 for North direction.
    fn test_delta_y_north() {
        let result = delta_y(&Direction::North);
        assert_eq!(result, -1);
    }

    #[test]
    /// Test that delta_y returns 1 for South direction.
    fn test_delta_y_south() {
        let result = delta_y(&Direction::South);
        assert_eq!(result, 1);
    }

    #[test]
    /// Test that delta_y returns 0 for East direction.
    fn test_delta_y_east() {
        let result = delta_y(&Direction::East);
        assert_eq!(result, 0);
    }

    #[test]
    /// Test new_x calculation.
    fn test_new_x() {
        let result = new_x(5, 2);
        assert_eq!(result, 7);
    }

    #[test]
    /// Test new_y calculation.
    fn test_new_y() {
        let result = new_y(3, -1);
        assert_eq!(result, 2);
    }

    #[test]
    /// Test horizontal distance calculation.
    fn test_horizontal_distance() {
        let result = horizontal_distance(10, 3);
        assert_eq!(result, 7);
    }

    #[test]
    /// Test vertical distance calculation.
    fn test_vertical_distance() {
        let result = vertical_distance(5, 12);
        assert_eq!(result, 7);
    }

    #[test]
    /// Test default stats creation.
    fn test_create_default_stats() {
        let stats = create_default_stats();
        assert_eq!(stats.health, STARTING_HEALTH);
        assert_eq!(stats.level, 1);
    }

    #[test]
    /// Test damage calculation with defense.
    fn test_calculate_effective_damage() {
        let result = calculate_effective_damage(20, 10);
        assert_eq!(result, 15);
    }

    #[test]
    /// Test minimum damage is 1.
    fn test_calculate_effective_damage_minimum() {
        let result = calculate_effective_damage(5, 100);
        assert_eq!(result, 1);
    }

    #[test]
    /// Test applying damage to health.
    fn test_apply_damage_to_health() {
        let result = apply_damage_to_health(100, 30);
        assert_eq!(result, 70);
    }

    #[test]
    /// Test healing calculation.
    fn test_calculate_healed_health() {
        let result = calculate_healed_health(50, 30, 100);
        assert_eq!(result, 80);
    }

    #[test]
    /// Test healing does not exceed max.
    fn test_calculate_healed_health_max() {
        let result = calculate_healed_health(90, 50, 100);
        assert_eq!(result, 100);
    }

    #[test]
    /// Test experience requirement calculation.
    fn test_calculate_exp_requirement() {
        let result = calculate_exp_requirement(1);
        assert_eq!(result, BASE_EXP_REQUIREMENT);
    }

    #[test]
    /// Test level up check returns true.
    fn test_should_level_up_true() {
        let result = should_level_up(150, 1);
        assert!(result);
    }

    #[test]
    /// Test level up check returns false.
    fn test_should_level_up_false() {
        let result = should_level_up(50, 1);
        assert!(!result);
    }

    #[test]
    /// Test max health increase on level up.
    fn test_level_up_max_health() {
        let result = level_up_max_health(100);
        assert_eq!(result, 100 + HEALTH_PER_LEVEL);
    }

    #[test]
    /// Test attack increase on level up.
    fn test_level_up_attack() {
        let result = level_up_attack(10);
        assert_eq!(result, 10 + ATTACK_PER_LEVEL);
    }

    #[test]
    /// Test defense increase on level up.
    fn test_level_up_defense() {
        let result = level_up_defense(5);
        assert_eq!(result, 5 + DEFENSE_PER_LEVEL);
    }

    #[test]
    /// Test moving north decreases Y.
    fn test_move_player_north() {
        let pos = Position { x: 5, y: 5 };
        let result = <Component as MovementGuest>::move_player(pos, Direction::North);
        assert_eq!(result.y, 4);
    }

    #[test]
    /// Test moving east increases X.
    fn test_move_player_east() {
        let pos = Position { x: 5, y: 5 };
        let result = <Component as MovementGuest>::move_player(pos, Direction::East);
        assert_eq!(result.x, 6);
    }

    #[test]
    /// Test Manhattan distance calculation.
    fn test_calculate_distance() {
        let start = Position { x: 0, y: 0 };
        let end = Position { x: 3, y: 4 };
        let result = <Component as MovementGuest>::calculate_distance(start, end);
        assert_eq!(result, 7);
    }

    #[test]
    /// Test create_player returns valid stats.
    fn test_create_player() {
        let stats = <Component as StatsGuest>::create_player();
        assert_eq!(stats.health, STARTING_HEALTH);
        assert_eq!(stats.attack, STARTING_ATTACK);
    }

    #[test]
    /// Test take_damage reduces health.
    fn test_take_damage() {
        let stats = create_default_stats();
        let result = <Component as StatsGuest>::take_damage(stats, 20);
        assert!(result.health < STARTING_HEALTH);
    }

    #[test]
    /// Test heal increases health.
    fn test_heal() {
        let mut stats = create_default_stats();
        stats.health = 50;
        let result = <Component as StatsGuest>::heal(stats, 30);
        assert_eq!(result.health, 80);
    }

    #[test]
    /// Test is_defeated with zero health.
    fn test_is_defeated_true() {
        let mut stats = create_default_stats();
        stats.health = 0;
        assert!(<Component as StatsGuest>::is_defeated(stats));
    }

    #[test]
    /// Test is_defeated with positive health.
    fn test_is_defeated_false() {
        let stats = create_default_stats();
        assert!(!<Component as StatsGuest>::is_defeated(stats));
    }

    #[test]
    /// Test exp_to_next_level calculation.
    fn test_exp_to_next_level() {
        let result = <Component as StatsGuest>::exp_to_next_level(1);
        assert_eq!(result, BASE_EXP_REQUIREMENT);
    }
}
