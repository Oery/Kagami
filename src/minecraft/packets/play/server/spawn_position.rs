use crate::minecraft::Packet;
use crate::serialization::Position;
use kagami_macro::{packet, Deserialize, Serialize};

#[packet(0x05, server)]
pub struct SpawnPosition {
    pub position: Position,
}
