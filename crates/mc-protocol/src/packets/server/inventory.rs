//! Serverbound containers, items, trades, books, signs and recipes for
//! protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::packet;

use crate::types::VarInt;
use crate::types::bytes::Remainder;

/// A window button: an enchantment, a lectern page, a stonecutter or loom recipe.
#[packet(Play, 0x11, Serverbound)]
pub struct ClickContainerButton {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub button_id: i32,
}

/// Toggles a crafter slot.
#[packet(Play, 0x14, Serverbound)]
pub struct ChangeContainerSlotState {
    #[format = "varint"]
    pub slot: i32,
    #[format = "varint"]
    pub window_id: i32,
    pub enabled: bool,
}

/// A creative inventory change, including middle-click picks.
#[packet(Play, 0x39, Serverbound)]
pub struct SetCreativeModeSlot<'p> {
    pub slot: i16,
    /// The item stack; not decoded.
    pub item: Remainder<'p>,
}

#[packet(Play, 0x24, Serverbound)]
pub struct PickItemFromBlock {
    pub location: i64,
    /// Creative mode only: copy the block entity's data into the stack.
    pub include_data: bool,
}

#[packet(Play, 0x25, Serverbound)]
pub struct PickItemFromEntity {
    #[format = "varint"]
    pub entity_id: i32,
    pub include_data: bool,
}

/// A recipe book click; `make_all` is set with shift.
#[packet(Play, 0x27, Serverbound)]
pub struct PlaceRecipe {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub recipe_id: i32,
    pub make_all: bool,
}

/// `RecipeBookType` ordinals.
pub mod recipe_book {
    pub const CRAFTING: i32 = 0;
    pub const FURNACE: i32 = 1;
    pub const BLAST_FURNACE: i32 = 2;
    pub const SMOKER: i32 = 3;
}

#[packet(Play, 0x2F, Serverbound)]
pub struct ChangeRecipeBookSettings {
    /// A [`recipe_book`] value.
    #[format = "varint"]
    pub book: i32,
    pub open: bool,
    pub filtering: bool,
}

#[packet(Play, 0x30, Serverbound)]
pub struct SetSeenRecipe {
    #[format = "varint"]
    pub recipe_id: i32,
}

/// The anvil's name field.
#[packet(Play, 0x31, Serverbound)]
pub struct RenameItem<'p> {
    pub name: Cow<'p, str>,
}

#[packet(Play, 0x34, Serverbound)]
pub struct SelectTrade {
    #[format = "varint"]
    pub slot: i32,
}

/// Effects are IDs in the `minecraft:mob_effect` registry.
#[packet(Play, 0x35, Serverbound)]
pub struct SetBeaconEffect {
    pub primary: Option<VarInt>,
    pub secondary: Option<VarInt>,
}

/// Edits a book and quill, or signs it when `title` is present.
#[packet(Play, 0x18, Serverbound)]
pub struct EditBook<'p> {
    /// The hotbar slot holding the book.
    #[format = "varint"]
    pub slot: i32,
    pub pages: Vec<Cow<'p, str>>,
    pub title: Option<Cow<'p, str>>,
}

#[packet(Play, 0x3E, Serverbound)]
pub struct UpdateSign<'p> {
    pub location: i64,
    pub line_1: Cow<'p, str>,
    pub line_2: Cow<'p, str>,
    pub line_3: Cow<'p, str>,
    pub line_4: Cow<'p, str>,
    /// A [`sign_side`](crate::packets::client::sign_side) value.
    #[format = "varint"]
    pub side: i32,
}
