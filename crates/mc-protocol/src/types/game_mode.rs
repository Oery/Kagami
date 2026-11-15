use std::fmt;

use crate::error::DecodeError;

/// Protocol game-mode IDs. Gameplay policy belongs to the server.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMode {
    #[default]
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl TryFrom<i32> for GameMode {
    type Error = DecodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Survival),
            1 => Ok(Self::Creative),
            2 => Ok(Self::Adventure),
            3 => Ok(Self::Spectator),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Survival => "survival",
            Self::Creative => "creative",
            Self::Adventure => "adventure",
            Self::Spectator => "spectator",
        })
    }
}
