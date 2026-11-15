use mc_protocol_macros::IntoOwned;

use crate::error::DecodeError;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Handshake,
    Status,
    Login,
    Config,
    Play,
}

/// The handshake carries an intent, not a freely selectable protocol state.
#[derive(Clone, Copy, Debug, PartialEq, IntoOwned)]
#[repr(i32)]
pub enum HandshakeIntent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl TryFrom<i32> for HandshakeIntent {
    type Error = DecodeError;

    fn try_from(val: i32) -> Result<Self, Self::Error> {
        match val {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}

impl HandshakeIntent {
    pub fn state(self) -> State {
        match self {
            Self::Status => State::Status,
            Self::Login | Self::Transfer => State::Login,
        }
    }
}

#[cfg(test)]
#[path = "tests/tests_state.rs"]
mod tests;
