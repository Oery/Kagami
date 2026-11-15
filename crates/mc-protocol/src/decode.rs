use crate::error::DecodeError;

pub type Result<T> = std::result::Result<T, DecodeError>;

/// Application limits, checked before allocating or waiting for a frame body.
pub const MAX_FRAME_LEN: usize = (1 << 21) - 1;
pub const MAX_COLLECTION_LEN: usize = 65_536;

pub fn read_length(input: &mut &[u8], limit: usize) -> Result<usize> {
    use crate::{PacketWriter, VarInt};
    let length = usize::try_from(VarInt::read(input)?.0).map_err(|_| DecodeError::InvalidLength)?;
    if length > limit {
        return Err(DecodeError::InvalidLength);
    }
    Ok(length)
}

pub fn take<'p>(input: &mut &'p [u8], n: usize) -> Result<&'p [u8]> {
    if input.len() < n {
        return Err(DecodeError::UnexpectedEof);
    }

    let (head, tail) = input.split_at(n);
    *input = tail;

    Ok(head)
}

/// Takes exactly `N` bytes as an array. Length is pre-checked by `take`,
/// so the conversion is infallible; `UnexpectedEof` is a defensive fallback.
pub fn take_array<const N: usize>(input: &mut &[u8]) -> std::result::Result<[u8; N], DecodeError> {
    take(input, N)?.try_into().map_err(|_| DecodeError::UnexpectedEof)
}
