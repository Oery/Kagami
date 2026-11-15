use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::packets::PacketWriter;

/// A block position in a named dimension.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct GlobalPos<'p> {
    pub dimension: Cow<'p, str>,
    /// A packed block position, as in the packets' `location` fields.
    pub position: i64,
}
