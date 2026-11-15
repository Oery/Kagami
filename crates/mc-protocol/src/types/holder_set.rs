use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::decode::{MAX_COLLECTION_LEN, Result as DecodeResult};
use crate::encode::Result as EncodeResult;
use crate::error::{DecodeError, EncodeError};
use crate::packets::PacketWriter;
use crate::types::VarInt;

/// Entries of the registry the field implies: a tag, or IDs listed inline.
///
/// On the wire a zero VarInt is followed by the tag's identifier; any other
/// value is the number of IDs plus one, then the IDs.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum HolderSet<'p> {
    /// A tag identifier, without the leading `#`.
    Tag(Cow<'p, str>),
    Ids(Vec<VarInt>),
}

impl<'p> PacketWriter<'p> for HolderSet<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let header = VarInt::read(input)?.0;
        if header == 0 {
            return Ok(Self::Tag(Cow::read(input)?));
        }

        let count = usize::try_from(header - 1).map_err(|_| DecodeError::InvalidLength)?;
        if count > MAX_COLLECTION_LEN {
            return Err(DecodeError::InvalidLength);
        }

        let mut ids = Vec::new();
        for _ in 0..count {
            ids.push(VarInt::read(input)?);
        }

        Ok(Self::Ids(ids))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Tag(tag) => {
                VarInt(0).encode(out)?;
                tag.encode(out)
            }
            Self::Ids(ids) => {
                let header = i32::try_from(ids.len() + 1).map_err(|_| EncodeError::InvalidLength)?;
                VarInt(header).encode(out)?;

                for id in ids {
                    id.encode(out)?;
                }

                Ok(())
            }
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::Tag(tag) => 1 + tag.body_len(),
            Self::Ids(ids) => {
                let header = VarInt((ids.len() + 1) as i32).body_len();
                let ids_len: usize = ids.iter().map(VarInt::body_len).sum();

                header + ids_len
            }
        }
    }
}
