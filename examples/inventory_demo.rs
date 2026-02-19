//! Inventory Demo - A comprehensive example showcasing all msg_inventory traits
//!
//! This example demonstrates how to use the inventory trait system to create
//! different item types for a game: spells, perks, consumables, and quest items.
//!
//! Run with: `cargo run --example inventory_demo`

use bevy::prelude::*;
use msg_inventory::prelude::*;

// =============================================================================
// Item Type Definitions
// =============================================================================

/// Spells that can be dragged and arranged in spell slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[allow(dead_code)]
enum Spell {
    Fireball,
    IceShield,
    LightningBolt,
    Heal,
    Teleport,
}

/// Perks that provide passive bonuses with levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[allow(dead_code)]
enum Perk {
    HealthBoost,
    ManaRegeneration,
    CriticalStrike,
    Dodge,
}

/// Consumable items with stack counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[allow(dead_code)]
enum Consumable {
    HealthPotion,
    ManaPotion,
    Antidote,
    Elixir,
}

/// Quest items that are unique and cannot be stacked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[allow(dead_code)]
enum QuestItem {
    AncientKey,
    MysteriousMap,
    DragonScale,
    RoyalSeal,
}

// =============================================================================
// InventoryItem Implementations
// =============================================================================

impl InventoryItem for Spell {
    fn icon_path(&self) -> &str {
        match self {
            Spell::Fireball => "textures/icons/spells/fireball.png",
            Spell::IceShield => "textures/icons/spells/ice_shield.png",
            Spell::LightningBolt => "textures/icons/spells/lightning.png",
            Spell::Heal => "textures/icons/spells/heal.png",
            Spell::Teleport => "textures/icons/spells/teleport.png",
        }
    }
}

impl InventoryItem for Perk {
    fn icon_path(&self) -> &str {
        match self {
            Perk::HealthBoost => "textures/icons/perks/health.png",
            Perk::ManaRegeneration => "textures/icons/perks/mana_regen.png",
            Perk::CriticalStrike => "textures/icons/perks/crit.png",
            Perk::Dodge => "textures/icons/perks/dodge.png",
        }
    }
}

impl InventoryItem for Consumable {
    fn icon_path(&self) -> &str {
        match self {
            Consumable::HealthPotion => "textures/icons/items/health_potion.png",
            Consumable::ManaPotion => "textures/icons/items/mana_potion.png",
            Consumable::Antidote => "textures/icons/items/antidote.png",
            Consumable::Elixir => "textures/icons/items/elixir.png",
        }
    }
}

impl InventoryItem for QuestItem {
    fn icon_path(&self) -> &str {
        match self {
            QuestItem::AncientKey => "textures/icons/quest/ancient_key.png",
            QuestItem::MysteriousMap => "textures/icons/quest/map.png",
            QuestItem::DragonScale => "textures/icons/quest/dragon_scale.png",
            QuestItem::RoyalSeal => "textures/icons/quest/seal.png",
        }
    }
}

// =============================================================================
// InventoryItemData Implementations
// =============================================================================

impl InventoryItemData for Spell {
    fn display_name(&self) -> &str {
        match self {
            Spell::Fireball => "Fireball",
            Spell::IceShield => "Ice Shield",
            Spell::LightningBolt => "Lightning Bolt",
            Spell::Heal => "Heal",
            Spell::Teleport => "Teleport",
        }
    }

    fn description(&self) -> &str {
        match self {
            Spell::Fireball => "Hurls a ball of fire that explodes on impact.",
            Spell::IceShield => "Creates a protective barrier of ice.",
            Spell::LightningBolt => "Strikes a target with powerful lightning.",
            Spell::Heal => "Restores health to the caster.",
            Spell::Teleport => "Instantly moves to a target location.",
        }
    }
}

impl InventoryItemData for Perk {
    fn display_name(&self) -> &str {
        match self {
            Perk::HealthBoost => "Health Boost",
            Perk::ManaRegeneration => "Mana Regeneration",
            Perk::CriticalStrike => "Critical Strike",
            Perk::Dodge => "Dodge",
        }
    }

    fn description(&self) -> &str {
        match self {
            Perk::HealthBoost => "Increases maximum health by 10% per level.",
            Perk::ManaRegeneration => "Increases mana regeneration by 5% per level.",
            Perk::CriticalStrike => "Increases critical hit chance by 2% per level.",
            Perk::Dodge => "Increases dodge chance by 1% per level.",
        }
    }
}

impl InventoryItemData for Consumable {
    fn display_name(&self) -> &str {
        match self {
            Consumable::HealthPotion => "Health Potion",
            Consumable::ManaPotion => "Mana Potion",
            Consumable::Antidote => "Antidote",
            Consumable::Elixir => "Elixir of Power",
        }
    }

    fn description(&self) -> &str {
        match self {
            Consumable::HealthPotion => "Restores 50 health points.",
            Consumable::ManaPotion => "Restores 30 mana points.",
            Consumable::Antidote => "Cures all poison effects.",
            Consumable::Elixir => "Temporarily increases all stats by 20%.",
        }
    }
}

impl InventoryItemData for QuestItem {
    fn display_name(&self) -> &str {
        match self {
            QuestItem::AncientKey => "Ancient Key",
            QuestItem::MysteriousMap => "Mysterious Map",
            QuestItem::DragonScale => "Dragon Scale",
            QuestItem::RoyalSeal => "Royal Seal",
        }
    }

    fn description(&self) -> &str {
        match self {
            QuestItem::AncientKey => "An ornate key from a forgotten civilization.",
            QuestItem::MysteriousMap => "A map showing the location of hidden treasure.",
            QuestItem::DragonScale => "A scale from the legendary dragon Fyrnax.",
            QuestItem::RoyalSeal => "The official seal of the kingdom.",
        }
    }
}

// =============================================================================
// InventoryDraggable Implementations (only for spells)
// =============================================================================

impl InventoryDraggable for Spell {}

// =============================================================================
// InventoryCounted Implementations
// =============================================================================

impl InventoryCounted for Perk {
    type Count = u8; // Perk levels (1-10)
}

impl InventoryCounted for Consumable {
    type Count = u32; // Stack counts
}

// =============================================================================
// InventoryItemColor Implementations
// =============================================================================

impl InventoryItemColor for Spell {
    fn card_color(&self) -> InventoryCardColor {
        match self {
            Spell::Fireball | Spell::LightningBolt => InventoryCardColor::Red,
            Spell::IceShield => InventoryCardColor::Blue,
            Spell::Heal => InventoryCardColor::Green,
            Spell::Teleport => InventoryCardColor::Purple,
        }
    }
}

impl InventoryItemColor for Perk {
    fn card_color(&self) -> InventoryCardColor {
        InventoryCardColor::Purple
    }
}

impl InventoryItemColor for Consumable {
    fn card_color(&self) -> InventoryCardColor {
        InventoryCardColor::Green
    }
}

impl InventoryItemColor for QuestItem {
    fn card_color(&self) -> InventoryCardColor {
        InventoryCardColor::Yellow
    }
}

// =============================================================================
// Inventory Storage Components
// =============================================================================

/// Stores the count/level for counted items.
#[derive(Component)]
struct ItemCount<T: InventoryCounted>(T::Count);

/// Marker component for items in the player's inventory.
#[derive(Component)]
struct InPlayerInventory;

// =============================================================================
// Demo Application
// =============================================================================

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_inventory)
        .add_systems(Update, display_inventory)
        .run();
}

fn setup_inventory(mut commands: Commands) {
    println!("=== Inventory Demo ===\n");
    println!("Setting up player inventory with various item types...\n");

    // Add some spells (draggable items)
    commands.spawn((Spell::Fireball, InPlayerInventory));
    commands.spawn((Spell::IceShield, InPlayerInventory));
    commands.spawn((Spell::Heal, InPlayerInventory));

    // Add some perks with levels
    commands.spawn((Perk::HealthBoost, ItemCount::<Perk>(3), InPlayerInventory));
    commands.spawn((
        Perk::CriticalStrike,
        ItemCount::<Perk>(5),
        InPlayerInventory,
    ));

    // Add some consumables with stack counts
    commands.spawn((
        Consumable::HealthPotion,
        ItemCount::<Consumable>(10),
        InPlayerInventory,
    ));
    commands.spawn((
        Consumable::ManaPotion,
        ItemCount::<Consumable>(5),
        InPlayerInventory,
    ));

    // Add a quest item
    commands.spawn((QuestItem::AncientKey, InPlayerInventory));
}

fn display_inventory(
    spells: Query<&Spell, With<InPlayerInventory>>,
    perks: Query<(&Perk, &ItemCount<Perk>), With<InPlayerInventory>>,
    consumables: Query<(&Consumable, &ItemCount<Consumable>), With<InPlayerInventory>>,
    quest_items: Query<&QuestItem, With<InPlayerInventory>>,
    mut exit: MessageWriter<AppExit>,
) {
    // Display Spells
    println!("--- SPELLS (Draggable) ---");
    for spell in spells.iter() {
        print_item_info(spell);
        println!("  Draggable: Yes");
        println!();
    }

    // Display Perks
    println!("--- PERKS (Leveled) ---");
    for (perk, count) in perks.iter() {
        print_item_info(perk);
        println!("  Level: {}", count.0);
        println!();
    }

    // Display Consumables
    println!("--- CONSUMABLES (Stacked) ---");
    for (consumable, count) in consumables.iter() {
        print_item_info(consumable);
        println!("  Count: {}", count.0);
        println!();
    }

    // Display Quest Items
    println!("--- QUEST ITEMS ---");
    for quest_item in quest_items.iter() {
        print_item_info(quest_item);
        println!();
    }

    println!("=== Demo Complete ===");
    exit.write(AppExit::Success);
}

/// Helper function to print common item information.
fn print_item_info<T>(item: &T)
where
    T: InventoryItem + InventoryItemData + InventoryItemColor,
{
    println!("[{:?}] {}", item.card_color(), item.display_name());
    println!("  Description: {}", item.description());
    println!("  Icon: {}", item.icon_path());
}
