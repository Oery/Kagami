use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::decode::{MAX_FRAME_LEN, read_length, take};
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::VarInt;

impl<'p> PacketWriter<'p> for &'p [u8] {
    fn read(input: &mut &'p [u8]) -> Result<&'p [u8], DecodeError> {
        let len = read_length(input, MAX_FRAME_LEN)?;
        let bytes = take(input, len)?;

        Ok(bytes)
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        VarInt(self.len() as i32).encode(out)?;
        out.extend_from_slice(self);

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.len() + VarInt(self.len() as i32).body_len()
    }
}

impl<'p> PacketWriter<'p> for Cow<'p, [u8]> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        Ok(Cow::Borrowed(<&[u8]>::read(input)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.as_ref().encode(out)
    }

    fn body_len(&self) -> usize {
        self.as_ref().body_len()
    }
}

/// The rest of the payload, without a length prefix.
#[derive(Debug, IntoOwned)]
pub struct Remainder<'p>(pub Cow<'p, [u8]>);

impl<'p> PacketWriter<'p> for Remainder<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let rest = take(input, input.len())?;
        Ok(Remainder(Cow::Borrowed(rest)))
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        out.extend_from_slice(&self.0);

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.0.len()
    }
}

impl<'p> PacketWriter<'p> for String {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        Ok(<&str>::read(input)?.to_owned())
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.as_str().encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.as_str().body_len()
    }
}
