use thiserror::Error;

// TODO:
// Split this into multiple error types
//
#[derive(Error, Debug)]
pub enum PacketError {
    #[error("failed to deserialize packet")]
    Deserializing(nom::Err<nom::error::Error<Vec<u8>>>),

    #[error("incomplete packet")]
    Incomplete,

    #[error("unknown packet")]
    UnknownPacket,

    #[error("unsupported packet")]
    Unsupported,

    #[error("something happened")]
    IOError(#[from] std::io::Error),
}

use nom::{Err, error::Error};

impl From<Err<Error<&[u8]>>> for PacketError {
    fn from(err: Err<Error<&[u8]>>) -> Self {
        Self::Deserializing(err.map_input(|input| input.to_owned()))
    }
}

#[derive(Error, Debug)]
pub enum KagamiError {
    #[error("packet error")]
    PacketError(#[from] PacketError),

    #[error("io error")]
    IOError(#[from] std::io::Error),
}

pub type KResult<T> = Result<T, KagamiError>;
pub type PResult<T> = Result<T, PacketError>;
