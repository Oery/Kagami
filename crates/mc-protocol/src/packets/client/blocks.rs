//! Protocol 777 block feedback and supported block-entity synchronization.
use crate::types::Nbt;
use mc_protocol_macros::packet;

#[packet(Play, 0x05, Clientbound)]
pub struct SetBlockDestroyStage {
    #[format = "varint"]
    pub entity_id: i32,
    pub location: i64,
    pub stage: u8,
}

#[packet(Play, 0x06, Clientbound)]
pub struct BlockEntityData<'p> {
    pub location: i64,
    #[format = "varint"]
    pub kind: i32,
    pub data: Nbt<'p>,
}

#[packet(Play, 0x2F, Clientbound)]
pub struct WorldEvent {
    pub event: i32,
    pub location: i64,
    pub data: i32,
    pub global: bool,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_client_blocks.rs"]
mod tests;
