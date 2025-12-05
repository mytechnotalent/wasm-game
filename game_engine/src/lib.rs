//! # Game Engine Component for Legend of WASM
//!
//! This module implements the main game engine component for the Legend of WASM
//! game using the WebAssembly Component Model. It provides game initialization,
//! world management, and action processing for a Zelda-style action-adventure.
//!
//! ## Architecture
//!
//! The component implements the WIT interfaces defined in `wit/game_engine/world.wit`:
//! - `init`: Game state initialization and validation
//! - `engine`: Game action processing and status reporting
//! - `game-world`: World tile and area management
//!
//! ## Author
//!
//! Kevin Thomas <kevin@mytechnotalent.com>
//!
//! ## License
//!
//! MIT License

#[allow(warnings)]
mod bindings;

use bindings::exports::docs::game_engine::engine::Guest as EngineGuest;
use bindings::exports::docs::game_engine::game_world::Guest as WorldGuest;
use bindings::exports::docs::game_engine::init::Guest as InitGuest;
use bindings::exports::docs::game_engine::types::{
    ActionResult, GameAction, GamePhase, GameState, TileType,
};

/// Component struct for the game engine implementation.
///
/// This struct serves as the main entry point for the WebAssembly component,
/// implementing the init, engine, and game-world interfaces.
struct Component;

bindings::export!(Component with_types_in bindings);

// ============================================================================
// Game Initialization Functions
// ============================================================================

/// Create a new game state with default starting values.
///
/// Initializes the player at the center of Hyrule Field with full health
/// and ready to begin their adventure.
///
/// # Returns
///
/// A fresh `GameState` with default starting values.
fn new_game_impl() -> GameState {
    GameState {
        phase: GamePhase::Exploration,
        player_x: 50,
        player_y: 50,
        player_health: 100,
        player_max_health: 100,
        player_attack: 10,
        player_defense: 5,
        player_level: 1,
        player_exp: 0,
        enemies_defeated: 0,
        boss_defeated: false,
        current_area: "Hyrule Field".to_string(),
        turn_number: 1,
    }
}

/// Validate a game state for consistency.
///
/// Checks that health values are within bounds and that the game state
/// is internally consistent.
///
/// # Arguments
///
/// * `state` - The game state to validate
///
/// # Returns
///
/// `true` if the state is valid, `false` otherwise.
fn validate_state_impl(state: &GameState) -> bool {
    let health_valid = state.player_health <= state.player_max_health;
    let position_valid = is_in_bounds(state.player_x, state.player_y);
    health_valid && position_valid
}

/// Check if coordinates are within world bounds.
///
/// # Arguments
///
/// * `x` - X coordinate to check
/// * `y` - Y coordinate to check
///
/// # Returns
///
/// `true` if the position is within the 100x100 world grid.
fn is_in_bounds(x: i32, y: i32) -> bool {
    (0..100).contains(&x) && (0..100).contains(&y)
}

// ============================================================================
// Action Processing Functions
// ============================================================================
/// Clamp a value to the world bounds.
///
/// # Arguments
///
/// * `val` - The value to clamp
///
/// # Returns
///
/// The value clamped to 0-99 range.
fn clamp_coord(val: i32) -> i32 {
    val.clamp(0, 99)
}

/// Create a successful action result.
///
/// # Arguments
///
/// * `msg` - The message describing the action
/// * `phase` - The new game phase
///
/// # Returns
///
/// An `ActionResult` indicating success.
fn success_result(msg: &str, phase: GamePhase) -> ActionResult {
    ActionResult {
        success: true,
        message: msg.to_string(),
        new_phase: phase,
        game_continues: true,
    }
}

/// Create a game over action result.
///
/// # Arguments
///
/// * `msg` - The message describing the game over
///
/// # Returns
///
/// An `ActionResult` indicating game over.
fn game_over_result(msg: &str) -> ActionResult {
    ActionResult {
        success: true,
        message: msg.to_string(),
        new_phase: GamePhase::GameOver,
        game_continues: false,
    }
}

/// Process a movement action.
///
/// # Arguments
///
/// * `state` - Current game state
/// * `dx` - X direction delta
/// * `dy` - Y direction delta
/// * `dir_name` - Name of direction for message
///
/// # Returns
///
/// An `ActionResult` with the movement outcome.
fn process_move(state: &GameState, dx: i32, dy: i32, dir_name: &str) -> ActionResult {
    let _new_x = clamp_coord(state.player_x + dx);
    let _new_y = clamp_coord(state.player_y + dy);
    let msg = format!("You move {}.", dir_name);
    success_result(&msg, GamePhase::Exploration)
}

/// Process an attack action.
///
/// # Arguments
///
/// * `_state` - Current game state (unused but available)
///
/// # Returns
///
/// An `ActionResult` with the attack outcome.
fn process_attack(_state: &GameState) -> ActionResult {
    success_result("You swing your sword!", GamePhase::Combat)
}

/// Process an item use action.
///
/// # Arguments
///
/// * `_state` - Current game state (unused but available)
///
/// # Returns
///
/// An `ActionResult` with the item use outcome.
fn process_use_item(_state: &GameState) -> ActionResult {
    success_result("You use an item.", GamePhase::Exploration)
}

/// Process an interact action.
///
/// # Arguments
///
/// * `_state` - Current game state (unused but available)
///
/// # Returns
///
/// An `ActionResult` with the interaction outcome.
fn process_interact(_state: &GameState) -> ActionResult {
    success_result("You interact with the environment.", GamePhase::Dialogue)
}

/// Process an open inventory action.
///
/// # Returns
///
/// An `ActionResult` opening the inventory.
fn process_inventory() -> ActionResult {
    success_result("Opening inventory...", GamePhase::Inventory)
}

/// Process a wait action.
///
/// # Returns
///
/// An `ActionResult` for waiting.
fn process_wait() -> ActionResult {
    success_result("You wait...", GamePhase::Exploration)
}

/// Process a quit action.
///
/// # Returns
///
/// An `ActionResult` ending the game.
fn process_quit() -> ActionResult {
    game_over_result("Thanks for playing!")
}

/// Process a game action and return the result.
///
/// Dispatches the action to the appropriate handler based on action type.
///
/// # Arguments
///
/// * `state` - The current game state
/// * `action` - The action to process
///
/// # Returns
///
/// An `ActionResult` describing the outcome.
fn process_action_impl(state: &GameState, action: &GameAction) -> ActionResult {
    match action {
        GameAction::MoveNorth => process_move(state, 0, -1, "north"),
        GameAction::MoveSouth => process_move(state, 0, 1, "south"),
        GameAction::MoveEast => process_move(state, 1, 0, "east"),
        GameAction::MoveWest => process_move(state, -1, 0, "west"),
        GameAction::Attack => process_attack(state),
        GameAction::UseItem => process_use_item(state),
        GameAction::OpenInventory => process_inventory(),
        GameAction::Interact => process_interact(state),
        GameAction::Wait => process_wait(),
        GameAction::Quit => process_quit(),
    }
}

/// Get the current game status as a formatted string.
///
/// # Arguments
///
/// * `state` - The current game state
///
/// # Returns
///
/// A formatted status string showing health, level, and location.
fn get_status_impl(state: &GameState) -> String {
    format!(
        "HP: {}/{} | Lvl: {} | Area: {} | Turn: {}",
        state.player_health,
        state.player_max_health,
        state.player_level,
        state.current_area,
        state.turn_number
    )
}

/// Check for random encounters after movement.
///
/// Uses position-based pseudo-random to determine encounters.
///
/// # Arguments
///
/// * `state` - The current game state
///
/// # Returns
///
/// `true` if an encounter occurs, `false` otherwise.
fn check_encounter_impl(state: &GameState) -> bool {
    let hash = (state.player_x * 31 + state.player_y * 17) % 10;
    hash < 2
}

/// Get help text for available actions.
///
/// # Returns
///
/// A string containing help information for all commands.
fn get_help_impl() -> String {
    let lines = [
        "=== LEGEND OF WASM: HELP ===",
        "Movement: n/s/e/w - Move in direction",
        "Combat: a - Attack with equipped weapon",
        "Items: u - Use item, i - Open inventory",
        "Other: x - Interact, . - Wait, q - Quit",
    ];
    lines.join("\n")
}

// ============================================================================
// World Management Functions
// ============================================================================

/// Check if position is in water region.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if the position is water.
fn is_water(x: i32, y: i32) -> bool {
    (20..30).contains(&x) && (40..60).contains(&y)
}

/// Check if position is in forest region.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if the position is forest.
fn is_forest(x: i32, y: i32) -> bool {
    (60..80).contains(&x) && (10..30).contains(&y)
}

/// Check if position is a dungeon entrance.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if the position is a dungeon entrance.
fn is_dungeon(x: i32, y: i32) -> bool {
    (x == 75 && y == 75) || (x == 25 && y == 25)
}

/// Check if position is a wall.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if the position is a wall.
fn is_wall(x: i32, y: i32) -> bool {
    x == 0 || x == 99 || y == 0 || y == 99
}

/// Get the tile type at a position.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// The `TileType` at the given position.
fn get_tile_impl(x: i32, y: i32) -> TileType {
    if is_wall(x, y) {
        TileType::Wall
    } else if is_water(x, y) {
        TileType::Water
    } else if is_forest(x, y) {
        TileType::Forest
    } else if is_dungeon(x, y) {
        TileType::DungeonEntrance
    } else {
        TileType::Grass
    }
}

/// Check if a position is walkable.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if the player can walk on this tile.
fn is_walkable_impl(x: i32, y: i32) -> bool {
    let tile = get_tile_impl(x, y);
    !matches!(tile, TileType::Wall | TileType::Water)
}

/// Get the name of an area based on position.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// The name of the area at this position.
fn get_area_name_impl(x: i32, y: i32) -> String {
    let area_index = calc_area_index(x, y);
    area_name_by_index(area_index).to_string()
}

/// Calculate area index from position.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// An area index from 0-15.
fn calc_area_index(x: i32, y: i32) -> u32 {
    let ax = (x / 25) as u32;
    let ay = (y / 25) as u32;
    (ay * 4 + ax).min(15)
}

/// Get area name by index.
///
/// # Arguments
///
/// * `index` - Area index (0-15)
///
/// # Returns
///
/// The name of the area.
fn area_name_by_index(index: u32) -> &'static str {
    match index {
        0 => "Hyrule Field NW",
        1 => "Hyrule Castle",
        2 => "Kakariko Village",
        3 => "Death Mountain",
        4 => "Lake Hylia West",
        5 => "Lake Hylia",
        6 => "Zora's Domain",
        7 => "Goron City",
        8 => "Lost Woods West",
        9 => "Lost Woods",
        10 => "Sacred Grove",
        11 => "Temple of Time",
        12 => "Gerudo Desert",
        13 => "Gerudo Fortress",
        14 => "Spirit Temple",
        15 => "Ganon's Tower",
        _ => "Unknown",
    }
}

/// Check if a position has a special event.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
///
/// # Returns
///
/// `true` if this position triggers an event.
fn has_event_impl(x: i32, y: i32) -> bool {
    is_dungeon(x, y) || (x == 50 && y == 50)
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl InitGuest for Component {
    /// Create a new game with default starting state.
    ///
    /// # Returns
    ///
    /// A fresh `GameState` ready for a new adventure.
    fn new_game() -> GameState {
        new_game_impl()
    }

    /// Validate a game state for consistency.
    ///
    /// # Arguments
    ///
    /// * `state` - The game state to validate
    ///
    /// # Returns
    ///
    /// `true` if the state is valid.
    fn validate_state(state: GameState) -> bool {
        validate_state_impl(&state)
    }
}

impl EngineGuest for Component {
    /// Process a player action and return the result.
    ///
    /// # Arguments
    ///
    /// * `state` - The current game state
    /// * `action` - The action to process
    ///
    /// # Returns
    ///
    /// An `ActionResult` describing what happened.
    fn process_action(state: GameState, action: GameAction) -> ActionResult {
        process_action_impl(&state, &action)
    }

    /// Get the current game state as a formatted string.
    ///
    /// # Arguments
    ///
    /// * `state` - The current game state
    ///
    /// # Returns
    ///
    /// A formatted status string.
    fn get_status(state: GameState) -> String {
        get_status_impl(&state)
    }

    /// Check for enemy encounters after movement.
    ///
    /// # Arguments
    ///
    /// * `state` - The current game state
    ///
    /// # Returns
    ///
    /// `true` if an encounter should trigger.
    fn check_encounter(state: GameState) -> bool {
        check_encounter_impl(&state)
    }

    /// Get help text for available actions.
    ///
    /// # Returns
    ///
    /// A help string listing all commands.
    fn get_help() -> String {
        get_help_impl()
    }
}

impl WorldGuest for Component {
    /// Get the tile type at a position.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// The `TileType` at this position.
    fn get_tile(x: i32, y: i32) -> TileType {
        get_tile_impl(x, y)
    }

    /// Check if a position is walkable.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// `true` if the player can walk here.
    fn is_walkable(x: i32, y: i32) -> bool {
        is_walkable_impl(x, y)
    }

    /// Get the name of the current area.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// The area name at this position.
    fn get_area_name(x: i32, y: i32) -> String {
        get_area_name_impl(x, y)
    }

    /// Check if position has a special event.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// `true` if an event triggers here.
    fn has_event(x: i32, y: i32) -> bool {
        has_event_impl(x, y)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that new_game creates a valid initial state.
    ///
    /// Verifies that all starting values are set correctly for a new player
    /// beginning their adventure in Hyrule Field.
    #[test]
    fn test_new_game() {
        let state = new_game_impl();
        assert_eq!(state.player_x, 50);
        assert_eq!(state.player_y, 50);
        assert_eq!(state.player_health, 100);
        assert_eq!(state.player_level, 1);
    }

    /// Test that new_game sets correct phase.
    ///
    /// Verifies that the initial game phase is Exploration.
    #[test]
    fn test_new_game_phase() {
        let state = new_game_impl();
        assert!(matches!(state.phase, GamePhase::Exploration));
    }

    /// Test that new_game sets boss_defeated to false.
    ///
    /// Verifies that the boss has not been defeated at game start.
    #[test]
    fn test_new_game_boss_not_defeated() {
        let state = new_game_impl();
        assert!(!state.boss_defeated);
    }

    /// Test validation of a valid game state.
    ///
    /// Verifies that a freshly created game state passes validation.
    #[test]
    fn test_validate_state_valid() {
        let state = new_game_impl();
        assert!(validate_state_impl(&state));
    }

    /// Test validation fails for invalid health.
    ///
    /// Verifies that a state with health exceeding max fails validation.
    #[test]
    fn test_validate_state_invalid_health() {
        let mut state = new_game_impl();
        state.player_health = 150;
        assert!(!validate_state_impl(&state));
    }

    /// Test validation fails for out-of-bounds position.
    ///
    /// Verifies that a state with invalid coordinates fails validation.
    #[test]
    fn test_validate_state_invalid_position() {
        let mut state = new_game_impl();
        state.player_x = 150;
        assert!(!validate_state_impl(&state));
    }

    /// Test is_in_bounds with valid coordinates.
    ///
    /// Verifies that center coordinates are considered in bounds.
    #[test]
    fn test_is_in_bounds_valid() {
        assert!(is_in_bounds(50, 50));
        assert!(is_in_bounds(0, 0));
        assert!(is_in_bounds(99, 99));
    }

    /// Test is_in_bounds with invalid coordinates.
    ///
    /// Verifies that out-of-range coordinates are rejected.
    #[test]
    fn test_is_in_bounds_invalid() {
        assert!(!is_in_bounds(-1, 50));
        assert!(!is_in_bounds(50, 100));
        assert!(!is_in_bounds(100, 100));
    }

    /// Test process_action for move north.
    ///
    /// Verifies that moving north produces the correct result.
    #[test]
    fn test_process_action_move_north() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::MoveNorth);
        assert!(result.success);
        assert!(result.message.contains("north"));
    }

    /// Test process_action for move south.
    ///
    /// Verifies that moving south produces the correct result.
    #[test]
    fn test_process_action_move_south() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::MoveSouth);
        assert!(result.success);
        assert!(result.message.contains("south"));
    }

    /// Test process_action for move east.
    ///
    /// Verifies that moving east produces the correct result.
    #[test]
    fn test_process_action_move_east() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::MoveEast);
        assert!(result.message.contains("east"));
    }

    /// Test process_action for move west.
    ///
    /// Verifies that moving west produces the correct result.
    #[test]
    fn test_process_action_move_west() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::MoveWest);
        assert!(result.message.contains("west"));
    }

    /// Test process_action for attack.
    ///
    /// Verifies that attack sets phase to Combat.
    #[test]
    fn test_process_action_attack() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::Attack);
        assert!(matches!(result.new_phase, GamePhase::Combat));
    }

    /// Test process_action for interact.
    ///
    /// Verifies that interact sets phase to Dialogue.
    #[test]
    fn test_process_action_interact() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::Interact);
        assert!(matches!(result.new_phase, GamePhase::Dialogue));
    }

    /// Test process_action for quit.
    ///
    /// Verifies that quit ends the game.
    #[test]
    fn test_process_action_quit() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::Quit);
        assert!(!result.game_continues);
        assert!(matches!(result.new_phase, GamePhase::GameOver));
    }

    /// Test process_action for open inventory.
    ///
    /// Verifies that opening inventory sets correct phase.
    #[test]
    fn test_process_action_inventory() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::OpenInventory);
        assert!(matches!(result.new_phase, GamePhase::Inventory));
    }

    /// Test process_action for wait.
    ///
    /// Verifies that waiting keeps exploration phase.
    #[test]
    fn test_process_action_wait() {
        let state = new_game_impl();
        let result = process_action_impl(&state, &GameAction::Wait);
        assert!(matches!(result.new_phase, GamePhase::Exploration));
    }

    /// Test get_status produces formatted output.
    ///
    /// Verifies that status string contains expected elements.
    #[test]
    fn test_get_status() {
        let state = new_game_impl();
        let status = get_status_impl(&state);
        assert!(status.contains("HP:"));
        assert!(status.contains("Lvl:"));
        assert!(status.contains("Area:"));
    }

    /// Test get_help produces help text.
    ///
    /// Verifies that help text contains command information.
    #[test]
    fn test_get_help() {
        let help = get_help_impl();
        assert!(help.contains("Movement"));
        assert!(help.contains("Combat"));
    }

    /// Test check_encounter returns boolean.
    ///
    /// Verifies that encounter check works without crashing.
    #[test]
    fn test_check_encounter() {
        let state = new_game_impl();
        let _result = check_encounter_impl(&state);
        // Just verify it doesn't panic
    }

    /// Test get_tile returns grass for center.
    ///
    /// Verifies that the center of the map is grass.
    #[test]
    fn test_get_tile_grass() {
        let tile = get_tile_impl(50, 50);
        assert!(matches!(tile, TileType::Grass));
    }

    /// Test get_tile returns water in water region.
    ///
    /// Verifies that the water region returns water tiles.
    #[test]
    fn test_get_tile_water() {
        let tile = get_tile_impl(25, 50);
        assert!(matches!(tile, TileType::Water));
    }

    /// Test get_tile returns forest in forest region.
    ///
    /// Verifies that the forest region returns forest tiles.
    #[test]
    fn test_get_tile_forest() {
        let tile = get_tile_impl(70, 20);
        assert!(matches!(tile, TileType::Forest));
    }

    /// Test get_tile returns wall at edge.
    ///
    /// Verifies that the map edges are walls.
    #[test]
    fn test_get_tile_wall() {
        let tile = get_tile_impl(0, 50);
        assert!(matches!(tile, TileType::Wall));
    }

    /// Test get_tile returns dungeon entrance.
    ///
    /// Verifies that dungeon positions return entrance tiles.
    #[test]
    fn test_get_tile_dungeon() {
        let tile = get_tile_impl(75, 75);
        assert!(matches!(tile, TileType::DungeonEntrance));
    }

    /// Test is_walkable for grass.
    ///
    /// Verifies that grass tiles are walkable.
    #[test]
    fn test_is_walkable_grass() {
        assert!(is_walkable_impl(50, 50));
    }

    /// Test is_walkable for forest.
    ///
    /// Verifies that forest tiles are walkable.
    #[test]
    fn test_is_walkable_forest() {
        assert!(is_walkable_impl(70, 20));
    }

    /// Test is_walkable for wall.
    ///
    /// Verifies that wall tiles are not walkable.
    #[test]
    fn test_is_walkable_wall() {
        assert!(!is_walkable_impl(0, 0));
    }

    /// Test is_walkable for water.
    ///
    /// Verifies that water tiles are not walkable.
    #[test]
    fn test_is_walkable_water() {
        assert!(!is_walkable_impl(25, 50));
    }

    /// Test get_area_name for northwest.
    ///
    /// Verifies that the northwest area is named correctly.
    #[test]
    fn test_get_area_name_nw() {
        let name = get_area_name_impl(10, 10);
        assert_eq!(name, "Hyrule Field NW");
    }

    /// Test get_area_name for southeast.
    ///
    /// Verifies that the southeast corner returns correct area.
    #[test]
    fn test_get_area_name_se() {
        let name = get_area_name_impl(90, 90);
        assert_eq!(name, "Ganon's Tower");
    }

    /// Test calc_area_index bounds.
    ///
    /// Verifies that area index calculation stays within bounds.
    #[test]
    fn test_calc_area_index() {
        assert_eq!(calc_area_index(0, 0), 0);
        assert_eq!(calc_area_index(99, 99), 15);
    }

    /// Test area_name_by_index for all areas.
    ///
    /// Verifies that known area indices return correct names.
    #[test]
    fn test_area_name_by_index() {
        assert_eq!(area_name_by_index(0), "Hyrule Field NW");
        assert_eq!(area_name_by_index(1), "Hyrule Castle");
        assert_eq!(area_name_by_index(15), "Ganon's Tower");
        assert_eq!(area_name_by_index(99), "Unknown");
    }

    /// Test has_event at dungeon entrance.
    ///
    /// Verifies that dungeon entrances trigger events.
    #[test]
    fn test_has_event_dungeon() {
        assert!(has_event_impl(75, 75));
        assert!(has_event_impl(25, 25));
    }

    /// Test has_event at center.
    ///
    /// Verifies that the center position triggers an event.
    #[test]
    fn test_has_event_center() {
        assert!(has_event_impl(50, 50));
    }

    /// Test has_event at random position.
    ///
    /// Verifies that most positions don't trigger events.
    #[test]
    fn test_has_event_none() {
        assert!(!has_event_impl(60, 60));
    }

    /// Test clamp_coord for in-range values.
    ///
    /// Verifies that values within range are unchanged.
    #[test]
    fn test_clamp_coord_in_range() {
        assert_eq!(clamp_coord(50), 50);
        assert_eq!(clamp_coord(0), 0);
        assert_eq!(clamp_coord(99), 99);
    }

    /// Test clamp_coord for out-of-range values.
    ///
    /// Verifies that values outside range are clamped.
    #[test]
    fn test_clamp_coord_out_of_range() {
        assert_eq!(clamp_coord(-10), 0);
        assert_eq!(clamp_coord(150), 99);
    }

    /// Test success_result helper.
    ///
    /// Verifies that success results are constructed correctly.
    #[test]
    fn test_success_result() {
        let result = success_result("Test", GamePhase::Exploration);
        assert!(result.success);
        assert!(result.game_continues);
        assert_eq!(result.message, "Test");
    }

    /// Test game_over_result helper.
    ///
    /// Verifies that game over results are constructed correctly.
    #[test]
    fn test_game_over_result() {
        let result = game_over_result("Game Over");
        assert!(result.success);
        assert!(!result.game_continues);
        assert!(matches!(result.new_phase, GamePhase::GameOver));
    }
}
