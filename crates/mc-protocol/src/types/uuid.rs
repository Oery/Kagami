use uuid::Uuid;

use crate::decode::take_array;
use crate::error::DecodeError;
use crate::packets::PacketWriter;

impl<'p> PacketWriter<'p> for Uuid {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        Ok(Uuid::from_bytes(take_array(input)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        out.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn body_len(&self) -> usize {
        16
    }
}
