use std::io::Write;

use kagami_nbt::NbtTag;
use nom::IResult;

use crate::error::PResult;
use crate::traits::Serializable;

#[derive(Debug, Clone)]
pub struct Nbt(pub NbtTag);

impl Nbt {
    pub fn into_inner(self) -> NbtTag {
        self.0
    }

    pub fn inner(&self) -> &NbtTag {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut NbtTag {
        &mut self.0
    }
}

impl<'a> Serializable<'a> for Nbt {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        let bytes = kagami_nbt::serializer::to_bytes(&self.0, "");
        payload.write_all(&bytes)?;
        Ok(())
    }

    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (remaining, (_name, tag)) = kagami_nbt::parse_nbt(input)?;
        Ok((remaining, Nbt(tag)))
    }
}
