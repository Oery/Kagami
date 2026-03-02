use kagami_macros::Serializable;

use std::io::Write;

use crate::error::PResult;
use crate::packet::*;
use crate::traits::Serializable;

// TODO: Add NBT support
#[derive(Debug, Serializable)]
pub struct Item {
    pub id: i16,
    pub count: u8,
    pub damage: i16,
    // pub metadata: Nbt,
}

impl Serializable<'_> for Option<Item> {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        match &self {
            None => payload.write_all(&i16::to_be_bytes(-1i16))?,
            Some(item) => item.serialize(payload)?,
        };

        Ok(())
    }

    fn deserialize(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let (c_input, id) = varint_i32(input)?;

        if id == -1 {
            return Ok((c_input, None));
        }

        let (input, item) = Item::deserialize(input)?;
        Ok((input, Some(item)))
    }
}
