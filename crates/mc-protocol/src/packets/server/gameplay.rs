//! Gameplay commands are decoded into bounded owned values before simulation.
use futures_lite::AsyncWrite;
use mc_protocol_macros::{Payload, packet};

use crate::State;
use crate::data::items::{COMPONENT_COUNT, ITEMS};
use crate::decode::{Result as DecodeResult, read_length};
use crate::encode::Result as EncodeResult;
use crate::error::{DecodeError, EncodeError};
use crate::packets::{PacketWriter, Payload};
use crate::types::VarInt;

#[packet(Play, 0x0C, Server)]
pub struct ClientCommand {
    #[format = "varint"]
    pub action: i32,
}

#[packet(Play, 0x13, Server)]
pub struct CloseContainer {
    #[format = "varint"]
    pub window_id: i32,
}

#[packet(Play, 0x36, Server)]
pub struct SetHeldItem {
    pub slot: i16,
}

#[packet(Play, 0x29, Server)]
pub struct PlayerAction {
    #[format = "varint"]
    pub action: i32,
    pub location: i64,
    pub face: u8,
    #[format = "varint"]
    pub sequence: i32,
}

impl PlayerAction {
    // 26.3 added CHANGE_DESTROY_DIRECTION; older versions' action ordinals differ.
    pub const START_DESTROY_BLOCK: i32 = 0;
    pub const DROP_ALL_ITEMS: i32 = 4;
    pub const DROP_ITEM: i32 = 5;
    pub const RELEASE_USE_ITEM: i32 = 6;
    pub const SWAP_ITEM_WITH_OFFHAND: i32 = 7;
}

#[packet(Play, 0x2A, Server)]
pub struct PlayerCommand {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub action: i32,
    #[format = "varint"]
    pub jump_boost: i32,
}

#[packet(Play, 0x2B, Server)]
pub struct PlayerInput {
    pub flags: u8,
}

#[packet(Play, 0x28, Server)]
pub struct PlayerAbilitiesRequest {
    /// The client may only request the flying bit (0x02).
    pub flags: u8,
}

#[packet(Play, 0x42, Server)]
pub struct UseItemOn {
    #[format = "varint"]
    pub hand: i32,
    pub location: i64,
    #[format = "varint"]
    pub face: i32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    #[format = "varint"]
    pub sequence: i32,
}

#[packet(Play, 0x43, Server)]
pub struct UseItem {
    #[format = "varint"]
    pub hand: i32,
    #[format = "varint"]
    pub sequence: i32,
    pub yaw: f32,
    pub pitch: f32,
}

/// Prediction hashes never enter gameplay ownership. Keeping only the slot mask
/// lets the server overwrite every predicted change without implementing HashOps.
#[derive(Debug)]
pub struct ClickContainer {
    pub window_id: i32,
    pub state_id: i32,
    pub slot: i16,
    pub button: u8,
    pub mode: i32,
    pub predicted_slots: u128,
}

fn component_id(input: &mut &[u8]) -> DecodeResult<usize> {
    let id = usize::try_from(VarInt::read(input)?.0).map_err(|_| DecodeError::EnumOutOfRange)?;
    if id >= COMPONENT_COUNT {
        return Err(DecodeError::EnumOutOfRange);
    }
    Ok(id)
}

fn skip_hashed_stack(input: &mut &[u8]) -> DecodeResult<()> {
    if !bool::read(input)? {
        return Ok(());
    }
    let id = usize::try_from(VarInt::read(input)?.0).map_err(|_| DecodeError::EnumOutOfRange)?;
    if id == 0 || id >= ITEMS.len() {
        return Err(DecodeError::EnumOutOfRange);
    }
    let count = VarInt::read(input)?.0;
    if !(1..=99).contains(&count) {
        return Err(DecodeError::InvalidLength);
    }
    let added = read_length(input, COMPONENT_COUNT)?;
    let mut seen = 0u128;
    for _ in 0..added {
        let id = component_id(input)?;
        if seen & (1 << id) != 0 {
            return Err(DecodeError::InvalidLength);
        }
        seen |= 1 << id;
        i32::read(input)?;
    }
    let removed = read_length(input, COMPONENT_COUNT)?;
    for _ in 0..removed {
        let id = component_id(input)?;
        if seen & (1 << id) != 0 {
            return Err(DecodeError::InvalidLength);
        }
        seen |= 1 << id;
    }
    Ok(())
}

impl<'p> Payload<'p> for ClickContainer {
    const ID: VarInt = VarInt(0x12);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        Err(EncodeError::InvalidLength)
    }

    fn decode_payload(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let window_id = VarInt::read(input)?.0;
        let state_id = VarInt::read(input)?.0;
        let slot = i16::read(input)?;
        let button = u8::read(input)?;
        let mode = VarInt::read(input)?.0;
        let changed = read_length(input, 128)?;
        let mut predicted_slots = 0u128;
        for _ in 0..changed {
            let slot = i16::read(input)?;
            if !(0..90).contains(&slot) {
                return Err(DecodeError::EnumOutOfRange);
            }
            let bit = 1u128 << slot;
            if predicted_slots & bit != 0 {
                return Err(DecodeError::InvalidLength);
            }
            predicted_slots |= bit;
            skip_hashed_stack(input)?;
        }
        skip_hashed_stack(input)?;
        Ok(Self { window_id, state_id, slot, button, mode, predicted_slots })
    }

    async fn write_payload<W: AsyncWrite + Unpin>(&self, _w: &mut W) -> EncodeResult {
        Err(EncodeError::InvalidLength)
    }
}

#[cfg(test)]
#[path = "../../tests/tests_packets_server_gameplay.rs"]
mod tests;
