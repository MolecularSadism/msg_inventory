//! Inventory system traits for displaying enum-based items in game inventories.
//!
//! This crate provides traits that can be implemented on enums to make them
//! displayable in inventory UIs. The design is data-driven and supports:
//!
//! - **Base display**: Icon paths and display names via [`InventoryItem`]
//! - **Visibility control**: Per-item hiding for dev-only items via [`InventoryItemData`]
//! - **Draggable items**: Spell-like items that can be dragged via [`InventoryDraggable`]
//! - **Counted items**: Perks and consumables with quantity display via [`InventoryCounted`]
//!
//! ## Example
//!
//! ```rust
//! use msg_inventory::prelude::*;
//!
//! #[derive(Clone, Copy)]
//! enum MyItem {
//!     HealthPotion,
//!     ManaPotion,
//! }
//!
//! impl InventoryItem for MyItem {
//!     fn icon_path(&self) -> &str {
//!         match self {
//!             MyItem::HealthPotion => "images/icons/items/health_potion.png",
//!             MyItem::ManaPotion => "images/icons/items/mana_potion.png",
//!         }
//!     }
//! }
//! ```

/// Core trait for items that can be displayed in an inventory grid.
///
/// This trait defines the minimum requirements for an enum to be shown
/// in the inventory UI. Items must provide an icon path for display.
///
/// The [`InventoryItemData`] associated type allows accessing additional
/// metadata like display names, descriptions, and visibility flags that
/// may be loaded from asset files.
pub trait InventoryItem: Copy + Clone + Send + Sync + 'static {
    /// Returns the asset path to the icon image for this item.
    fn icon_path(&self) -> &str;
}

/// Trait for accessing item metadata that may include visibility control.
///
/// This is separate from [`InventoryItem`] because the hidden status
/// is typically stored in loaded asset data (like RON files), not on
/// the enum variant itself.
pub trait InventoryItemData {
    /// Returns the display name of this item.
    fn display_name(&self) -> &str;

    /// Returns the description of this item.
    fn description(&self) -> &str;

    /// Returns whether this item should be hidden from the inventory
    /// unless the `dev` feature is enabled.
    ///
    /// Hidden items are typically debug/test items that shouldn't
    /// appear in production builds.
    fn is_hidden(&self) -> bool {
        false
    }
}

/// Marker trait for items that can be dragged in the inventory.
///
/// Currently only spells support drag-and-drop to arrange them
/// in wand spell stacks. Perks and consumable items are not draggable.
///
/// Implementing this trait enables the drag-and-drop system for the item type.
pub trait InventoryDraggable: InventoryItem {}

/// Trait for items that display a count/quantity in the inventory.
///
/// Perks show their level (e.g., "3" for level 3), while consumable
/// items show their stack count. Spells are typically unlimited and
/// don't implement this trait.
pub trait InventoryCounted: InventoryItem {
    /// The type used to store the count for this item type.
    /// Usually `u32` for perk levels or item stack counts.
    type Count: Copy + Clone + Send + Sync + std::fmt::Display + 'static;
}

/// Card color for inventory item display.
///
/// Different item types use different colors to help players
/// distinguish between them at a glance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InventoryCardColor {
    /// Red color for projectile/attack spells
    Red,
    /// Blue color for modifier spells
    #[default]
    Blue,
    /// Purple color for perks
    Purple,
    /// Green color for consumable items
    Green,
    /// Yellow color for quest items
    Yellow,
}

/// Trait for items that have a custom card color.
///
/// Items that don't implement this trait will use the default blue color.
pub trait InventoryItemColor: InventoryItem {
    /// Returns the card color for this item.
    fn card_color(&self) -> InventoryCardColor {
        InventoryCardColor::default()
    }
}

/// Prelude module for convenient imports.
pub mod prelude {
    pub use super::{
        InventoryCardColor, InventoryCounted, InventoryDraggable, InventoryItem,
        InventoryItemColor, InventoryItemData,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test item enum for comprehensive trait testing
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestSpell {
        Fireball,
        IceShield,
        Lightning,
        HiddenDebugSpell,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestPerk {
        Health,
        Speed,
        Damage,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestConsumable {
        HealthPotion,
        ManaPotion,
        Antidote,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestQuestItem {
        AncientKey,
        MysteriousMap,
    }

    // InventoryItem implementations
    impl InventoryItem for TestSpell {
        fn icon_path(&self) -> &str {
            match self {
                TestSpell::Fireball => "icons/spells/fireball.png",
                TestSpell::IceShield => "icons/spells/ice_shield.png",
                TestSpell::Lightning => "icons/spells/lightning.png",
                TestSpell::HiddenDebugSpell => "icons/spells/debug.png",
            }
        }
    }

    impl InventoryItem for TestPerk {
        fn icon_path(&self) -> &str {
            match self {
                TestPerk::Health => "icons/perks/health.png",
                TestPerk::Speed => "icons/perks/speed.png",
                TestPerk::Damage => "icons/perks/damage.png",
            }
        }
    }

    impl InventoryItem for TestConsumable {
        fn icon_path(&self) -> &str {
            match self {
                TestConsumable::HealthPotion => "icons/items/health_potion.png",
                TestConsumable::ManaPotion => "icons/items/mana_potion.png",
                TestConsumable::Antidote => "icons/items/antidote.png",
            }
        }
    }

    impl InventoryItem for TestQuestItem {
        fn icon_path(&self) -> &str {
            match self {
                TestQuestItem::AncientKey => "icons/quest/ancient_key.png",
                TestQuestItem::MysteriousMap => "icons/quest/map.png",
            }
        }
    }

    // InventoryItemData implementations
    impl InventoryItemData for TestSpell {
        fn display_name(&self) -> &str {
            match self {
                TestSpell::Fireball => "Fireball",
                TestSpell::IceShield => "Ice Shield",
                TestSpell::Lightning => "Lightning Bolt",
                TestSpell::HiddenDebugSpell => "Debug Spell",
            }
        }

        fn description(&self) -> &str {
            match self {
                TestSpell::Fireball => "Launches a ball of fire at enemies.",
                TestSpell::IceShield => "Creates a protective shield of ice.",
                TestSpell::Lightning => "Strikes enemies with lightning.",
                TestSpell::HiddenDebugSpell => "A debug spell for testing.",
            }
        }

        fn is_hidden(&self) -> bool {
            matches!(self, TestSpell::HiddenDebugSpell)
        }
    }

    impl InventoryItemData for TestPerk {
        fn display_name(&self) -> &str {
            match self {
                TestPerk::Health => "Health Boost",
                TestPerk::Speed => "Speed Boost",
                TestPerk::Damage => "Damage Boost",
            }
        }

        fn description(&self) -> &str {
            match self {
                TestPerk::Health => "Increases maximum health.",
                TestPerk::Speed => "Increases movement speed.",
                TestPerk::Damage => "Increases damage dealt.",
            }
        }
    }

    // InventoryDraggable implementations (spells are draggable)
    impl InventoryDraggable for TestSpell {}

    // InventoryCounted implementations
    impl InventoryCounted for TestPerk {
        type Count = u8; // Perk levels 1-10
    }

    impl InventoryCounted for TestConsumable {
        type Count = u32; // Stack counts
    }

    // InventoryItemColor implementations
    impl InventoryItemColor for TestSpell {
        fn card_color(&self) -> InventoryCardColor {
            match self {
                TestSpell::Fireball | TestSpell::Lightning => InventoryCardColor::Red,
                TestSpell::IceShield => InventoryCardColor::Blue,
                TestSpell::HiddenDebugSpell => InventoryCardColor::Blue,
            }
        }
    }

    impl InventoryItemColor for TestPerk {
        fn card_color(&self) -> InventoryCardColor {
            InventoryCardColor::Purple
        }
    }

    impl InventoryItemColor for TestConsumable {
        fn card_color(&self) -> InventoryCardColor {
            InventoryCardColor::Green
        }
    }

    impl InventoryItemColor for TestQuestItem {
        fn card_color(&self) -> InventoryCardColor {
            InventoryCardColor::Yellow
        }
    }

    // =========================================================================
    // InventoryItem trait tests
    // =========================================================================

    #[test]
    fn test_inventory_item_icon_path() {
        assert_eq!(TestSpell::Fireball.icon_path(), "icons/spells/fireball.png");
        assert_eq!(TestPerk::Health.icon_path(), "icons/perks/health.png");
        assert_eq!(
            TestConsumable::HealthPotion.icon_path(),
            "icons/items/health_potion.png"
        );
        assert_eq!(
            TestQuestItem::AncientKey.icon_path(),
            "icons/quest/ancient_key.png"
        );
    }

    #[test]
    fn test_inventory_item_is_copy() {
        let spell = TestSpell::Fireball;
        let spell_copy = spell; // Copy
        assert_eq!(spell, spell_copy);
    }

    #[test]
    fn test_inventory_item_is_clone() {
        let spell = TestSpell::Fireball;
        let spell_clone = spell;
        assert_eq!(spell, spell_clone);
    }

    // =========================================================================
    // InventoryItemData trait tests
    // =========================================================================

    #[test]
    fn test_inventory_item_data_display_name() {
        assert_eq!(TestSpell::Fireball.display_name(), "Fireball");
        assert_eq!(TestSpell::IceShield.display_name(), "Ice Shield");
        assert_eq!(TestPerk::Health.display_name(), "Health Boost");
    }

    #[test]
    fn test_inventory_item_data_description() {
        assert_eq!(
            TestSpell::Fireball.description(),
            "Launches a ball of fire at enemies."
        );
        assert_eq!(TestPerk::Speed.description(), "Increases movement speed.");
    }

    #[test]
    fn test_inventory_item_data_is_hidden() {
        assert!(!TestSpell::Fireball.is_hidden());
        assert!(!TestSpell::IceShield.is_hidden());
        assert!(TestSpell::HiddenDebugSpell.is_hidden());
    }

    #[test]
    fn test_inventory_item_data_default_is_hidden() {
        // TestPerk doesn't override is_hidden, so it should use the default (false)
        assert!(!TestPerk::Health.is_hidden());
    }

    // =========================================================================
    // InventoryDraggable trait tests
    // =========================================================================

    #[test]
    fn test_draggable_items_are_inventory_items() {
        fn accepts_draggable<T: InventoryDraggable>(_item: T) {}

        // This compiles, proving TestSpell implements InventoryDraggable
        accepts_draggable(TestSpell::Fireball);
    }

    #[test]
    fn test_draggable_item_icon_path() {
        // Draggable items should still have icon_path from InventoryItem
        fn check_icon<T: InventoryDraggable>(item: T, expected: &str) {
            assert_eq!(item.icon_path(), expected);
        }

        check_icon(TestSpell::Fireball, "icons/spells/fireball.png");
    }

    // =========================================================================
    // InventoryCounted trait tests
    // =========================================================================

    #[test]
    fn test_counted_item_count_type() {
        fn accepts_counted<T: InventoryCounted>(_item: T)
        where
            T::Count: std::fmt::Display,
        {
        }

        accepts_counted(TestPerk::Health);
        accepts_counted(TestConsumable::HealthPotion);
    }

    #[test]
    fn test_perk_count_type_is_u8() {
        fn get_count_type_name<T: InventoryCounted>(_item: T) -> &'static str {
            std::any::type_name::<T::Count>()
        }

        assert_eq!(get_count_type_name(TestPerk::Health), "u8");
    }

    #[test]
    fn test_consumable_count_type_is_u32() {
        fn get_count_type_name<T: InventoryCounted>(_item: T) -> &'static str {
            std::any::type_name::<T::Count>()
        }

        assert_eq!(get_count_type_name(TestConsumable::HealthPotion), "u32");
    }

    #[test]
    fn test_count_type_display() {
        // Verify count types implement Display
        fn format_count<T: InventoryCounted>(count: T::Count) -> String {
            format!("{}", count)
        }

        let perk_count: <TestPerk as InventoryCounted>::Count = 5;
        assert_eq!(format_count::<TestPerk>(perk_count), "5");

        let consumable_count: <TestConsumable as InventoryCounted>::Count = 99;
        assert_eq!(format_count::<TestConsumable>(consumable_count), "99");
    }

    // =========================================================================
    // InventoryCardColor tests
    // =========================================================================

    #[test]
    fn test_card_color_default() {
        assert_eq!(InventoryCardColor::default(), InventoryCardColor::Blue);
    }

    #[test]
    fn test_card_color_equality() {
        assert_eq!(InventoryCardColor::Red, InventoryCardColor::Red);
        assert_ne!(InventoryCardColor::Red, InventoryCardColor::Blue);
    }

    #[test]
    fn test_card_color_debug() {
        assert_eq!(format!("{:?}", InventoryCardColor::Red), "Red");
        assert_eq!(format!("{:?}", InventoryCardColor::Purple), "Purple");
    }

    #[test]
    fn test_card_color_clone() {
        let color = InventoryCardColor::Green;
        let cloned = color;
        assert_eq!(color, cloned);
    }

    #[test]
    fn test_card_color_copy() {
        let color = InventoryCardColor::Yellow;
        let copied = color; // Copy
        assert_eq!(color, copied);
    }

    #[test]
    fn test_card_color_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(InventoryCardColor::Red);
        set.insert(InventoryCardColor::Blue);
        set.insert(InventoryCardColor::Red); // Duplicate

        assert_eq!(set.len(), 2);
    }

    // =========================================================================
    // InventoryItemColor trait tests
    // =========================================================================

    #[test]
    fn test_spell_card_colors() {
        assert_eq!(TestSpell::Fireball.card_color(), InventoryCardColor::Red);
        assert_eq!(TestSpell::IceShield.card_color(), InventoryCardColor::Blue);
        assert_eq!(TestSpell::Lightning.card_color(), InventoryCardColor::Red);
    }

    #[test]
    fn test_perk_card_color() {
        assert_eq!(TestPerk::Health.card_color(), InventoryCardColor::Purple);
        assert_eq!(TestPerk::Speed.card_color(), InventoryCardColor::Purple);
    }

    #[test]
    fn test_consumable_card_color() {
        assert_eq!(
            TestConsumable::HealthPotion.card_color(),
            InventoryCardColor::Green
        );
    }

    #[test]
    fn test_quest_item_card_color() {
        assert_eq!(
            TestQuestItem::AncientKey.card_color(),
            InventoryCardColor::Yellow
        );
    }

    #[test]
    fn test_item_color_default_implementation() {
        // Create a type that uses the default card_color implementation
        #[derive(Clone, Copy)]
        struct DefaultColorItem;

        impl InventoryItem for DefaultColorItem {
            fn icon_path(&self) -> &str {
                "icons/default.png"
            }
        }

        impl InventoryItemColor for DefaultColorItem {}

        assert_eq!(DefaultColorItem.card_color(), InventoryCardColor::Blue);
    }

    // =========================================================================
    // Trait bound verification tests (compile-time checks)
    // =========================================================================

    #[test]
    fn test_send_sync_bounds() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<TestSpell>();
        assert_sync::<TestSpell>();
        assert_send::<TestPerk>();
        assert_sync::<TestPerk>();
        assert_send::<TestConsumable>();
        assert_sync::<TestConsumable>();
        assert_send::<TestQuestItem>();
        assert_sync::<TestQuestItem>();
        assert_send::<InventoryCardColor>();
        assert_sync::<InventoryCardColor>();
    }

    #[test]
    fn test_static_bounds() {
        fn assert_static<T: 'static>() {}

        assert_static::<TestSpell>();
        assert_static::<TestPerk>();
        assert_static::<TestConsumable>();
        assert_static::<TestQuestItem>();
        assert_static::<InventoryCardColor>();
    }

    // =========================================================================
    // Prelude tests
    // =========================================================================

    #[test]
    fn test_prelude_exports() {
        // Verify all types are accessible via prelude
        use crate::prelude::*;

        let _: InventoryCardColor = InventoryCardColor::Blue;

        // Trait bounds check - these compile if prelude exports correctly
        fn _check_traits<
            T: InventoryItem
                + InventoryItemData
                + InventoryDraggable
                + InventoryCounted
                + InventoryItemColor,
        >() {
        }
    }

    // =========================================================================
    // Integration-style tests
    // =========================================================================

    #[test]
    fn test_complete_inventory_item_workflow() {
        // Simulates how an inventory system would use these traits
        let spell = TestSpell::Fireball;

        // Get basic info
        let icon = spell.icon_path();
        let name = spell.display_name();
        let desc = spell.description();
        let color = spell.card_color();
        let hidden = spell.is_hidden();

        assert_eq!(icon, "icons/spells/fireball.png");
        assert_eq!(name, "Fireball");
        assert_eq!(desc, "Launches a ball of fire at enemies.");
        assert_eq!(color, InventoryCardColor::Red);
        assert!(!hidden);
    }

    #[test]
    fn test_counted_item_workflow() {
        let perk = TestPerk::Health;

        // In a real system, you'd store the count separately
        let level: <TestPerk as InventoryCounted>::Count = 3;

        // Format for display
        let display = format!("Lv. {}", level);
        assert_eq!(display, "Lv. 3");

        // Verify the perk itself
        assert_eq!(perk.icon_path(), "icons/perks/health.png");
        assert_eq!(perk.card_color(), InventoryCardColor::Purple);
    }

    #[test]
    fn test_generic_inventory_display() {
        // Test a generic function that works with any inventory item
        fn render_item_card<T: InventoryItem + InventoryItemColor>(item: T) -> String {
            format!("Icon: {}, Color: {:?}", item.icon_path(), item.card_color())
        }

        let spell_card = render_item_card(TestSpell::Fireball);
        assert!(spell_card.contains("icons/spells/fireball.png"));
        assert!(spell_card.contains("Red"));

        let perk_card = render_item_card(TestPerk::Health);
        assert!(perk_card.contains("icons/perks/health.png"));
        assert!(perk_card.contains("Purple"));
    }

    #[test]
    fn test_filter_hidden_items() {
        fn filter_visible<T: InventoryItemData + Copy>(items: &[T]) -> Vec<T> {
            items
                .iter()
                .filter(|item| !item.is_hidden())
                .copied()
                .collect()
        }

        let spells = [
            TestSpell::Fireball,
            TestSpell::IceShield,
            TestSpell::HiddenDebugSpell,
            TestSpell::Lightning,
        ];

        let visible = filter_visible(&spells);
        assert_eq!(visible.len(), 3);
        assert!(!visible.contains(&TestSpell::HiddenDebugSpell));
    }
}
