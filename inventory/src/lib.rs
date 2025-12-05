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

//! # Inventory Component
//!
//! WebAssembly component implementing inventory mechanics for the Legend of WASM game.
//! Handles items, equipment, and inventory management.

#![allow(dead_code)]

mod bindings;

use bindings::exports::docs::inventory::items::{Guest as ItemsGuest, Item as ItemsItem};
use bindings::exports::docs::inventory::management::{Guest as ManagementGuest, InventoryState};
use bindings::exports::docs::inventory::types::{
    ArmorType, ConsumableType, Item, ItemCategory, UseResult, WeaponType,
};
use bindings::exports::docs::inventory::usage::Guest as UsageGuest;

/// Default maximum inventory capacity.
const DEFAULT_MAX_CAPACITY: u32 = 20;

/// Component structure for inventory functionality.
struct Component;

bindings::export!(Component with_types_in bindings);

/// Get weapon ID based on type.
///
/// # Arguments
///
/// * `weapon` - Weapon type
///
/// # Returns
///
/// * `u32` - Unique weapon ID
fn weapon_id(weapon: &WeaponType) -> u32 {
    match weapon {
        WeaponType::WoodenSword => 1,
        WeaponType::SteelSword => 2,
        WeaponType::MasterSword => 3,
        WeaponType::Bow => 4,
        WeaponType::FireRod => 5,
    }
}

/// Get weapon name based on type.
///
/// # Arguments
///
/// * `weapon` - Weapon type
///
/// # Returns
///
/// * `String` - Weapon name
fn weapon_name(weapon: &WeaponType) -> String {
    match weapon {
        WeaponType::WoodenSword => "Wooden Sword".to_string(),
        WeaponType::SteelSword => "Steel Sword".to_string(),
        WeaponType::MasterSword => "Master Sword".to_string(),
        WeaponType::Bow => "Bow".to_string(),
        WeaponType::FireRod => "Fire Rod".to_string(),
    }
}

/// Get weapon attack bonus.
///
/// # Arguments
///
/// * `weapon` - Weapon type
///
/// # Returns
///
/// * `u32` - Attack bonus value
fn weapon_attack(weapon: &WeaponType) -> u32 {
    match weapon {
        WeaponType::WoodenSword => 5,
        WeaponType::SteelSword => 10,
        WeaponType::MasterSword => 25,
        WeaponType::Bow => 8,
        WeaponType::FireRod => 15,
    }
}

/// Get armor ID based on type.
///
/// # Arguments
///
/// * `armor` - Armor type
///
/// # Returns
///
/// * `u32` - Unique armor ID
fn armor_id(armor: &ArmorType) -> u32 {
    match armor {
        ArmorType::ClothTunic => 101,
        ArmorType::LeatherArmor => 102,
        ArmorType::ChainMail => 103,
        ArmorType::Shield => 104,
        ArmorType::MagicRobe => 105,
    }
}

/// Get armor name based on type.
///
/// # Arguments
///
/// * `armor` - Armor type
///
/// # Returns
///
/// * `String` - Armor name
fn armor_name(armor: &ArmorType) -> String {
    match armor {
        ArmorType::ClothTunic => "Cloth Tunic".to_string(),
        ArmorType::LeatherArmor => "Leather Armor".to_string(),
        ArmorType::ChainMail => "Chain Mail".to_string(),
        ArmorType::Shield => "Shield".to_string(),
        ArmorType::MagicRobe => "Magic Robe".to_string(),
    }
}

/// Get armor defense bonus.
///
/// # Arguments
///
/// * `armor` - Armor type
///
/// # Returns
///
/// * `u32` - Defense bonus value
fn armor_defense(armor: &ArmorType) -> u32 {
    match armor {
        ArmorType::ClothTunic => 2,
        ArmorType::LeatherArmor => 5,
        ArmorType::ChainMail => 10,
        ArmorType::Shield => 8,
        ArmorType::MagicRobe => 4,
    }
}

/// Get consumable ID based on type.
///
/// # Arguments
///
/// * `consumable` - Consumable type
///
/// # Returns
///
/// * `u32` - Unique consumable ID
fn consumable_id(consumable: &ConsumableType) -> u32 {
    match consumable {
        ConsumableType::HealthPotion => 201,
        ConsumableType::FullHealthPotion => 202,
        ConsumableType::AttackBoost => 203,
        ConsumableType::DefenseBoost => 204,
        ConsumableType::Antidote => 205,
    }
}

/// Get consumable name based on type.
///
/// # Arguments
///
/// * `consumable` - Consumable type
///
/// # Returns
///
/// * `String` - Consumable name
fn consumable_name(consumable: &ConsumableType) -> String {
    match consumable {
        ConsumableType::HealthPotion => "Health Potion".to_string(),
        ConsumableType::FullHealthPotion => "Full Health Potion".to_string(),
        ConsumableType::AttackBoost => "Attack Boost".to_string(),
        ConsumableType::DefenseBoost => "Defense Boost".to_string(),
        ConsumableType::Antidote => "Antidote".to_string(),
    }
}

/// Get consumable heal amount.
///
/// # Arguments
///
/// * `consumable` - Consumable type
///
/// # Returns
///
/// * `u32` - Heal amount (0 for non-healing items)
fn consumable_heal(consumable: &ConsumableType) -> u32 {
    match consumable {
        ConsumableType::HealthPotion => 50,
        ConsumableType::FullHealthPotion => 9999,
        _ => 0,
    }
}

/// Create a weapon item.
///
/// # Arguments
///
/// * `weapon` - Weapon type
///
/// # Returns
///
/// * `Item` - Created weapon item
fn create_weapon_item(weapon: &WeaponType) -> Item {
    Item {
        id: weapon_id(weapon),
        name: weapon_name(weapon),
        category: ItemCategory::Weapon,
        attack_bonus: weapon_attack(weapon),
        defense_bonus: 0,
        heal_amount: 0,
        quantity: 1,
        is_equipped: false,
    }
}

/// Create an armor item.
///
/// # Arguments
///
/// * `armor` - Armor type
///
/// # Returns
///
/// * `Item` - Created armor item
fn create_armor_item(armor: &ArmorType) -> Item {
    Item {
        id: armor_id(armor),
        name: armor_name(armor),
        category: ItemCategory::Armor,
        attack_bonus: 0,
        defense_bonus: armor_defense(armor),
        heal_amount: 0,
        quantity: 1,
        is_equipped: false,
    }
}

/// Create a consumable item.
///
/// # Arguments
///
/// * `consumable` - Consumable type
/// * `quantity` - Initial quantity
///
/// # Returns
///
/// * `Item` - Created consumable item
fn create_consumable_item(consumable: &ConsumableType, quantity: u32) -> Item {
    Item {
        id: consumable_id(consumable),
        name: consumable_name(consumable),
        category: ItemCategory::Consumable,
        attack_bonus: 0,
        defense_bonus: 0,
        heal_amount: consumable_heal(consumable),
        quantity,
        is_equipped: false,
    }
}

/// Create default inventory state.
///
/// # Returns
///
/// * `InventoryState` - Fresh inventory state
fn create_default_inventory() -> InventoryState {
    InventoryState {
        equipped_weapon: 0,
        equipped_armor: 0,
        item_count: 0,
        max_capacity: DEFAULT_MAX_CAPACITY,
        gold: 0,
    }
}

/// Calculate health after healing.
///
/// # Arguments
///
/// * `current` - Current health
/// * `heal` - Amount to heal
/// * `max` - Maximum health
///
/// # Returns
///
/// * `u32` - Health after healing
fn calculate_healed_health(current: u32, heal: u32, max: u32) -> u32 {
    (current + heal).min(max)
}

/// Calculate health restored from healing.
///
/// # Arguments
///
/// * `current` - Current health
/// * `heal` - Amount to heal
/// * `max` - Maximum health
///
/// # Returns
///
/// * `u32` - Actual amount restored
fn calculate_health_restored(current: u32, heal: u32, max: u32) -> u32 {
    let new_health = calculate_healed_health(current, heal, max);
    new_health - current
}

/// Create successful heal result.
///
/// # Arguments
///
/// * `restored` - Health restored
///
/// # Returns
///
/// * `UseResult` - Success result
fn create_heal_result(restored: u32) -> UseResult {
    UseResult {
        success: true,
        health_restored: restored,
        attack_boost: 0,
        defense_boost: 0,
        message: format!("Restored {} health!", restored),
    }
}

/// Create attack boost result.
///
/// # Arguments
///
/// * `boost` - Attack boost amount
///
/// # Returns
///
/// * `UseResult` - Success result
fn create_attack_boost_result(boost: u32) -> UseResult {
    UseResult {
        success: true,
        health_restored: 0,
        attack_boost: boost,
        defense_boost: 0,
        message: format!("Attack increased by {}!", boost),
    }
}

/// Create defense boost result.
///
/// # Arguments
///
/// * `boost` - Defense boost amount
///
/// # Returns
///
/// * `UseResult` - Success result
fn create_defense_boost_result(boost: u32) -> UseResult {
    UseResult {
        success: true,
        health_restored: 0,
        attack_boost: 0,
        defense_boost: boost,
        message: format!("Defense increased by {}!", boost),
    }
}

/// Create antidote result.
///
/// # Returns
///
/// * `UseResult` - Success result
fn create_antidote_result() -> UseResult {
    UseResult {
        success: true,
        health_restored: 0,
        attack_boost: 0,
        defense_boost: 0,
        message: "Cured poison!".to_string(),
    }
}

/// Create unknown item result.
///
/// # Returns
///
/// * `UseResult` - Failure result
fn create_unknown_item_result() -> UseResult {
    UseResult {
        success: false,
        health_restored: 0,
        attack_boost: 0,
        defense_boost: 0,
        message: "Unknown item!".to_string(),
    }
}

/// Convert Item to ItemsItem (they're the same type).
///
/// # Arguments
///
/// * `item` - Source item
///
/// # Returns
///
/// * `ItemsItem` - Same item
fn to_items_item(item: Item) -> ItemsItem {
    item
}

impl ItemsGuest for Component {
    /// Create a weapon item.
    ///
    /// # Arguments
    ///
    /// * `weapon` - Weapon type to create
    ///
    /// # Returns
    ///
    /// * `Item` - Created weapon item
    fn create_weapon(weapon: WeaponType) -> ItemsItem {
        let item = create_weapon_item(&weapon);
        to_items_item(item)
    }

    /// Create an armor item.
    ///
    /// # Arguments
    ///
    /// * `armor` - Armor type to create
    ///
    /// # Returns
    ///
    /// * `Item` - Created armor item
    fn create_armor(armor: ArmorType) -> ItemsItem {
        let item = create_armor_item(&armor);
        to_items_item(item)
    }

    /// Create a consumable item.
    ///
    /// # Arguments
    ///
    /// * `consumable` - Consumable type to create
    /// * `quantity` - Initial quantity
    ///
    /// # Returns
    ///
    /// * `Item` - Created consumable item
    fn create_consumable(consumable: ConsumableType, quantity: u32) -> ItemsItem {
        let item = create_consumable_item(&consumable, quantity);
        to_items_item(item)
    }

    /// Get item stats by ID.
    ///
    /// # Arguments
    ///
    /// * `item_id` - Item ID to look up
    ///
    /// # Returns
    ///
    /// * `Item` - Item with stats
    fn get_item_stats(item_id: u32) -> ItemsItem {
        let item = get_item_by_id(item_id);
        to_items_item(item)
    }
}

/// Get item by ID (helper function).
///
/// # Arguments
///
/// * `item_id` - Item ID to look up
///
/// # Returns
///
/// * `Item` - Item with stats
fn get_item_by_id(item_id: u32) -> Item {
    match item_id {
        1 => create_weapon_item(&WeaponType::WoodenSword),
        2 => create_weapon_item(&WeaponType::SteelSword),
        3 => create_weapon_item(&WeaponType::MasterSword),
        4 => create_weapon_item(&WeaponType::Bow),
        5 => create_weapon_item(&WeaponType::FireRod),
        101 => create_armor_item(&ArmorType::ClothTunic),
        102 => create_armor_item(&ArmorType::LeatherArmor),
        103 => create_armor_item(&ArmorType::ChainMail),
        104 => create_armor_item(&ArmorType::Shield),
        105 => create_armor_item(&ArmorType::MagicRobe),
        201 => create_consumable_item(&ConsumableType::HealthPotion, 1),
        202 => create_consumable_item(&ConsumableType::FullHealthPotion, 1),
        203 => create_consumable_item(&ConsumableType::AttackBoost, 1),
        204 => create_consumable_item(&ConsumableType::DefenseBoost, 1),
        205 => create_consumable_item(&ConsumableType::Antidote, 1),
        _ => create_unknown_item(),
    }
}

/// Create unknown item placeholder.
///
/// # Returns
///
/// * `Item` - Unknown item
fn create_unknown_item() -> Item {
    Item {
        id: 0,
        name: "Unknown".to_string(),
        category: ItemCategory::Treasure,
        attack_bonus: 0,
        defense_bonus: 0,
        heal_amount: 0,
        quantity: 0,
        is_equipped: false,
    }
}

impl ManagementGuest for Component {
    /// Create a new empty inventory.
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Fresh inventory
    fn create_inventory() -> InventoryState {
        create_default_inventory()
    }

    /// Add an item to the inventory.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `item_id` - Item ID to add
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn add_item(inv: InventoryState, _item_id: u32) -> InventoryState {
        if inv.item_count >= inv.max_capacity {
            return inv;
        }
        InventoryState {
            item_count: inv.item_count + 1,
            ..inv
        }
    }

    /// Remove an item from the inventory.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `item_id` - Item ID to remove
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn remove_item(inv: InventoryState, _item_id: u32) -> InventoryState {
        if inv.item_count == 0 {
            return inv;
        }
        InventoryState {
            item_count: inv.item_count - 1,
            ..inv
        }
    }

    /// Equip a weapon by item ID.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `item_id` - Weapon ID to equip
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn equip_weapon(inv: InventoryState, item_id: u32) -> InventoryState {
        InventoryState {
            equipped_weapon: item_id,
            ..inv
        }
    }

    /// Equip armor by item ID.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `item_id` - Armor ID to equip
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn equip_armor(inv: InventoryState, item_id: u32) -> InventoryState {
        InventoryState {
            equipped_armor: item_id,
            ..inv
        }
    }

    /// Add gold to the inventory.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `amount` - Gold to add
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn add_gold(inv: InventoryState, amount: u32) -> InventoryState {
        InventoryState {
            gold: inv.gold + amount,
            ..inv
        }
    }

    /// Spend gold from the inventory.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    /// * `amount` - Gold to spend
    ///
    /// # Returns
    ///
    /// * `InventoryState` - Updated inventory
    fn spend_gold(inv: InventoryState, amount: u32) -> InventoryState {
        if inv.gold < amount {
            return inv;
        }
        InventoryState {
            gold: inv.gold - amount,
            ..inv
        }
    }

    /// Check if inventory is full.
    ///
    /// # Arguments
    ///
    /// * `inv` - Current inventory state
    ///
    /// # Returns
    ///
    /// * `bool` - True if full
    fn is_full(inv: InventoryState) -> bool {
        inv.item_count >= inv.max_capacity
    }
}

impl UsageGuest for Component {
    /// Use a consumable item.
    ///
    /// # Arguments
    ///
    /// * `item_id` - Item ID to use
    /// * `current_health` - Current health
    /// * `max_health` - Maximum health
    ///
    /// # Returns
    ///
    /// * `UseResult` - Result of using item
    fn use_item(item_id: u32, current_health: u32, max_health: u32) -> UseResult {
        match item_id {
            201 => use_health_potion(current_health, max_health, 50),
            202 => use_health_potion(current_health, max_health, max_health),
            203 => create_attack_boost_result(5),
            204 => create_defense_boost_result(5),
            205 => create_antidote_result(),
            _ => create_unknown_item_result(),
        }
    }

    /// Get total attack bonus from weapon.
    ///
    /// # Arguments
    ///
    /// * `weapon_id` - Equipped weapon ID
    ///
    /// # Returns
    ///
    /// * `u32` - Attack bonus
    fn get_total_attack_bonus(weapon_id: u32) -> u32 {
        let item = get_item_by_id(weapon_id);
        item.attack_bonus
    }

    /// Get total defense bonus from armor.
    ///
    /// # Arguments
    ///
    /// * `armor_id` - Equipped armor ID
    ///
    /// # Returns
    ///
    /// * `u32` - Defense bonus
    fn get_total_defense_bonus(armor_id: u32) -> u32 {
        let item = get_item_by_id(armor_id);
        item.defense_bonus
    }
}

/// Use health potion helper.
///
/// # Arguments
///
/// * `current` - Current health
/// * `max` - Maximum health
/// * `heal` - Heal amount
///
/// # Returns
///
/// * `UseResult` - Heal result
fn use_health_potion(current: u32, max: u32, heal: u32) -> UseResult {
    let restored = calculate_health_restored(current, heal, max);
    create_heal_result(restored)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test wooden sword ID.
    fn test_weapon_id_wooden_sword() {
        let id = weapon_id(&WeaponType::WoodenSword);
        assert_eq!(id, 1);
    }

    #[test]
    /// Test master sword ID.
    fn test_weapon_id_master_sword() {
        let id = weapon_id(&WeaponType::MasterSword);
        assert_eq!(id, 3);
    }

    #[test]
    /// Test wooden sword name.
    fn test_weapon_name_wooden_sword() {
        let name = weapon_name(&WeaponType::WoodenSword);
        assert_eq!(name, "Wooden Sword");
    }

    #[test]
    /// Test master sword attack.
    fn test_weapon_attack_master_sword() {
        let attack = weapon_attack(&WeaponType::MasterSword);
        assert_eq!(attack, 25);
    }

    #[test]
    /// Test chain mail ID.
    fn test_armor_id_chain_mail() {
        let id = armor_id(&ArmorType::ChainMail);
        assert_eq!(id, 103);
    }

    #[test]
    /// Test chain mail defense.
    fn test_armor_defense_chain_mail() {
        let defense = armor_defense(&ArmorType::ChainMail);
        assert_eq!(defense, 10);
    }

    #[test]
    /// Test health potion ID.
    fn test_consumable_id_health_potion() {
        let id = consumable_id(&ConsumableType::HealthPotion);
        assert_eq!(id, 201);
    }

    #[test]
    /// Test health potion heal amount.
    fn test_consumable_heal_health_potion() {
        let heal = consumable_heal(&ConsumableType::HealthPotion);
        assert_eq!(heal, 50);
    }

    #[test]
    /// Test creating weapon item.
    fn test_create_weapon_item() {
        let item = create_weapon_item(&WeaponType::SteelSword);
        assert_eq!(item.attack_bonus, 10);
    }

    #[test]
    /// Test creating armor item.
    fn test_create_armor_item() {
        let item = create_armor_item(&ArmorType::LeatherArmor);
        assert_eq!(item.defense_bonus, 5);
    }

    #[test]
    /// Test creating consumable item.
    fn test_create_consumable_item() {
        let item = create_consumable_item(&ConsumableType::HealthPotion, 3);
        assert_eq!(item.quantity, 3);
    }

    #[test]
    /// Test default inventory creation.
    fn test_create_default_inventory() {
        let inv = create_default_inventory();
        assert_eq!(inv.max_capacity, DEFAULT_MAX_CAPACITY);
    }

    #[test]
    /// Test healed health calculation.
    fn test_calculate_healed_health() {
        let result = calculate_healed_health(50, 30, 100);
        assert_eq!(result, 80);
    }

    #[test]
    /// Test healed health with cap.
    fn test_calculate_healed_health_capped() {
        let result = calculate_healed_health(90, 50, 100);
        assert_eq!(result, 100);
    }

    #[test]
    /// Test health restored calculation.
    fn test_calculate_health_restored() {
        let result = calculate_health_restored(50, 30, 100);
        assert_eq!(result, 30);
    }

    #[test]
    /// Test get item by ID weapon.
    fn test_get_item_by_id_weapon() {
        let item = get_item_by_id(1);
        assert_eq!(item.name, "Wooden Sword");
    }

    #[test]
    /// Test get item by ID armor.
    fn test_get_item_by_id_armor() {
        let item = get_item_by_id(103);
        assert_eq!(item.name, "Chain Mail");
    }

    #[test]
    /// Test get item by ID consumable.
    fn test_get_item_by_id_consumable() {
        let item = get_item_by_id(201);
        assert_eq!(item.name, "Health Potion");
    }

    #[test]
    /// Test get item by unknown ID.
    fn test_get_item_by_id_unknown() {
        let item = get_item_by_id(999);
        assert_eq!(item.name, "Unknown");
    }

    #[test]
    /// Test create inventory function.
    fn test_create_inventory() {
        let inv = <Component as ManagementGuest>::create_inventory();
        assert_eq!(inv.gold, 0);
    }

    #[test]
    /// Test add item to inventory.
    fn test_add_item() {
        let inv = create_default_inventory();
        let updated = <Component as ManagementGuest>::add_item(inv, 1);
        assert_eq!(updated.item_count, 1);
    }

    #[test]
    /// Test remove item from inventory.
    fn test_remove_item() {
        let mut inv = create_default_inventory();
        inv.item_count = 5;
        let updated = <Component as ManagementGuest>::remove_item(inv, 1);
        assert_eq!(updated.item_count, 4);
    }

    #[test]
    /// Test equip weapon.
    fn test_equip_weapon() {
        let inv = create_default_inventory();
        let updated = <Component as ManagementGuest>::equip_weapon(inv, 3);
        assert_eq!(updated.equipped_weapon, 3);
    }

    #[test]
    /// Test equip armor.
    fn test_equip_armor() {
        let inv = create_default_inventory();
        let updated = <Component as ManagementGuest>::equip_armor(inv, 103);
        assert_eq!(updated.equipped_armor, 103);
    }

    #[test]
    /// Test add gold.
    fn test_add_gold() {
        let inv = create_default_inventory();
        let updated = <Component as ManagementGuest>::add_gold(inv, 100);
        assert_eq!(updated.gold, 100);
    }

    #[test]
    /// Test spend gold success.
    fn test_spend_gold_success() {
        let mut inv = create_default_inventory();
        inv.gold = 100;
        let updated = <Component as ManagementGuest>::spend_gold(inv, 50);
        assert_eq!(updated.gold, 50);
    }

    #[test]
    /// Test spend gold insufficient.
    fn test_spend_gold_insufficient() {
        let inv = create_default_inventory();
        let updated = <Component as ManagementGuest>::spend_gold(inv, 50);
        assert_eq!(updated.gold, 0);
    }

    #[test]
    /// Test is full true.
    fn test_is_full_true() {
        let mut inv = create_default_inventory();
        inv.item_count = 20;
        assert!(<Component as ManagementGuest>::is_full(inv));
    }

    #[test]
    /// Test is full false.
    fn test_is_full_false() {
        let inv = create_default_inventory();
        assert!(!<Component as ManagementGuest>::is_full(inv));
    }

    #[test]
    /// Test use health potion.
    fn test_use_item_health_potion() {
        let result = <Component as UsageGuest>::use_item(201, 50, 100);
        assert!(result.success);
        assert_eq!(result.health_restored, 50);
    }

    #[test]
    /// Test use attack boost.
    fn test_use_item_attack_boost() {
        let result = <Component as UsageGuest>::use_item(203, 100, 100);
        assert!(result.success);
        assert_eq!(result.attack_boost, 5);
    }

    #[test]
    /// Test use defense boost.
    fn test_use_item_defense_boost() {
        let result = <Component as UsageGuest>::use_item(204, 100, 100);
        assert!(result.success);
        assert_eq!(result.defense_boost, 5);
    }

    #[test]
    /// Test use unknown item.
    fn test_use_item_unknown() {
        let result = <Component as UsageGuest>::use_item(999, 100, 100);
        assert!(!result.success);
    }

    #[test]
    /// Test get attack bonus.
    fn test_get_total_attack_bonus() {
        let bonus = <Component as UsageGuest>::get_total_attack_bonus(3);
        assert_eq!(bonus, 25);
    }

    #[test]
    /// Test get defense bonus.
    fn test_get_total_defense_bonus() {
        let bonus = <Component as UsageGuest>::get_total_defense_bonus(103);
        assert_eq!(bonus, 10);
    }
}
