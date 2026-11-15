//! Serverbound gameplay packets for protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{LpVec3, VarInt};

#[packet(Play, 0x0C, Serverbound)]
pub struct ClientCommand {
    #[format = "varint"]
    pub action: i32,
}

#[packet(Play, 0x13, Serverbound)]
pub struct CloseContainer {
    #[format = "varint"]
    pub window_id: i32,
}

#[packet(Play, 0x36, Serverbound)]
pub struct SetHeldItem {
    pub slot: i16,
}

#[packet(Play, 0x29, Serverbound)]
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

#[packet(Play, 0x2A, Serverbound)]
pub struct PlayerCommand {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub action: i32,
    #[format = "varint"]
    pub jump_boost: i32,
}

#[packet(Play, 0x2B, Serverbound)]
pub struct PlayerInput {
    pub flags: u8,
}

#[packet(Play, 0x28, Serverbound)]
pub struct PlayerAbilitiesRequest {
    /// The client may only request the flying bit (0x02).
    pub flags: u8,
}

#[packet(Play, 0x42, Serverbound)]
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

#[packet(Play, 0x43, Serverbound)]
pub struct UseItem {
    #[format = "varint"]
    pub hand: i32,
    #[format = "varint"]
    pub sequence: i32,
    pub yaw: f32,
    pub pitch: f32,
}

/// An item stack as the client predicts it: components are sent as hashes.
#[derive(Debug)]
pub struct HashedItem {
    pub item_id: i32,
    pub count: i32,
    pub added: Vec<ComponentHash>,
    pub removed: Vec<VarInt>,
}

impl<'p> PacketWriter<'p> for HashedItem {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let item_id = VarInt::read(input)?.0;
        let count = VarInt::read(input)?.0;
        let added = Vec::read(input)?;
        let removed = Vec::read(input)?;

        Ok(Self { item_id, count, added, removed })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.item_id).encode(out)?;
        VarInt(self.count).encode(out)?;
        self.added.encode(out)?;

        self.removed.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.item_id).body_len() + VarInt(self.count).body_len() + self.added.body_len() + self.removed.body_len()
    }
}

#[derive(Debug)]
pub struct ComponentHash {
    pub component: i32,
    /// CRC32C of the component value.
    pub hash: i32,
}

impl<'p> PacketWriter<'p> for ComponentHash {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(Self { component: VarInt::read(input)?.0, hash: i32::read(input)? })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.component).encode(out)?;
        self.hash.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.component).body_len() + 4
    }
}

#[derive(Debug)]
pub struct ChangedSlot {
    pub slot: i16,
    pub item: Option<HashedItem>,
}

impl<'p> PacketWriter<'p> for ChangedSlot {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(Self { slot: i16::read(input)?, item: Option::read(input)? })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.slot.encode(out)?;
        self.item.encode(out)
    }

    fn body_len(&self) -> usize {
        2 + self.item.body_len()
    }
}

#[packet(Play, 0x12, Serverbound)]
pub struct ClickContainer {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub state_id: i32,
    pub slot: i16,
    pub button: u8,
    #[format = "varint"]
    pub mode: i32,
    pub changed_slots: Vec<ChangedSlot>,
    pub carried: Option<HashedItem>,
}

/// A right click on an entity. Attacks are [`Attack`](super::Attack).
#[packet(Play, 0x1A, Serverbound)]
pub struct Interact {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub hand: i32,
    /// The clicked point, relative to the entity's position.
    pub target_offset: LpVec3,
    pub sneaking: bool,
}

/// The client moving the vehicle it controls; degrees.
#[packet(Play, 0x22, Serverbound)]
pub struct ClientMoveVehicle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[packet(Play, 0x23, Serverbound)]
pub struct PaddleBoat {
    pub left_turning: bool,
    pub right_turning: bool,
}

/// A spectator's click: the targeted entity ID plus one, or zero for none.
#[packet(Play, 0x3F, Serverbound)]
pub struct SpectatorAction {
    #[format = "varint"]
    pub entity_id: i32,
}

/// The game mode switcher (F3 + F4).
#[packet(Play, 0x05, Serverbound)]
pub struct ChangeGameMode {
    #[format = "varint"]
    pub game_mode: i32,
}

/// A [`difficulty`](crate::packets::client::difficulty) value.
#[packet(Play, 0x04, Serverbound)]
pub struct ChangeDifficultyRequest {
    pub difficulty: u8,
}

#[packet(Play, 0x1D, Serverbound)]
pub struct LockDifficulty {
    pub locked: bool,
}

#[derive(Debug, IntoOwned)]
pub enum AdvancementsScreenAction<'p> {
    OpenedTab(Cow<'p, str>),
    ClosedScreen,
}

impl<'p> PacketWriter<'p> for AdvancementsScreenAction<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::OpenedTab(Cow::read(input)?)),
            1 => Ok(Self::ClosedScreen),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::OpenedTab(tab) => {
                VarInt(0).encode(out)?;
                tab.encode(out)
            }
            Self::ClosedScreen => VarInt(1).encode(out),
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::OpenedTab(tab) => 1 + tab.body_len(),
            Self::ClosedScreen => 1,
        }
    }
}

#[packet(Play, 0x33, Serverbound)]
pub struct SeenAdvancements<'p> {
    pub action: AdvancementsScreenAction<'p>,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_server_gameplay.rs"]
mod tests;
