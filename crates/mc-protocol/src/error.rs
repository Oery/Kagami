use std::io;
use std::str;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("invalid or excessive length")]
    InvalidLength,

    #[error("io error")]
    IOError(#[from] io::Error),

    #[error("can't encode json")]
    InvalidJSON(#[from] serde_json::Error),

    #[error("a field the packet's flags require is missing")]
    MissingField,
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid compressed packet")]
    InvalidCompression,

    #[error("invalid or excessive length")]
    InvalidLength,

    #[error("packet contains trailing data")]
    TrailingData,

    #[error("expected bytes but got eof")]
    UnexpectedEof,

    #[error("bool is not 0/1, got {0}")]
    InvalidBool(u8),

    #[error("varint exceeds 32 bits")]
    InvalidVarInt,

    #[error("string is not valid utf8")]
    InvalidString(#[from] str::Utf8Error),

    #[error("enum variant is out of range")]
    EnumOutOfRange,

    #[error("values nest too deeply")]
    NestedTooDeeply,

    #[error("json is not valid")]
    InvalidJSON(#[from] serde_json::Error),

    #[error("rsa decrypt failed")]
    RsaDecrypt,

    #[error("mojang auth rejected the login")]
    AuthRejected,

    #[error("mojang profile id is not a valid uuid")]
    InvalidProfileId,
}
