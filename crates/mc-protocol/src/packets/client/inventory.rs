//! Clientbound containers, items, trades and recipes for protocol 777.
//!
//! Item stacks (Slot), recipe displays and slot displays are not decoded:
//! their components have no length prefix, so a packet keeps everything
//! from the first of them on as raw bytes, which re-encode unchanged.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::VarInt;
use crate::types::bytes::Remainder;

/// Replaces every slot of a window.
#[packet(Play, 0x12, Clientbound)]
pub struct SetContainerContent<'p> {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub state_id: i32,
    /// The prefixed array of slots, then the carried slot; not decoded.
    pub slots: Remainder<'p>,
}

/// A window-specific value, such as furnace progress or an enchantment cost.
#[packet(Play, 0x13, Clientbound)]
pub struct SetContainerProperty {
    #[format = "varint"]
    pub window_id: i32,
    pub property: i16,
    pub value: i16,
}

#[packet(Play, 0x14, Clientbound)]
pub struct SetContainerSlot<'p> {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub state_id: i32,
    pub slot: i16,
    /// The slot's item stack; not decoded.
    pub item: Remainder<'p>,
}

/// The item stack under the mouse in an open window.
#[packet(Play, 0x62, Clientbound)]
pub struct SetCursorItem<'p> {
    /// The item stack; not decoded.
    pub item: Remainder<'p>,
}

/// Uses player inventory indices, not those of any window.
#[packet(Play, 0x6E, Clientbound)]
pub struct SetPlayerInventorySlot<'p> {
    #[format = "varint"]
    pub slot: i32,
    /// The item stack; not decoded.
    pub item: Remainder<'p>,
}

/// `group` is an item identifier or a `use_cooldown` group; zero ticks clear it.
#[packet(Play, 0x16, Clientbound)]
pub struct SetCooldown<'p> {
    pub group: Cow<'p, str>,
    #[format = "varint"]
    pub ticks: i32,
}

#[packet(Play, 0x2A, Clientbound)]
pub struct OpenHorseScreen {
    #[format = "varint"]
    pub window_id: i32,
    /// Columns of three inventory slots.
    #[format = "varint"]
    pub columns: i32,
    pub entity_id: i32,
}

#[packet(Play, 0x35, Clientbound)]
pub struct MerchantOffers<'p> {
    #[format = "varint"]
    pub window_id: i32,
    /// The trades, then the villager's level, experience and flags; not decoded.
    pub offers: Remainder<'p>,
}

/// Opens the written book in the given [`InteractionHand`](super::InteractionHand).
#[packet(Play, 0x3B, Clientbound)]
pub struct OpenBook {
    #[format = "varint"]
    pub hand: i32,
}

/// `SignText` sides.
pub mod sign_side {
    pub const BACK: i32 = 0;
    pub const FRONT: i32 = 1;
}

#[packet(Play, 0x3D, Clientbound)]
pub struct OpenSignEditor {
    pub location: i64,
    /// A [`sign_side`] value.
    #[format = "varint"]
    pub side: i32,
}

#[packet(Play, 0x40, Clientbound)]
pub struct PlaceGhostRecipe<'p> {
    #[format = "varint"]
    pub window_id: i32,
    /// The recipe display; not decoded.
    pub recipe: Remainder<'p>,
}

#[packet(Play, 0x4B, Clientbound)]
pub struct RecipeBookAdd<'p> {
    /// The prefixed array of recipes, each starting with a recipe display,
    /// then whether they replace the known recipes; not decoded.
    pub entries: Remainder<'p>,
}

#[packet(Play, 0x4C, Clientbound)]
pub struct RecipeBookRemove {
    pub recipes: Vec<VarInt>,
}

/// Whether each recipe book is open and filtering craftable recipes.
#[packet(Play, 0x4D, Clientbound)]
pub struct RecipeBookSettings {
    pub crafting_open: bool,
    pub crafting_filtering: bool,
    pub furnace_open: bool,
    pub furnace_filtering: bool,
    pub blast_furnace_open: bool,
    pub blast_furnace_filtering: bool,
    pub smoker_open: bool,
    pub smoker_filtering: bool,
}

/// Items accepted by an input slot, such as furnace fuels.
#[derive(Debug, IntoOwned)]
pub struct PropertySet<'p> {
    pub id: Cow<'p, str>,
    /// IDs in the `minecraft:item` registry.
    pub items: Vec<VarInt>,
}

impl<'p> PacketWriter<'p> for PropertySet<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let id = Cow::read(input)?;
        let items = Vec::read(input)?;

        Ok(Self { id, items })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.id.encode(out)?;
        self.items.encode(out)
    }

    fn body_len(&self) -> usize {
        self.id.body_len() + self.items.body_len()
    }
}

#[packet(Play, 0x88, Clientbound)]
pub struct UpdateRecipes<'p> {
    pub property_sets: Vec<PropertySet<'p>>,
    /// The prefixed array of stonecutter recipes, each an ID set and a slot
    /// display; not decoded.
    pub stonecutter_recipes: Remainder<'p>,
}
