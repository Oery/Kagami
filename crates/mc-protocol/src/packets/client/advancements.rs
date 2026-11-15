//! Advancements for protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::packet;

use crate::types::bytes::Remainder;

#[packet(Play, 0x85, Clientbound)]
pub struct UpdateAdvancements<'p> {
    /// Clears the known advancements first.
    pub reset: bool,
    /// The added advancements, the removed identifiers, the progress and
    /// whether to show them; not decoded, since displays hold item stacks.
    pub changes: Remainder<'p>,
}

/// Opens a tab of the advancements screen; `None` closes the screen.
#[packet(Play, 0x57, Clientbound)]
pub struct SelectAdvancementsTab<'p> {
    pub tab: Option<Cow<'p, str>>,
}
