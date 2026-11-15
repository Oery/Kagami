//! Protocol state changes after login. Each direction changes state on its
//! own packets, so the two pumps track their state independently.

use mc_protocol::packets::PacketKind;
use mc_protocol::packets::client::{FinishConfiguration, StartConfiguration};
use mc_protocol::packets::server::{AcknowledgeConfiguration, AcknowledgeFinish};
use mc_protocol::{Direction, State};

/// Every packet that changes state is its id alone, which fits in a byte.
/// A longer packet never changes state.
pub const STATE_CHANGE_LEN: usize = 1;

/// The state `direction` is in after sending packet `id` in `state`.
pub fn next_state(direction: Direction, state: State, id: u8) -> State {
    match (direction, state, id) {
        (Direction::Clientbound, State::Config, <FinishConfiguration as PacketKind>::ID) => State::Play,
        (Direction::Clientbound, State::Play, <StartConfiguration as PacketKind>::ID) => State::Config,
        (Direction::Serverbound, State::Config, <AcknowledgeFinish as PacketKind>::ID) => State::Play,
        (Direction::Serverbound, State::Play, <AcknowledgeConfiguration as PacketKind>::ID) => State::Config,
        _ => state,
    }
}

#[cfg(test)]
#[path = "../tests/tests_transition.rs"]
mod tests;
