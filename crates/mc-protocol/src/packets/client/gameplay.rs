//! Screens, respawn and combat packets for protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::Nbt;

// Protocol 777; verified against the 26.3 client screen codecs and MenuType registry.
#[packet(Play, 0x3C, Clientbound)]
pub struct OpenScreen<'p> {
    #[format = "varint"]
    pub window_id: i32,
    #[format = "varint"]
    pub kind: i32,
    pub title: Nbt<'p>,
}

#[packet(Play, 0x11, Clientbound)]
pub struct CloseScreen {
    #[format = "varint"]
    pub window_id: i32,
}

#[packet(Play, 0x07, Clientbound)]
pub struct BlockEvent {
    pub location: i64,
    pub action: u8,
    pub parameter: u8,
    #[format = "varint"]
    pub block: i32,
}

#[packet(Play, 0x6B, Clientbound)]
pub struct SetHeldItemResponse {
    #[format = "varint"]
    pub slot: i32,
}

#[derive(Debug, IntoOwned)]
pub struct DeathLocation<'p> {
    pub dimension: Cow<'p, str>,
    pub location: i64,
}

impl<'p> PacketWriter<'p> for DeathLocation<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(Self { dimension: Cow::read(input)?, location: i64::read(input)? })
    }
    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.dimension.encode(out)?;
        self.location.encode(out)
    }
    fn body_len(&self) -> usize {
        self.dimension.body_len() + 8
    }
}

#[packet(Play, 0x54, Clientbound)]
pub struct Respawn<'p> {
    #[format = "varint"]
    pub dimension_type: i32,
    pub dimension_name: Cow<'p, str>,
    pub hashed_seed: i64,
    #[format = "varint"]
    pub game_mode: i32,
    #[format = "varint"]
    pub previous_game_mode: i32,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation<'p>>,
    #[format = "varint"]
    pub portal_cooldown: i32,
    #[format = "varint"]
    pub sea_level: i32,
    pub data_kept: u8,
}

#[packet(Play, 0x45, Clientbound)]
pub struct CombatKill<'p> {
    #[format = "varint"]
    pub player_id: i32,
    pub message: Nbt<'p>,
}

#[derive(Debug)]
pub struct DamagePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PacketWriter<'_> for DamagePosition {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        Ok(Self { x: f64::read(input)?, y: f64::read(input)?, z: f64::read(input)? })
    }
    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.x.encode(out)?;
        self.y.encode(out)?;
        self.z.encode(out)
    }
    fn body_len(&self) -> usize {
        24
    }
}

#[packet(Play, 0x19, Clientbound)]
pub struct DamageEvent {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub source_type_id: i32,
    /// Protocol entity ID plus one; zero represents no source entity.
    #[format = "varint"]
    pub source_cause_id: i32,
    #[format = "varint"]
    pub source_direct_id: i32,
    pub source_position: Option<DamagePosition>,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_client_gameplay.rs"]
mod tests;
