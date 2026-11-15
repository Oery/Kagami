//! Protocol 777 block feedback and supported block-entity synchronization.
use crate::State;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::{PacketWriter, Payload};
use crate::types::{Nbt, VarInt};
use mc_protocol_macros::{Payload, packet};

#[packet(Play, 0x05, Client)]
pub struct SetBlockDestroyStage {
    #[format = "varint"]
    pub entity_id: i32,
    pub location: i64,
    pub stage: u8,
}

#[packet(Play, 0x06, Client)]
pub struct BlockEntityData<'p> {
    pub location: i64,
    #[format = "varint"]
    pub kind: i32,
    pub data: Nbt<'p>,
}

#[packet(Play, 0x2F, Client)]
pub struct WorldEvent {
    pub event: i32,
    pub location: i64,
    pub data: i32,
    pub global: bool,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_client_blocks.rs"]
mod tests;
