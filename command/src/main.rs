//! # Command Component for Legend of WASM
//!
//! This module implements the command-line interface for the Legend of WASM
//! game. It provides user input parsing, game display, and the main game loop
//! that ties together all game components.
//!
//! ## Architecture
//!
//! This is the main entry point that composes all game components:
//! - Player movement and statistics
//! - Enemy spawning and AI behavior
//! - Combat system and damage calculation
//! - Inventory management and item usage
//! - Game engine state coordination
//!
//! ## Author
//!
//! Kevin Thomas <kevin@mytechnotalent.com>
//!
//! ## License
//!
//! MIT License

use std::io::{self, Write};

/// Map dimensions for the game world.
const MAP_WIDTH: i32 = 20;
/// Map height for the game world.
const MAP_HEIGHT: i32 = 15;

/// Represents a user input command.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Move in a cardinal direction.
    Move(Direction),
    /// Attack with the equipped weapon.
    Attack,
    /// Interact with the environment or NPC.
    Interact,
    /// Use an item from inventory.
    UseItem,
    /// Open the inventory menu.
    Inventory,
    /// Display player status.
    Status,
    /// Display help information.
    Help,
    /// Quit the game.
    Quit,
    /// Wait/skip a turn.
    Wait,
    /// Unknown or invalid command.
    Unknown,
}

/// Movement direction enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    /// Move upward (north).
    North,
    /// Move downward (south).
    South,
    /// Move rightward (east).
    East,
    /// Move leftward (west).
    West,
}

/// Enemy type in the game.
#[derive(Debug, Clone, PartialEq)]
pub enum EnemyKind {
    /// Weak enemy, easy to defeat.
    Slime,
    /// Medium enemy with bones.
    Skeleton,
    /// Fast flying enemy.
    Bat,
    /// Strong melee enemy.
    Goblin,
    /// Powerful elite enemy.
    DarkKnight,
    /// Boss enemy.
    Boss,
}

/// An enemy in the game world.
#[derive(Debug, Clone)]
pub struct Enemy {
    /// Enemy type.
    pub kind: EnemyKind,
    /// X position on map.
    pub x: i32,
    /// Y position on map.
    pub y: i32,
    /// Current health points.
    pub health: i32,
    /// Attack power.
    pub attack: i32,
    /// Experience reward.
    pub exp: i32,
}

/// A collectible item in the game world.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemKind {
    /// Health potion.
    Potion,
    /// Gold coins.
    Gold,
    /// Treasure chest.
    Chest,
    /// Sword upgrade.
    Sword,
}

/// An item on the map.
#[derive(Debug, Clone)]
pub struct Item {
    /// Item type.
    pub kind: ItemKind,
    /// X position on map.
    pub x: i32,
    /// Y position on map.
    pub y: i32,
}

/// Tile type for terrain.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Tile {
    /// Open grass tile.
    Grass,
    /// Tree/forest tile.
    Tree,
    /// Water tile (impassable).
    Water,
    /// Mountain tile (impassable).
    Mountain,
    /// Wall tile (impassable).
    Wall,
}

/// Parse input for a movement command.
fn parse_move(input: &str) -> Option<Command> {
    match input {
        "n" | "north" => Some(Command::Move(Direction::North)),
        "s" | "south" => Some(Command::Move(Direction::South)),
        "e" | "east" => Some(Command::Move(Direction::East)),
        "w" | "west" => Some(Command::Move(Direction::West)),
        _ => None,
    }
}

/// Parse input for an action command.
fn parse_action(input: &str) -> Option<Command> {
    match input {
        "a" | "attack" => Some(Command::Attack),
        "x" | "interact" => Some(Command::Interact),
        "u" | "use" => Some(Command::UseItem),
        "." | "wait" => Some(Command::Wait),
        _ => None,
    }
}

/// Parse input for a system command.
fn parse_system(input: &str) -> Option<Command> {
    match input {
        "i" | "inv" | "inventory" => Some(Command::Inventory),
        "stat" | "status" => Some(Command::Status),
        "h" | "help" | "?" => Some(Command::Help),
        "q" | "quit" | "exit" => Some(Command::Quit),
        _ => None,
    }
}

/// Parse user input into a command.
pub fn parse_input(input: &str) -> Command {
    let input = input.trim().to_lowercase();
    parse_move(&input)
        .or_else(|| parse_action(&input))
        .or_else(|| parse_system(&input))
        .unwrap_or(Command::Unknown)
}

/// Get symbol for enemy kind.
fn enemy_symbol(kind: &EnemyKind) -> char {
    match kind {
        EnemyKind::Slime => 's',
        EnemyKind::Skeleton => 'k',
        EnemyKind::Bat => 'b',
        EnemyKind::Goblin => 'g',
        EnemyKind::DarkKnight => 'D',
        EnemyKind::Boss => 'B',
    }
}

/// Get symbol for item kind.
fn item_symbol(kind: &ItemKind) -> char {
    match kind {
        ItemKind::Potion => '*',
        ItemKind::Gold => '$',
        ItemKind::Chest => 'C',
        ItemKind::Sword => '+',
    }
}

/// Get symbol for tile.
fn tile_symbol(tile: &Tile) -> char {
    match tile {
        Tile::Grass => '.',
        Tile::Tree => 'T',
        Tile::Water => '~',
        Tile::Mountain => '^',
        Tile::Wall => '#',
    }
}

/// Create a new slime enemy.
fn create_slime(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::Slime,
        x,
        y,
        health: 10,
        attack: 3,
        exp: 5,
    }
}

/// Create a new skeleton enemy.
fn create_skeleton(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::Skeleton,
        x,
        y,
        health: 20,
        attack: 5,
        exp: 10,
    }
}

/// Create a new bat enemy.
fn create_bat(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::Bat,
        x,
        y,
        health: 8,
        attack: 4,
        exp: 7,
    }
}

/// Create a new goblin enemy.
fn create_goblin(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::Goblin,
        x,
        y,
        health: 25,
        attack: 8,
        exp: 15,
    }
}

/// Create a new dark knight enemy.
fn create_dark_knight(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::DarkKnight,
        x,
        y,
        health: 40,
        attack: 12,
        exp: 25,
    }
}

/// Create a new boss enemy.
fn create_boss(x: i32, y: i32) -> Enemy {
    Enemy {
        kind: EnemyKind::Boss,
        x,
        y,
        health: 100,
        attack: 20,
        exp: 100,
    }
}

/// Simple game state for the game.
pub struct SimpleGameState {
    /// Player X position.
    pub player_x: i32,
    /// Player Y position.
    pub player_y: i32,
    /// Current health points.
    pub health: i32,
    /// Maximum health points.
    pub max_health: i32,
    /// Current score.
    pub score: i32,
    /// Player attack power.
    pub attack: i32,
    /// Player defense.
    pub defense: i32,
    /// Player experience points.
    pub exp: i32,
    /// Player level.
    pub level: i32,
    /// Gold coins.
    pub gold: i32,
    /// Number of potions.
    pub potions: i32,
    /// Whether the game is running.
    pub is_running: bool,
    /// Enemies on the map.
    pub enemies: Vec<Enemy>,
    /// Items on the map.
    pub items: Vec<Item>,
    /// Terrain tiles.
    pub terrain: Vec<Vec<Tile>>,
    /// Turn counter.
    pub turn: i32,
    /// Message to display.
    pub message: String,
}

/// Initialize terrain grid with grass.
fn init_terrain() -> Vec<Vec<Tile>> {
    vec![vec![Tile::Grass; MAP_WIDTH as usize]; MAP_HEIGHT as usize]
}

/// Place trees on the terrain.
fn place_trees(terrain: &mut [Vec<Tile>]) {
    let trees = [
        (3, 2),
        (4, 2),
        (5, 7),
        (6, 7),
        (7, 7),
        (15, 3),
        (16, 3),
        (17, 3),
        (2, 10),
        (3, 10),
        (12, 12),
        (13, 12),
        (14, 12),
        (8, 4),
        (9, 4),
    ];
    for (x, y) in trees {
        terrain[y][x] = Tile::Tree;
    }
}

/// Place water on the terrain.
fn place_water(terrain: &mut [Vec<Tile>]) {
    let waters = [
        (10, 6),
        (11, 6),
        (12, 6),
        (10, 7),
        (11, 7),
        (12, 7),
        (10, 8),
        (11, 8),
        (12, 8),
    ];
    for (x, y) in waters {
        terrain[y][x] = Tile::Water;
    }
}

/// Place mountains on the terrain.
fn place_mountains(terrain: &mut [Vec<Tile>]) {
    let mountains = [
        (0, 0),
        (1, 0),
        (2, 0),
        (18, 0),
        (19, 0),
        (0, 14),
        (1, 14),
        (18, 14),
        (19, 14),
    ];
    for (x, y) in mountains {
        terrain[y][x] = Tile::Mountain;
    }
}

/// Place walls on the terrain.
fn place_walls(terrain: &mut [Vec<Tile>]) {
    let walls = [
        (5, 10),
        (6, 10),
        (7, 10),
        (5, 11),
        (7, 11),
        (5, 12),
        (6, 12),
        (7, 12),
    ];
    for (x, y) in walls {
        terrain[y][x] = Tile::Wall;
    }
}

/// Generate the game terrain.
fn generate_terrain() -> Vec<Vec<Tile>> {
    let mut terrain = init_terrain();
    place_trees(&mut terrain);
    place_water(&mut terrain);
    place_mountains(&mut terrain);
    place_walls(&mut terrain);
    terrain
}

/// Spawn initial enemies.
fn spawn_enemies() -> Vec<Enemy> {
    vec![
        create_slime(5, 3),
        create_slime(14, 5),
        create_bat(8, 9),
        create_skeleton(16, 10),
        create_goblin(3, 12),
        create_dark_knight(17, 13),
        create_boss(10, 2),
    ]
}

/// Spawn initial items.
fn spawn_items() -> Vec<Item> {
    vec![
        Item {
            kind: ItemKind::Potion,
            x: 2,
            y: 5,
        },
        Item {
            kind: ItemKind::Potion,
            x: 15,
            y: 8,
        },
        Item {
            kind: ItemKind::Gold,
            x: 8,
            y: 3,
        },
        Item {
            kind: ItemKind::Gold,
            x: 12,
            y: 11,
        },
        Item {
            kind: ItemKind::Chest,
            x: 6,
            y: 11,
        },
        Item {
            kind: ItemKind::Sword,
            x: 18,
            y: 7,
        },
    ]
}

impl SimpleGameState {
    /// Create a new game state with default values.
    pub fn new() -> Self {
        SimpleGameState {
            player_x: 10,
            player_y: 10,
            health: 100,
            max_health: 100,
            score: 0,
            attack: 15,
            defense: 5,
            exp: 0,
            level: 1,
            gold: 0,
            potions: 1,
            is_running: true,
            enemies: spawn_enemies(),
            items: spawn_items(),
            terrain: generate_terrain(),
            turn: 0,
            message: String::new(),
        }
    }

    /// Get the current area name based on position.
    pub fn area_name(&self) -> &'static str {
        area_from_position(self.player_x, self.player_y)
    }

    /// Set a message to display.
    pub fn set_message(&mut self, msg: &str) {
        self.message = msg.to_string();
    }

    /// Clear the message.
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
}

impl Default for SimpleGameState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get area name from position coordinates.
fn area_from_position(x: i32, y: i32) -> &'static str {
    let ax = x / 7;
    let ay = y / 5;
    let area = (ay * 3 + ax) as u32;
    area_name(area)
}

/// Get area name by index.
fn area_name(area: u32) -> &'static str {
    match area {
        0 => "Hyrule Field NW",
        1 => "Hyrule Castle",
        2 => "Kakariko Village",
        3 => "Lost Woods",
        4 => "Lake Hylia",
        5 => "Death Mountain",
        6 => "Zora's Domain",
        7 => "Gerudo Valley",
        8 => "Temple of Time",
        _ => "Unknown Lands",
    }
}

/// Check if a position is walkable.
fn is_walkable(terrain: &[Vec<Tile>], x: i32, y: i32) -> bool {
    if !(0..MAP_WIDTH).contains(&x) || !(0..MAP_HEIGHT).contains(&y) {
        return false;
    }
    let tile = terrain[y as usize][x as usize];
    matches!(tile, Tile::Grass)
}

/// Apply north movement to game state.
fn apply_north(state: &mut SimpleGameState) {
    let new_y = state.player_y - 1;
    if is_walkable(&state.terrain, state.player_x, new_y) {
        state.player_y = new_y;
    } else {
        state.set_message("You can't go that way!");
    }
}

/// Apply south movement to game state.
fn apply_south(state: &mut SimpleGameState) {
    let new_y = state.player_y + 1;
    if is_walkable(&state.terrain, state.player_x, new_y) {
        state.player_y = new_y;
    } else {
        state.set_message("You can't go that way!");
    }
}

/// Apply east movement to game state.
fn apply_east(state: &mut SimpleGameState) {
    let new_x = state.player_x + 1;
    if is_walkable(&state.terrain, new_x, state.player_y) {
        state.player_x = new_x;
    } else {
        state.set_message("You can't go that way!");
    }
}

/// Apply west movement to game state.
fn apply_west(state: &mut SimpleGameState) {
    let new_x = state.player_x - 1;
    if is_walkable(&state.terrain, new_x, state.player_y) {
        state.player_x = new_x;
    } else {
        state.set_message("You can't go that way!");
    }
}

/// Apply a movement command to the game state.
fn apply_move(state: &mut SimpleGameState, dir: &Direction) {
    match dir {
        Direction::North => apply_north(state),
        Direction::South => apply_south(state),
        Direction::East => apply_east(state),
        Direction::West => apply_west(state),
    }
}

/// Find enemy at position.
fn find_enemy_at(enemies: &[Enemy], x: i32, y: i32) -> Option<usize> {
    enemies.iter().position(|e| e.x == x && e.y == y)
}

/// Find adjacent enemy.
fn find_adjacent_enemy(state: &SimpleGameState) -> Option<usize> {
    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (dx, dy) in dirs {
        let x = state.player_x + dx;
        let y = state.player_y + dy;
        if let Some(idx) = find_enemy_at(&state.enemies, x, y) {
            return Some(idx);
        }
    }
    None
}

/// Calculate damage dealt to enemy.
fn calc_damage(attack: i32) -> i32 {
    attack + (attack / 4)
}

/// Process enemy defeat.
fn defeat_enemy(state: &mut SimpleGameState, idx: usize) {
    let enemy = state.enemies.remove(idx);
    let name = enemy_kind_name(&enemy.kind);
    state.exp += enemy.exp;
    state.score += enemy.exp * 10;
    state.set_message(&format!(
        "You defeated the {}! +{} EXP, +{} score",
        name,
        enemy.exp,
        enemy.exp * 10
    ));
    check_level_up(state);
}

/// Get enemy kind name.
fn enemy_kind_name(kind: &EnemyKind) -> &'static str {
    match kind {
        EnemyKind::Slime => "Slime",
        EnemyKind::Skeleton => "Skeleton",
        EnemyKind::Bat => "Bat",
        EnemyKind::Goblin => "Goblin",
        EnemyKind::DarkKnight => "Dark Knight",
        EnemyKind::Boss => "Boss",
    }
}

/// Check for level up.
fn check_level_up(state: &mut SimpleGameState) {
    let exp_needed = state.level * 25;
    if state.exp >= exp_needed {
        state.level += 1;
        state.exp -= exp_needed;
        state.max_health += 10;
        state.health = state.max_health;
        state.attack += 3;
        state.defense += 2;
        println!("*** LEVEL UP! You are now level {}! ***", state.level);
    }
}

/// Apply an attack command to the game state.
fn apply_attack(state: &mut SimpleGameState) {
    if let Some(idx) = find_adjacent_enemy(state) {
        let damage = calc_damage(state.attack);
        state.enemies[idx].health -= damage;
        let name = enemy_kind_name(&state.enemies[idx].kind);
        if state.enemies[idx].health <= 0 {
            defeat_enemy(state, idx);
        } else {
            let hp = state.enemies[idx].health;
            state.set_message(&format!(
                "You hit the {} for {} damage! ({} HP left)",
                name, damage, hp
            ));
        }
    } else {
        state.set_message("No enemy nearby to attack!");
    }
}

/// Use a health potion.
fn use_potion(state: &mut SimpleGameState) {
    if state.potions > 0 {
        state.potions -= 1;
        let heal = 30;
        state.health = (state.health + heal).min(state.max_health);
        state.set_message(&format!(
            "You drink a potion and heal {} HP! ({} potions left)",
            heal, state.potions
        ));
    } else {
        state.set_message("You don't have any potions!");
    }
}

/// Collect item at player position.
fn collect_item(state: &mut SimpleGameState) {
    let x = state.player_x;
    let y = state.player_y;
    if let Some(idx) = state.items.iter().position(|i| i.x == x && i.y == y) {
        let item = state.items.remove(idx);
        apply_item_effect(state, &item);
    }
}

/// Apply effect of collected item.
fn apply_item_effect(state: &mut SimpleGameState, item: &Item) {
    match item.kind {
        ItemKind::Potion => {
            state.potions += 1;
            state.set_message("You found a health potion!");
        }
        ItemKind::Gold => {
            state.gold += 25;
            state.score += 50;
            state.set_message("You found 25 gold coins! +50 score");
        }
        ItemKind::Chest => {
            state.gold += 100;
            state.score += 200;
            state.set_message("You opened a treasure chest! +100 gold, +200 score");
        }
        ItemKind::Sword => {
            state.attack += 10;
            state.set_message("You found a better sword! +10 attack");
        }
    }
}

/// Move enemies toward player.
fn move_enemies(state: &mut SimpleGameState) {
    for i in 0..state.enemies.len() {
        move_single_enemy(state, i);
    }
}

/// Move a single enemy.
fn move_single_enemy(state: &mut SimpleGameState, idx: usize) {
    let enemy = &state.enemies[idx];
    let dx = (state.player_x - enemy.x).signum();
    let dy = (state.player_y - enemy.y).signum();
    let new_x = enemy.x + dx;
    let new_y = enemy.y + dy;
    let walkable = is_walkable(&state.terrain, new_x, new_y);
    let occupied = is_position_occupied(state, new_x, new_y, idx);
    if walkable && !occupied {
        state.enemies[idx].x = new_x;
        state.enemies[idx].y = new_y;
    }
}

/// Check if position is occupied by another enemy.
fn is_position_occupied(state: &SimpleGameState, x: i32, y: i32, skip: usize) -> bool {
    for (i, e) in state.enemies.iter().enumerate() {
        if i != skip && e.x == x && e.y == y {
            return true;
        }
    }
    x == state.player_x && y == state.player_y
}

/// Process enemy attacks.
fn enemy_attacks(state: &mut SimpleGameState) {
    for enemy in &state.enemies {
        if is_adjacent(enemy.x, enemy.y, state.player_x, state.player_y) {
            let damage = (enemy.attack - state.defense).max(1);
            state.health -= damage;
            let name = enemy_kind_name(&enemy.kind);
            println!("The {} hits you for {} damage!", name, damage);
        }
    }
}

/// Check if two positions are adjacent.
fn is_adjacent(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
}

/// Process end of turn.
fn end_turn(state: &mut SimpleGameState) {
    state.turn += 1;
    collect_item(state);
    move_enemies(state);
    enemy_attacks(state);
    if state.enemies.is_empty() {
        state.set_message("Victory! All enemies defeated!");
        state.score += 500;
        state.is_running = false;
    }
    if state.health <= 0 {
        state.set_message("You have been defeated...");
        state.is_running = false;
    }
}

/// Process a command on the game state.
pub fn process_command(state: &mut SimpleGameState, cmd: &Command) {
    state.clear_message();
    match cmd {
        Command::Move(dir) => {
            apply_move(state, dir);
            end_turn(state);
        }
        Command::Attack => {
            apply_attack(state);
            end_turn(state);
        }
        Command::UseItem => {
            use_potion(state);
            end_turn(state);
        }
        Command::Wait => {
            state.set_message("You wait...");
            end_turn(state);
        }
        Command::Quit => state.is_running = false,
        _ => {}
    }
}

/// Print the map header.
fn print_map_header() {
    println!("\n=== MAP ===");
}

/// Print map legend.
fn print_legend() {
    println!("@ You | s/k/b/g/D/B Enemies | * Potion | $ Gold | C Chest | + Sword");
}

/// Get character at map position.
fn get_map_char(state: &SimpleGameState, x: i32, y: i32) -> char {
    if state.player_x == x && state.player_y == y {
        return '@';
    }
    if let Some(enemy) = state.enemies.iter().find(|e| e.x == x && e.y == y) {
        return enemy_symbol(&enemy.kind);
    }
    if let Some(item) = state.items.iter().find(|i| i.x == x && i.y == y) {
        return item_symbol(&item.kind);
    }
    tile_symbol(&state.terrain[y as usize][x as usize])
}

/// Print a single map row.
fn print_map_row(state: &SimpleGameState, y: i32) {
    for x in 0..MAP_WIDTH {
        print!("{} ", get_map_char(state, x, y));
    }
    println!();
}

/// Display the game map.
pub fn display_map(state: &SimpleGameState) {
    print_map_header();
    for y in 0..MAP_HEIGHT {
        print_map_row(state, y);
    }
    print_legend();
}

/// Print the title border line.
fn print_border() {
    println!("╔════════════════════════════════════════╗");
}

/// Print the title text line.
fn print_title_text() {
    println!("║     LEGEND OF WASM: HYRULE HEROES      ║");
}

/// Print the title separator line.
fn print_separator() {
    println!("╠════════════════════════════════════════╣");
}

/// Print the subtitle text line.
fn print_subtitle() {
    println!("║   A WebAssembly Component Adventure    ║");
}

/// Print the bottom border line.
fn print_bottom_border() {
    println!("╚════════════════════════════════════════╝");
}

/// Display the game title screen.
pub fn display_title() {
    println!();
    print_border();
    print_title_text();
    print_separator();
    print_subtitle();
    print_bottom_border();
    println!();
}

/// Print the game over message box.
fn print_game_over_msg() {
    println!("╔════════════════════════════════════════╗");
    println!("║              GAME OVER                 ║");
    println!("╚════════════════════════════════════════╝");
}

/// Print the victory message box.
fn print_victory_msg() {
    println!("╔════════════════════════════════════════╗");
    println!("║         VICTORY! HYRULE IS SAVED!      ║");
    println!("╚════════════════════════════════════════╝");
}

/// Display the game over screen.
pub fn display_game_over(is_victory: bool) {
    println!();
    if is_victory {
        print_victory_msg();
    } else {
        print_game_over_msg();
    }
    println!();
}

/// Display help information.
pub fn display_help() {
    println!("\n=== COMMANDS ===");
    println!("n/s/e/w - Move in direction");
    println!("a - Attack adjacent enemy");
    println!("u - Use health potion");
    println!("i - Inventory");
    println!("stat - Status");
    println!(". - Wait a turn");
    println!("h - Help");
    println!("q - Quit");
}

/// Display player status.
pub fn display_status(state: &SimpleGameState) {
    println!("\n=== STATUS ===");
    println!("HP: {}/{}", state.health, state.max_health);
    println!(
        "Level: {} (EXP: {}/{})",
        state.level,
        state.exp,
        state.level * 25
    );
    println!("Attack: {}  Defense: {}", state.attack, state.defense);
    println!("Gold: {}  Potions: {}", state.gold, state.potions);
    println!("Score: {}", state.score);
    println!("Area: {}", state.area_name());
    println!("Turn: {}", state.turn);
    println!("Enemies remaining: {}", state.enemies.len());
}

/// Display inventory.
fn display_inventory(state: &SimpleGameState) {
    println!("\n=== INVENTORY ===");
    println!("Potions: {}", state.potions);
    println!("Gold: {}", state.gold);
    if state.potions > 0 {
        println!("\nUse 'u' to drink a potion.");
    }
}

/// Read user input from stdin.
pub fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

/// Handle unknown command.
fn handle_unknown() {
    println!("Unknown command. Type 'h' for help.");
}

/// Display any message.
fn display_message(state: &SimpleGameState) {
    if !state.message.is_empty() {
        println!("{}", state.message);
    }
}

/// Display HUD (heads up display).
fn display_hud(state: &SimpleGameState) {
    println!(
        "HP: {}/{}  Lvl: {}  Score: {}  Turn: {}",
        state.health, state.max_health, state.level, state.score, state.turn
    );
}

/// Execute a parsed command.
fn execute_command(state: &mut SimpleGameState, cmd: &Command) {
    match cmd {
        Command::Help => display_help(),
        Command::Inventory => display_inventory(state),
        Command::Status => display_status(state),
        Command::Unknown => handle_unknown(),
        Command::Interact => {
            state.set_message("Nothing to interact with here.");
            end_turn(state);
        }
        _ => process_command(state, cmd),
    }
}

/// Run a single game loop iteration.
fn game_loop_iteration(state: &mut SimpleGameState) {
    let input = read_input();
    let cmd = parse_input(&input);
    execute_command(state, &cmd);
}

/// Run the main game loop.
pub fn run_game_loop(state: &mut SimpleGameState) {
    while state.is_running && state.health > 0 {
        display_map(state);
        display_hud(state);
        display_message(state);
        game_loop_iteration(state);
    }
}

/// Start the game and display intro.
fn start_game() {
    display_title();
    println!("Welcome, Hero! Your quest begins...");
    println!("Defeat all enemies to save Hyrule!");
    println!("Collect items (* potions, $ gold, + swords) to grow stronger.\n");
    display_help();
}

/// End the game and display final results.
fn end_game(state: &SimpleGameState) {
    let victory = state.enemies.is_empty() && state.health > 0;
    display_game_over(victory);
    println!("Final Score: {}", state.score);
    println!("Level: {}  Gold: {}", state.level, state.gold);
    println!("Turns: {}", state.turn);
}

/// Main entry point for the command-line game.
fn main() {
    start_game();
    let mut state = SimpleGameState::new();
    run_game_loop(&mut state);
    end_game(&state);
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing north movement command.
    #[test]
    fn test_parse_north() {
        assert_eq!(parse_input("n"), Command::Move(Direction::North));
        assert_eq!(parse_input("north"), Command::Move(Direction::North));
    }

    /// Test parsing south movement command.
    #[test]
    fn test_parse_south() {
        assert_eq!(parse_input("s"), Command::Move(Direction::South));
        assert_eq!(parse_input("south"), Command::Move(Direction::South));
    }

    /// Test parsing east movement command.
    #[test]
    fn test_parse_east() {
        assert_eq!(parse_input("e"), Command::Move(Direction::East));
        assert_eq!(parse_input("east"), Command::Move(Direction::East));
    }

    /// Test parsing west movement command.
    #[test]
    fn test_parse_west() {
        assert_eq!(parse_input("w"), Command::Move(Direction::West));
        assert_eq!(parse_input("west"), Command::Move(Direction::West));
    }

    /// Test parsing attack command.
    #[test]
    fn test_parse_attack() {
        assert_eq!(parse_input("a"), Command::Attack);
        assert_eq!(parse_input("attack"), Command::Attack);
    }

    /// Test parsing interact command.
    #[test]
    fn test_parse_interact() {
        assert_eq!(parse_input("x"), Command::Interact);
        assert_eq!(parse_input("interact"), Command::Interact);
    }

    /// Test parsing help command.
    #[test]
    fn test_parse_help() {
        assert_eq!(parse_input("h"), Command::Help);
        assert_eq!(parse_input("help"), Command::Help);
        assert_eq!(parse_input("?"), Command::Help);
    }

    /// Test parsing quit command.
    #[test]
    fn test_parse_quit() {
        assert_eq!(parse_input("q"), Command::Quit);
        assert_eq!(parse_input("quit"), Command::Quit);
        assert_eq!(parse_input("exit"), Command::Quit);
    }

    /// Test parsing wait command.
    #[test]
    fn test_parse_wait() {
        assert_eq!(parse_input("."), Command::Wait);
        assert_eq!(parse_input("wait"), Command::Wait);
    }

    /// Test parsing inventory command.
    #[test]
    fn test_parse_inventory() {
        assert_eq!(parse_input("i"), Command::Inventory);
        assert_eq!(parse_input("inv"), Command::Inventory);
        assert_eq!(parse_input("inventory"), Command::Inventory);
    }

    /// Test parsing unknown command.
    #[test]
    fn test_parse_unknown() {
        assert_eq!(parse_input("xyz"), Command::Unknown);
        assert_eq!(parse_input("foo bar"), Command::Unknown);
    }

    /// Test case insensitivity of parser.
    #[test]
    fn test_parse_case_insensitive() {
        assert_eq!(parse_input("N"), Command::Move(Direction::North));
        assert_eq!(parse_input("NORTH"), Command::Move(Direction::North));
        assert_eq!(parse_input("Attack"), Command::Attack);
    }

    /// Test SimpleGameState creation.
    #[test]
    fn test_new_state() {
        let state = SimpleGameState::new();
        assert_eq!(state.player_x, 10);
        assert_eq!(state.player_y, 10);
        assert_eq!(state.health, 100);
        assert_eq!(state.score, 0);
        assert!(state.is_running);
    }

    /// Test SimpleGameState default trait.
    #[test]
    fn test_state_default() {
        let state = SimpleGameState::default();
        assert_eq!(state.player_x, 10);
        assert_eq!(state.player_y, 10);
    }

    /// Test apply_move for north direction.
    #[test]
    fn test_apply_move_north() {
        let mut state = SimpleGameState::new();
        state.player_y = 5;
        apply_move(&mut state, &Direction::North);
        assert_eq!(state.player_y, 4);
    }

    /// Test apply_move for south direction.
    #[test]
    fn test_apply_move_south() {
        let mut state = SimpleGameState::new();
        state.player_x = 5;
        state.player_y = 5;
        apply_move(&mut state, &Direction::South);
        assert_eq!(state.player_y, 6);
    }

    /// Test apply_move for east direction.
    #[test]
    fn test_apply_move_east() {
        let mut state = SimpleGameState::new();
        state.player_x = 5;
        state.player_y = 5;
        apply_move(&mut state, &Direction::East);
        assert_eq!(state.player_x, 6);
    }

    /// Test apply_move for west direction.
    #[test]
    fn test_apply_move_west() {
        let mut state = SimpleGameState::new();
        state.player_x = 5;
        apply_move(&mut state, &Direction::West);
        assert_eq!(state.player_x, 4);
    }

    /// Test movement blocked by water.
    #[test]
    fn test_move_blocked_water() {
        let mut state = SimpleGameState::new();
        state.player_x = 9;
        state.player_y = 6;
        apply_move(&mut state, &Direction::East);
        assert_eq!(state.player_x, 9);
    }

    /// Test create slime.
    #[test]
    fn test_create_slime() {
        let enemy = create_slime(5, 5);
        assert_eq!(enemy.kind, EnemyKind::Slime);
        assert_eq!(enemy.health, 10);
        assert_eq!(enemy.exp, 5);
    }

    /// Test create boss.
    #[test]
    fn test_create_boss() {
        let enemy = create_boss(10, 10);
        assert_eq!(enemy.kind, EnemyKind::Boss);
        assert_eq!(enemy.health, 100);
        assert_eq!(enemy.exp, 100);
    }

    /// Test enemy symbols.
    #[test]
    fn test_enemy_symbol() {
        assert_eq!(enemy_symbol(&EnemyKind::Slime), 's');
        assert_eq!(enemy_symbol(&EnemyKind::Boss), 'B');
    }

    /// Test item symbols.
    #[test]
    fn test_item_symbol() {
        assert_eq!(item_symbol(&ItemKind::Potion), '*');
        assert_eq!(item_symbol(&ItemKind::Gold), '$');
    }

    /// Test tile symbols.
    #[test]
    fn test_tile_symbol() {
        assert_eq!(tile_symbol(&Tile::Grass), '.');
        assert_eq!(tile_symbol(&Tile::Water), '~');
    }

    /// Test is_adjacent.
    #[test]
    fn test_is_adjacent() {
        assert!(is_adjacent(5, 5, 5, 6));
        assert!(is_adjacent(5, 5, 6, 5));
        assert!(!is_adjacent(5, 5, 6, 6));
        assert!(!is_adjacent(5, 5, 7, 5));
    }

    /// Test calc_damage.
    #[test]
    fn test_calc_damage() {
        assert_eq!(calc_damage(20), 25);
        assert_eq!(calc_damage(15), 18);
    }

    /// Test area_name.
    #[test]
    fn test_area_name() {
        assert_eq!(area_name(0), "Hyrule Field NW");
        assert_eq!(area_name(1), "Hyrule Castle");
        assert_eq!(area_name(99), "Unknown Lands");
    }

    /// Test is_walkable.
    #[test]
    fn test_is_walkable() {
        let state = SimpleGameState::new();
        assert!(is_walkable(&state.terrain, 5, 5));
        assert!(!is_walkable(&state.terrain, 10, 6));
    }

    /// Test use_potion.
    #[test]
    fn test_use_potion() {
        let mut state = SimpleGameState::new();
        state.health = 50;
        state.potions = 2;
        use_potion(&mut state);
        assert_eq!(state.health, 80);
        assert_eq!(state.potions, 1);
    }

    /// Test use_potion cap.
    #[test]
    fn test_use_potion_cap() {
        let mut state = SimpleGameState::new();
        state.health = 90;
        state.potions = 1;
        use_potion(&mut state);
        assert_eq!(state.health, 100);
    }

    /// Test use_potion empty.
    #[test]
    fn test_use_potion_empty() {
        let mut state = SimpleGameState::new();
        state.potions = 0;
        use_potion(&mut state);
        assert_eq!(state.health, 100);
    }

    /// Test quit command.
    #[test]
    fn test_quit_stops_game() {
        let mut state = SimpleGameState::new();
        process_command(&mut state, &Command::Quit);
        assert!(!state.is_running);
    }

    /// Test enemy kind name.
    #[test]
    fn test_enemy_kind_name() {
        assert_eq!(enemy_kind_name(&EnemyKind::Slime), "Slime");
        assert_eq!(enemy_kind_name(&EnemyKind::Boss), "Boss");
    }

    /// Test initial terrain.
    #[test]
    fn test_init_terrain() {
        let terrain = init_terrain();
        assert_eq!(terrain.len(), MAP_HEIGHT as usize);
        assert_eq!(terrain[0].len(), MAP_WIDTH as usize);
    }

    /// Test spawn enemies.
    #[test]
    fn test_spawn_enemies() {
        let enemies = spawn_enemies();
        assert!(!enemies.is_empty());
        assert!(enemies.len() >= 5);
    }

    /// Test spawn items.
    #[test]
    fn test_spawn_items() {
        let items = spawn_items();
        assert!(!items.is_empty());
        assert!(items.len() >= 4);
    }

    /// Test find_adjacent_enemy.
    #[test]
    fn test_find_adjacent_enemy() {
        let mut state = SimpleGameState::new();
        state.player_x = 5;
        state.player_y = 5;
        state.enemies.clear();
        state.enemies.push(create_slime(5, 4));
        assert!(find_adjacent_enemy(&state).is_some());
    }

    /// Test find_adjacent_enemy none.
    #[test]
    fn test_find_adjacent_enemy_none() {
        let mut state = SimpleGameState::new();
        state.player_x = 5;
        state.player_y = 5;
        state.enemies.clear();
        state.enemies.push(create_slime(10, 10));
        assert!(find_adjacent_enemy(&state).is_none());
    }

    /// Test state area_name method.
    #[test]
    fn test_state_area_name() {
        let mut state = SimpleGameState::new();
        state.player_x = 10;
        state.player_y = 7;
        assert!(!state.area_name().is_empty());
    }
}
