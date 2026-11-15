//! Item stacks and the data components that patch their item's defaults.
//!
//! The same stack has three wire forms: [`Slot`] in packets and entity
//! metadata, [`UntrustedSlot`] from the client in creative mode, and
//! [`ItemStackTemplate`] nested inside components and particles.
mod component;
mod components;

use mc_protocol_macros::IntoOwned;

pub use component::{Component, component_type};
pub use components::*;

use crate::decode::{MAX_COLLECTION_LEN, Result as DecodeResult, read_length, take};
use crate::encode::Result as EncodeResult;
use crate::error::{DecodeError, EncodeError};
use crate::packets::PacketWriter;
use crate::types::VarInt;

/// A non-empty stack: `count` of the `minecraft:item` registry entry `item`.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct ItemStack<'p> {
    pub item: i32,
    pub count: i32,
    pub components: ComponentPatch<'p>,
}

impl ItemStack<'_> {
    /// A stack of `item` with the item's default components.
    pub fn new(item: i32, count: i32) -> Self {
        Self { item, count, components: ComponentPatch::default() }
    }

    /// The added component with this [`component_type`], if any.
    pub fn component(&self, component_type: i32) -> Option<&Component<'_>> {
        self.components.added.iter().find(|component| component.id() == component_type)
    }
}

/// A slot's item stack, if any: a count, then the item and its components
/// when the count is positive. A zero count is an empty slot.
#[derive(Clone, Debug, Default, PartialEq, IntoOwned)]
pub struct Slot<'p>(pub Option<ItemStack<'p>>);

impl<'p> PacketWriter<'p> for Slot<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        read_slot(input, Framing::Plain).map(Self)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        encode_slot(self.0.as_ref(), out, Framing::Plain)
    }

    fn body_len(&self) -> usize {
        slot_len(self.0.as_ref(), Framing::Plain)
    }
}

/// A slot as the client sends it in creative mode: each added component's
/// value is prefixed with its length, so the server can skip invalid ones.
#[derive(Clone, Debug, Default, PartialEq, IntoOwned)]
pub struct UntrustedSlot<'p>(pub Option<ItemStack<'p>>);

impl<'p> PacketWriter<'p> for UntrustedSlot<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        read_slot(input, Framing::Delimited).map(Self)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        encode_slot(self.0.as_ref(), out, Framing::Delimited)
    }

    fn body_len(&self) -> usize {
        slot_len(self.0.as_ref(), Framing::Delimited)
    }
}

/// A stack nested in a component or particle: the item comes first, and
/// the stack is never empty.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct ItemStackTemplate<'p>(pub ItemStack<'p>);

impl<'p> PacketWriter<'p> for ItemStackTemplate<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let item = VarInt::read(input)?.0;
        let count = VarInt::read(input)?.0;
        let components = ComponentPatch::read(input)?;

        Ok(Self(ItemStack { item, count, components }))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let stack = &self.0;
        VarInt(stack.item).encode(out)?;
        VarInt(stack.count).encode(out)?;

        stack.components.encode(out)
    }

    fn body_len(&self) -> usize {
        let stack = &self.0;

        VarInt(stack.item).body_len() + VarInt(stack.count).body_len() + stack.components.body_len()
    }
}

/// How an added component's value is framed in a patch.
#[derive(Clone, Copy)]
enum Framing {
    Plain,
    /// Each value is prefixed with its length.
    Delimited,
}

fn read_slot<'p>(input: &mut &'p [u8], framing: Framing) -> DecodeResult<Option<ItemStack<'p>>> {
    let count = VarInt::read(input)?.0;
    if count <= 0 {
        return Ok(None);
    }

    let item = VarInt::read(input)?.0;
    let components = ComponentPatch::read_framed(input, framing)?;

    Ok(Some(ItemStack { item, count, components }))
}

fn encode_slot(stack: Option<&ItemStack>, out: &mut Vec<u8>, framing: Framing) -> EncodeResult {
    let Some(stack) = stack else {
        return VarInt(0).encode(out);
    };

    VarInt(stack.count).encode(out)?;
    VarInt(stack.item).encode(out)?;

    stack.components.encode_framed(out, framing)
}

fn slot_len(stack: Option<&ItemStack>, framing: Framing) -> usize {
    let Some(stack) = stack else {
        return 1;
    };

    VarInt(stack.count).body_len() + VarInt(stack.item).body_len() + stack.components.framed_len(framing)
}

/// The changes to an item's default components.
///
/// On the wire: the number of added and of removed components, then each
/// added component, then each removed [`component_type`].
#[derive(Clone, Debug, Default, PartialEq, IntoOwned)]
pub struct ComponentPatch<'p> {
    pub added: Vec<Component<'p>>,
    /// [`component_type`] IDs the item no longer has.
    pub removed: Vec<VarInt>,
}

impl<'p> ComponentPatch<'p> {
    fn read_framed(input: &mut &'p [u8], framing: Framing) -> DecodeResult<Self> {
        let added_count = read_length(input, MAX_COLLECTION_LEN)?;
        let removed_count = read_length(input, MAX_COLLECTION_LEN)?;

        let mut added = Vec::new();
        for _ in 0..added_count {
            let id = VarInt::read(input)?.0;
            let component = match framing {
                Framing::Plain => Component::read_value(id, input)?,
                Framing::Delimited => read_delimited_value(id, input)?,
            };

            added.push(component);
        }

        let mut removed = Vec::new();
        for _ in 0..removed_count {
            removed.push(VarInt::read(input)?);
        }

        Ok(Self { added, removed })
    }

    fn encode_framed(&self, out: &mut Vec<u8>, framing: Framing) -> EncodeResult {
        encode_count(self.added.len(), out)?;
        encode_count(self.removed.len(), out)?;

        for component in &self.added {
            VarInt(component.id()).encode(out)?;

            if let Framing::Delimited = framing {
                encode_count(component.value_len(), out)?;
            }

            component.encode_value(out)?;
        }

        for id in &self.removed {
            id.encode(out)?;
        }

        Ok(())
    }

    fn framed_len(&self, framing: Framing) -> usize {
        let mut length = VarInt(self.added.len() as i32).body_len() + VarInt(self.removed.len() as i32).body_len();

        for component in &self.added {
            let value_len = component.value_len();
            length += VarInt(component.id()).body_len() + value_len;

            if let Framing::Delimited = framing {
                length += VarInt(value_len as i32).body_len();
            }
        }

        length += self.removed.iter().map(VarInt::body_len).sum::<usize>();

        length
    }
}

impl<'p> PacketWriter<'p> for ComponentPatch<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        Self::read_framed(input, Framing::Plain)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.encode_framed(out, Framing::Plain)
    }

    fn body_len(&self) -> usize {
        self.framed_len(Framing::Plain)
    }
}

/// Reads a length-prefixed component value, which must fill its length.
fn read_delimited_value<'p>(id: i32, input: &mut &'p [u8]) -> DecodeResult<Component<'p>> {
    let length = read_length(input, input.len())?;
    let mut value = take(input, length)?;

    let component = Component::read_value(id, &mut value)?;
    if !value.is_empty() {
        return Err(DecodeError::TrailingData);
    }

    Ok(component)
}

fn encode_count(count: usize, out: &mut Vec<u8>) -> EncodeResult {
    let count = i32::try_from(count).map_err(|_| EncodeError::InvalidLength)?;

    VarInt(count).encode(out)
}

#[cfg(test)]
#[path = "../../../tests/tests_types_item.rs"]
mod tests;
