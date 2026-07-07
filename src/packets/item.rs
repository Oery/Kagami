use kagami_nbt::{NbtTag, parser, serializer};
use kagami_macros::Serializable;

use std::io::Write;

use nom::IResult;

use crate::error::PResult;
use crate::packet::*;
use crate::traits::Serializable;

#[derive(Debug, Clone)]
pub struct SlotNbt(pub Option<NbtTag>);

impl SlotNbt {
    pub fn into_inner(self) -> Option<NbtTag> {
        self.0
    }

    pub fn inner(&self) -> &Option<NbtTag> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Option<NbtTag> {
        &mut self.0
    }
}

impl Serializable<'_> for SlotNbt {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        match &self.0 {
            None => {
                payload.write_all(&[0])?;
            }
            Some(tag) => {
                let nbt_bytes = serializer::to_bytes(tag, "");
                payload.write_all(&nbt_bytes)?;
            }
        }
        Ok(())
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> {
        if input.is_empty() {
            return Ok((input, SlotNbt(None)));
        }
        if input[0] == 0 {
            return Ok((&input[1..], SlotNbt(None)));
        }
        if let Ok((remaining, (_, tag))) = parser::parse_nbt(input) {
            return Ok((remaining, SlotNbt(Some(tag))));
        }
        Ok((input, SlotNbt(None)))
    }
}

#[derive(Debug, Serializable)]
pub struct Item {
    pub id: i16,
    pub count: u8,
    pub damage: i16,
    pub metadata: SlotNbt,
}

impl Serializable<'_> for Option<Item> {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        match &self {
            None => {
                payload.write_all(&(-1i16).to_be_bytes())?;
            }
            Some(item) => item.serialize(payload)?,
        };
        Ok(())
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, id) = short(input)?;

        if id == -1 {
            return Ok((input, None));
        }

        let (input, count) = nom::bytes::streaming::take(1usize)(input)?;
        let count = count[0];
        let (input, damage) = short(input)?;
        let (input, metadata) = SlotNbt::deserialize(input)?;
        Ok((input, Some(Item { id, count, damage, metadata })))
    }
}
