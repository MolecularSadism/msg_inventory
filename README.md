# msg_inventory

Inventory system traits for displaying enum-based items in game inventories.

This crate provides traits that can be implemented on enums to make them displayable in inventory UIs. The design is data-driven and supports various item types with different display behaviors.

## Features

- **Base display**: Icon paths and display names via `InventoryItem`
- **Visibility control**: Per-item hiding for dev-only items via `InventoryItemData`
- **Draggable items**: Spell-like items that can be dragged via `InventoryDraggable`
- **Counted items**: Perks and consumables with quantity display via `InventoryCounted`
- **Card colors**: Visual distinction between item types via `InventoryItemColor`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
msg_inventory = "0.2"
bevy = "0.17"
```

## Quick Start

```rust
use msg_inventory::prelude::*;

#[derive(Clone, Copy)]
enum MyItem {
    HealthPotion,
    ManaPotion,
}

impl InventoryItem for MyItem {
    fn icon_path(&self) -> &str {
        match self {
            MyItem::HealthPotion => "images/icons/items/health_potion.png",
            MyItem::ManaPotion => "images/icons/items/mana_potion.png",
        }
    }
}
```

## Traits

### `InventoryItem`

Core trait for items that can be displayed in an inventory grid. Items must provide an icon path for display.

```rust
pub trait InventoryItem: Copy + Clone + Send + Sync + 'static {
    fn icon_path(&self) -> &str;
}
```

### `InventoryItemData`

Trait for accessing item metadata that may include visibility control.

```rust
pub trait InventoryItemData {
    fn display_name(&self) -> &str;
    fn description(&self) -> &str;
    fn is_hidden(&self) -> bool { false }
}
```

### `InventoryDraggable`

Marker trait for items that can be dragged in the inventory. Useful for spell-like items that can be rearranged.

```rust
pub trait InventoryDraggable: InventoryItem {}
```

### `InventoryCounted`

Trait for items that display a count/quantity. Useful for perks (showing level) or consumables (showing stack count).

```rust
pub trait InventoryCounted: InventoryItem {
    type Count: Copy + Clone + Send + Sync + std::fmt::Display + 'static;
}
```

### `InventoryItemColor`

Trait for items with custom card colors for visual distinction.

```rust
pub trait InventoryItemColor: InventoryItem {
    fn card_color(&self) -> InventoryCardColor {
        InventoryCardColor::default()
    }
}
```

## Card Colors

Available colors for inventory item cards:

| Color | Use Case |
|-------|----------|
| `Red` | Projectile/attack spells |
| `Blue` | Modifier spells (default) |
| `Purple` | Perks |
| `Green` | Consumable items |
| `Yellow` | Quest items |

## Complete Example

```rust
use msg_inventory::prelude::*;

#[derive(Clone, Copy)]
enum Spell {
    Fireball,
    IceShield,
}

impl InventoryItem for Spell {
    fn icon_path(&self) -> &str {
        match self {
            Spell::Fireball => "images/spells/fireball.png",
            Spell::IceShield => "images/spells/ice_shield.png",
        }
    }
}

impl InventoryDraggable for Spell {}

impl InventoryItemColor for Spell {
    fn card_color(&self) -> InventoryCardColor {
        match self {
            Spell::Fireball => InventoryCardColor::Red,
            Spell::IceShield => InventoryCardColor::Blue,
        }
    }
}
```

## Bevy Version Compatibility

| `msg_inventory` | Bevy |
|-----------------|------|
| 0.2             | 0.17 |
| 0.1             | 0.16 |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! This crate is part of the [MolecularSadism](https://github.com/MolecularSadism) game development libraries.
