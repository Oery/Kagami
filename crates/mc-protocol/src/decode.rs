use std::cell::Cell;

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

/// How deeply item stacks, components and boxed values may nest. Vanilla
/// items stay a few levels deep; a 2 MiB frame could otherwise nest far
/// enough to overflow the stack.
const MAX_NESTING: usize = 64;

thread_local! {
    static NESTING: Cell<usize> = const { Cell::new(0) };
}

/// One level of nesting, counted while alive. Recursive codecs hold one
/// while they decode, so crafted input fails instead of crashing.
pub(crate) struct Nesting(());

impl Nesting {
    pub(crate) fn enter() -> Result<Self> {
        let depth = NESTING.get();
        if depth >= MAX_NESTING {
            return Err(DecodeError::NestedTooDeeply);
        }

        NESTING.set(depth + 1);

        Ok(Self(()))
    }
}

impl Drop for Nesting {
    fn drop(&mut self) {
        NESTING.set(NESTING.get() - 1);
    }
}
