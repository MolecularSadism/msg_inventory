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
