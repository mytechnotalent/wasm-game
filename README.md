# Legend of WASM - A Zelda-Style Adventure

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-Component%20Model-654FF0.svg)](https://component-model.bytecodealliance.org/)

A comprehensive Zelda-style action-adventure game demonstrating the WebAssembly Component Model with Rust. This project builds a modular game using composable WASM components.

## Overview

This project walks you through building, composing, and running a WebAssembly component-based game. The architecture demonstrates how WebAssembly components can be developed independently and composed together at deployment time.

### Components

| Component       | Description                  | Exports              | Imports                          |
| --------------- | ---------------------------- | -------------------- | -------------------------------- |
| **player**      | Character movement and stats | `docs:player/*`      | None                             |
| **enemy**       | Monster AI and spawning      | `docs:enemy/*`       | None                             |
| **combat**      | Battle mechanics             | `docs:combat/*`      | None                             |
| **inventory**   | Items and equipment          | `docs:inventory/*`   | None                             |
| **game_engine** | Main game loop               | `docs:game-engine/*` | player, enemy, combat, inventory |
| **command**     | CLI interface                | `wasi:cli/run`       | game_engine                      |

## Prerequisites

Install the following tools:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-wasip1

# cargo-component for building WASM components
cargo install cargo-component --locked

# wac CLI for component composition
cargo install wac-cli

# wasmtime runtime
curl https://wasmtime.dev/install.sh -sSf | bash
```

After installing wasmtime, restart your shell to ensure the binary is on your PATH.

## Project Structure

```
wasm-game/
├── player/                     # Player character component
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Movement, health, stats with tests
├── enemy/                      # Enemy AI component
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Spawning, AI, damage with tests
├── combat/                     # Combat mechanics component
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Damage calc, battles with tests
├── inventory/                  # Inventory management component
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Items, equipment with tests
├── game_engine/                # Main game engine component
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Game loop, world with tests
├── command/                    # CLI interface component
│   ├── Cargo.toml
│   └── src/
│       └── main.rs             # Input handling with tests
├── wit/                        # WIT interface definitions
│   ├── player/world.wit
│   ├── enemy/world.wit
│   ├── combat/world.wit
│   ├── inventory/world.wit
│   └── game_engine/world.wit
├── Cargo.toml                  # Workspace manifest
├── README.md
└── LICENSE
```

## WIT Interfaces

The WebAssembly Interface Types (WIT) define the contracts between components.

### player/world.wit

```wit
package docs:player@0.1.0;

interface types {
    enum direction { north, south, west, east }
    record position { x: s32, y: s32 }
    record player-stats { health: u32, max-health: u32, attack: u32, defense: u32, experience: u32, level: u32 }
}

interface movement {
    use types.{direction, position};
    move-player: func(current-pos: position, dir: direction) -> position;
    calculate-distance: func(from: position, to: position) -> u32;
}

interface stats {
    use types.{player-stats};
    create-player: func() -> player-stats;
    take-damage: func(stats: player-stats, raw-damage: u32) -> player-stats;
    heal: func(stats: player-stats, amount: u32) -> player-stats;
    gain-experience: func(stats: player-stats, exp: u32) -> player-stats;
    is-defeated: func(stats: player-stats) -> bool;
}

world player {
    export types;
    export movement;
    export stats;
}
```

### enemy/world.wit

```wit
package docs:enemy@0.1.0;

interface types {
    enum enemy-kind { slime, skeleton, bat, goblin, dark-knight, boss }
    enum behavior { wander, chase, guard, flee, boss-pattern }
    record position { x: s32, y: s32 }
    record enemy-state { kind: enemy-kind, health: u32, attack: u32, ... }
}

interface spawn {
    spawn-enemy: func(kind: enemy-kind, pos: position) -> enemy-state;
    spawn-boss: func(pos: position) -> enemy-state;
}

interface ai {
    calculate-move: func(enemy: enemy-state, player-pos: position) -> position;
    should-attack: func(enemy: enemy-state, player-pos: position) -> bool;
}

world enemy {
    export types;
    export spawn;
    export ai;
    export damage;
}
```

### combat/world.wit

```wit
package docs:combat@0.1.0;

interface types {
    enum attack-type { sword-slash, spin-attack, bow-shot, magic-attack, shield-bash }
    record combat-result { damage-dealt: u32, is-critical: bool, target-defeated: bool, ... }
}

interface damage {
    calculate-base-damage: func(attack: attack-type, attacker-stats: combatant-stats) -> u32;
    calculate-final-damage: func(attack: attack-type, attacker: combatant-stats, defender: combatant-stats) -> u32;
}

interface actions {
    player-attack: func(attack: attack-type, player-stats: combatant-stats, enemy-stats: combatant-stats, enemy-exp: u32) -> combat-result;
    enemy-attack: func(enemy-attack: u32, enemy-stats: combatant-stats, player-stats: combatant-stats) -> combat-result;
}

world combat {
    export types;
    export damage;
    export actions;
    export battle;
}
```

## Building Components

### Build All Components

```bash
# Build each component for WebAssembly
cd player && cargo component build --release && cd ..
cd enemy && cargo component build --release && cd ..
cd combat && cargo component build --release && cd ..
cd inventory && cargo component build --release && cd ..
cd game_engine && cargo component build --release && cd ..
cd command && cargo component build --release && cd ..
```

### Output Locations

| Component   | Output Path                                               | Description         |
| ----------- | --------------------------------------------------------- | ------------------- |
| player      | player/target/wasm32-wasip1/release/player.wasm           | Character logic     |
| enemy       | enemy/target/wasm32-wasip1/release/enemy.wasm             | Monster logic       |
| combat      | combat/target/wasm32-wasip1/release/combat.wasm           | Battle system       |
| inventory   | inventory/target/wasm32-wasip1/release/inventory.wasm     | Item management     |
| game_engine | game_engine/target/wasm32-wasip1/release/game_engine.wasm | Game state          |
| **command** | **command/target/wasm32-wasip1/release/command.wasm**     | **Main executable** |

## Running the Game

Execute the command component with wasmtime from the project root directory:

```bash
# Navigate to the project root first
cd /path/to/wasm-game

# Build the command component
cd command && cargo component build --release && cd ..

# Start the game (from project root)
wasmtime run target/wasm32-wasip1/release/command.wasm
```

**Important:** The compiled WASM file is located in the workspace's shared `target/` directory, not in `command/target/`. Always run from the project root.

### Installing wasmtime

If you don't have wasmtime installed:

```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```

After installation, restart your terminal or run `source ~/.zshrc` (or `~/.bashrc`).

### Game Controls

| Command     | Shortcut | Description          |
| ----------- | -------- | -------------------- |
| `north`     | `n`      | Move north           |
| `south`     | `s`      | Move south           |
| `east`      | `e`      | Move east            |
| `west`      | `w`      | Move west            |
| `attack`    | `a`      | Attack with weapon   |
| `use`       | `u`      | Use item             |
| `inventory` | `i`      | Open inventory       |
| `interact`  | `x`      | Interact with object |
| `wait`      | `.`      | Skip turn            |
| `help`      | -        | Show commands        |
| `quit`      | `q`      | Exit game            |

## Testing

Each component includes comprehensive unit tests.

### Run All Tests

```bash
# Test all components
cargo test --manifest-path player/Cargo.toml
cargo test --manifest-path enemy/Cargo.toml
cargo test --manifest-path combat/Cargo.toml
cargo test --manifest-path inventory/Cargo.toml
cargo test --manifest-path game_engine/Cargo.toml
cargo test --manifest-path command/Cargo.toml

# Or test entire workspace
cargo test --workspace
```

### Test Summary

| Component   | Tests   | Coverage Areas                                |
| ----------- | ------- | --------------------------------------------- |
| player      | 35      | Movement, damage, healing, experience, levels |
| enemy       | 41      | Spawning, AI behavior, damage, defeat         |
| combat      | 35      | Damage calculation, attacks, battles, flee    |
| inventory   | 35      | Item creation, management, usage, equipment   |
| game_engine | 31      | Game init, actions, world, events             |
| command     | 33      | Input parsing, action formatting              |
| **Total**   | **210** |                                               |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              command                                    │
│                          (CLI Interface)                                │
│                        exports: wasi:cli/run                            │
└────────────────────────────────┬────────────────────────────────────────┘
                                 │ imports
                                 ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           game_engine                                   │
│                       (Main Game Loop)                                  │
│                  exports: docs:game-engine/*                            │
└─────┬──────────────┬─────────────────┬──────────────────┬───────────────┘
      │ imports      │ imports         │ imports          │ imports
      ▼              ▼                 ▼                  ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│    player    │ │    enemy     │ │    combat    │ │  inventory   │
│  (Character) │ │  (Monsters)  │ │  (Battles)   │ │   (Items)    │
│ exports:     │ │ exports:     │ │ exports:     │ │ exports:     │
│ docs:player  │ │ docs:enemy   │ │ docs:combat  │ │ docs:inv     │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

## Game Features

### Player System
- 4-directional movement on tile-based map
- Health, attack, defense, and level stats
- Experience points and level progression
- Equipment bonuses from weapons and armor

### Enemy System
- 6 enemy types: Slime, Skeleton, Bat, Goblin, Dark Knight, Boss
- AI behaviors: Wander, Chase, Guard, Flee, Boss Pattern
- Unique stats and experience rewards per type
- Dynamic behavior changes based on health

### Combat System
- 5 attack types with different damage modifiers
- Critical hit system
- Defense-based damage reduction
- Turn-based battle management
- Flee mechanic

### Inventory System
- Weapons: Wooden Sword, Steel Sword, Master Sword, Bow, Fire Rod
- Armor: Cloth Tunic, Leather Armor, Chain Mail, Shield, Magic Robe
- Consumables: Health potions, boost elixirs
- Gold currency system
- Equipment management

### World System
- 20x20 tile-based world map
- Multiple tile types: Grass, Forest, Water, Walls
- Special locations: Shops, NPCs, Chests, Dungeons
- Dynamic area naming
- Event triggers

## Key Concepts

### WebAssembly Component Model

The Component Model extends WebAssembly with:
- **Interface Types (WIT)**: Language-agnostic interface definitions
- **Composition**: Linking components at deployment time
- **Isolation**: Each component runs in its own sandbox

### Design Patterns Used

1. **Separation of Concerns**: Each game system is its own component
2. **Dependency Injection**: Game engine imports its dependencies via WIT
3. **Testability**: Core logic separated from WASM bindings for unit testing
4. **Entity-Component Pattern**: Stats and state passed as records

## Documentation

All source files include comprehensive Rust documentation:
- MIT License headers
- Module-level `//!` documentation
- Function docstrings with `# Details`, `# Arguments`, `# Returns` sections
- Inline comments explaining logic
- Test documentation

Generate documentation locally:

```bash
cargo doc --workspace --open
```

## Future Enhancements

- [ ] Add more enemy types and boss mechanics
- [ ] Implement dungeon generation
- [ ] Add save/load game functionality
- [ ] Create web-based UI with browser WASM
- [ ] Add multiplayer via component networking
- [ ] Sound effects and music via WASI audio

## License

MIT License

Copyright (c) 2025 Kevin Thomas

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the
Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR
A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
