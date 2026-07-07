use thiserror::Error;

#[derive(Error, Debug)]
pub enum NbtError {
    #[error("failed to parse NBT: {0}")]
    Parse(#[from] nom::Err<nom::error::Error<Vec<u8>>>),

    #[error("unexpected NBT tag type: {0}")]
    UnknownTagType(u8),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type NbtResult<T> = Result<T, NbtError>;
