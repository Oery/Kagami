use crate::IntoOwned;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;

/// One of two values, sent as a bool that is true for `Left`, then the value.
#[derive(Clone, Debug, PartialEq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'p, L: PacketWriter<'p>, R: PacketWriter<'p>> PacketWriter<'p> for Either<L, R> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let is_left = bool::read(input)?;
        if is_left {
            return Ok(Self::Left(L::read(input)?));
        }

        Ok(Self::Right(R::read(input)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Left(value) => {
                true.encode(out)?;
                value.encode(out)
            }
            Self::Right(value) => {
                false.encode(out)?;
                value.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        let value = match self {
            Self::Left(value) => value.body_len(),
            Self::Right(value) => value.body_len(),
        };

        1 + value
    }
}

impl<L: IntoOwned, R: IntoOwned> IntoOwned for Either<L, R> {
    type Owned = Either<L::Owned, R::Owned>;

    fn into_owned(self) -> Self::Owned {
        match self {
            Self::Left(value) => Either::Left(value.into_owned()),
            Self::Right(value) => Either::Right(value.into_owned()),
        }
    }
}
