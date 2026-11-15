use crate::IntoOwned;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::EncodeError;
use crate::packets::PacketWriter;
use crate::types::VarInt;

/// A registry reference or an inline value, the wire's "ID or X".
///
/// On the wire a zero VarInt is followed by the inline value; any other
/// value is the registry ID plus one.
#[derive(Clone, Debug, PartialEq)]
pub enum IdOr<T> {
    /// An entry of the registry the field implies, by its ID.
    Id(i32),
    Inline(T),
}

impl<'p, T: PacketWriter<'p>> PacketWriter<'p> for IdOr<T> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let id = VarInt::read(input)?.0;
        if id == 0 {
            return Ok(Self::Inline(T::read(input)?));
        }

        Ok(Self::Id(id - 1))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Id(id) => {
                let wire_id = id.checked_add(1).ok_or(EncodeError::InvalidLength)?;
                VarInt(wire_id).encode(out)
            }
            Self::Inline(value) => {
                VarInt(0).encode(out)?;
                value.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::Id(id) => VarInt(id.wrapping_add(1)).body_len(),
            Self::Inline(value) => 1 + value.body_len(),
        }
    }
}

impl<T: IntoOwned> IntoOwned for IdOr<T> {
    type Owned = IdOr<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        match self {
            Self::Id(id) => IdOr::Id(id),
            Self::Inline(value) => IdOr::Inline(value.into_owned()),
        }
    }
}
