use futures_lite::AsyncWriteExt;

use crate::error::DecodeError;
use crate::packets::PacketWriter;

/// Network NBT value. Only encoding is implemented (registry data is
/// clientbound-only in practice); decoding returns `UnexpectedEof`.
#[derive(Debug, Clone)]
pub enum Nbt<'p> {
    Bool(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Str(&'p str),
    List(Vec<Nbt<'p>>),
    Compound(Vec<(&'p str, Nbt<'p>)>),
}

impl Nbt<'_> {
    fn id(&self) -> u8 {
        match self {
            Nbt::Bool(_) | Nbt::Byte(_) => 1,
            Nbt::Short(_) => 2,
            Nbt::Int(_) => 3,
            Nbt::Long(_) => 4,
            Nbt::Float(_) => 5,
            Nbt::Double(_) => 6,
            Nbt::Str(_) => 8,
            Nbt::List(_) => 9,
            Nbt::Compound(_) => 10,
        }
    }

    fn payload_len(&self) -> usize {
        match self {
            Nbt::Bool(_) | Nbt::Byte(_) => 1,
            Nbt::Short(_) => 2,
            Nbt::Int(_) | Nbt::Float(_) => 4,
            Nbt::Long(_) | Nbt::Double(_) => 8,
            Nbt::Str(s) => 2 + string_len(s),
            Nbt::List(items) => 1 + 4 + items.iter().map(Nbt::payload_len).sum::<usize>(),
            Nbt::Compound(entries) => entries.iter().map(|(k, v)| 1 + 2 + string_len(k) + v.payload_len()).sum::<usize>() + 1,
        }
    }

    fn write_payload(&self, w: &mut Vec<u8>) -> crate::encode::Result {
        match self {
            Nbt::Bool(b) => w.push(*b as u8),
            Nbt::Byte(b) => w.push(*b as u8),
            Nbt::Short(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Int(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Long(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Float(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Double(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Str(s) => {
                write_string(w, s)?;
            }
            Nbt::List(items) => {
                let elem = items.first().map(Nbt::id).unwrap_or(0);
                if items.iter().any(|item| item.id() != elem) {
                    return Err(crate::error::EncodeError::InvalidLength);
                }
                w.push(elem);
                let len = i32::try_from(items.len()).map_err(|_| crate::error::EncodeError::InvalidLength)?;
                w.extend_from_slice(&len.to_be_bytes());
                for item in items {
                    item.write_payload(w)?;
                }
            }
            Nbt::Compound(entries) => {
                for (k, v) in entries {
                    w.push(v.id());
                    write_string(w, k)?;
                    v.write_payload(w)?;
                }
                w.push(0);
            }
        }

        Ok(())
    }
}

pub(crate) fn write_string(w: &mut impl std::io::Write, value: &str) -> crate::encode::Result {
    let len = u16::try_from(string_len(value)).map_err(|_| crate::error::EncodeError::InvalidLength)?;
    w.write_all(&len.to_be_bytes())?;
    // NBT uses DataOutput.writeUTF, unlike protocol strings: NUL takes two
    // bytes and supplementary characters are encoded as UTF-16 surrogate pairs.
    if value.is_ascii() && !value.as_bytes().contains(&0) {
        w.write_all(value.as_bytes())?;
    } else {
        for unit in value.encode_utf16() {
            match unit {
                1..=0x7F => w.write_all(&[unit as u8])?,
                0..=0x7FF => {
                    w.write_all(&[0xC0 | (unit >> 6) as u8, 0x80 | (unit & 0x3F) as u8])?;
                }
                _ => {
                    w.write_all(&[0xE0 | (unit >> 12) as u8, 0x80 | ((unit >> 6) & 0x3F) as u8, 0x80 | (unit & 0x3F) as u8])?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn string_len(value: &str) -> usize {
    value.len()
        + value
            .chars()
            .map(|c| match c {
                '\0' => 1,
                '\u{10000}'.. => 2,
                _ => 0,
            })
            .sum::<usize>()
}

impl PacketWriter<'_> for Nbt<'_> {
    fn read(_input: &mut &[u8]) -> Result<Self, DecodeError> {
        Err(DecodeError::UnexpectedEof)
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        let mut bytes = Vec::with_capacity(self.body_len());
        bytes.push(self.id());
        self.write_payload(&mut bytes)?;
        w.write_all(&bytes).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        1 + self.payload_len()
    }
}

#[cfg(test)]
#[path = "../tests/tests_types_nbt.rs"]
mod tests;
