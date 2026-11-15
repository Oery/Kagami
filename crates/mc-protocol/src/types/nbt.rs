use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::decode::{Result as DecodeResult, take, take_array};
use crate::error::DecodeError;
use crate::packets::PacketWriter;

/// Vanilla's limit on nested lists and compounds.
const MAX_DEPTH: usize = 512;

/// A network NBT value: an unnamed root tag, as text components and other
/// NBT fields are sent since 1.20.2.
///
/// Decoding borrows strings and byte arrays from the input. Two encodings
/// are not kept: a byte decodes as `Byte`, never `Bool`, and an empty list
/// re-encodes with the element type vanilla writes for it, `End`.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub enum Nbt<'p> {
    Bool(bool),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Cow<'p, [u8]>),
    Str(Cow<'p, str>),
    List(Vec<Nbt<'p>>),
    Compound(Vec<(Cow<'p, str>, Nbt<'p>)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
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
            Nbt::ByteArray(_) => 7,
            Nbt::Str(_) => 8,
            Nbt::List(_) => 9,
            Nbt::Compound(_) => 10,
            Nbt::IntArray(_) => 11,
            Nbt::LongArray(_) => 12,
        }
    }

    fn payload_len(&self) -> usize {
        match self {
            Nbt::Bool(_) | Nbt::Byte(_) => 1,
            Nbt::Short(_) => 2,
            Nbt::Int(_) | Nbt::Float(_) => 4,
            Nbt::Long(_) | Nbt::Double(_) => 8,
            Nbt::ByteArray(bytes) => 4 + bytes.len(),
            Nbt::IntArray(values) => 4 + values.len() * 4,
            Nbt::LongArray(values) => 4 + values.len() * 8,
            Nbt::Str(s) => 2 + string_len(s),
            Nbt::List(items) => 1 + 4 + items.iter().map(Nbt::payload_len).sum::<usize>(),
            Nbt::Compound(entries) => entries.iter().map(|(k, v)| 1 + 2 + string_len(k) + v.payload_len()).sum::<usize>() + 1,
        }
    }

    fn encode_payload(&self, w: &mut Vec<u8>) -> crate::encode::Result {
        match self {
            Nbt::Bool(b) => w.push(*b as u8),
            Nbt::Byte(b) => w.push(*b as u8),
            Nbt::Short(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Int(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Long(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Float(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::Double(v) => w.extend_from_slice(&v.to_be_bytes()),
            Nbt::ByteArray(bytes) => {
                write_array_len(w, bytes.len())?;
                w.extend_from_slice(bytes);
            }
            Nbt::IntArray(values) => {
                write_array_len(w, values.len())?;

                for value in values {
                    w.extend_from_slice(&value.to_be_bytes());
                }
            }
            Nbt::LongArray(values) => {
                write_array_len(w, values.len())?;

                for value in values {
                    w.extend_from_slice(&value.to_be_bytes());
                }
            }
            Nbt::Str(s) => {
                write_string(w, s)?;
            }
            Nbt::List(items) => {
                let elem = items.first().map(Nbt::id).unwrap_or(0);
                if items.iter().any(|item| item.id() != elem) {
                    return Err(crate::error::EncodeError::InvalidLength);
                }

                w.push(elem);
                write_array_len(w, items.len())?;

                for item in items {
                    item.encode_payload(w)?;
                }
            }
            Nbt::Compound(entries) => {
                for (k, v) in entries {
                    w.push(v.id());
                    write_string(w, k)?;
                    v.encode_payload(w)?;
                }

                w.push(0);
            }
        }

        Ok(())
    }
}

fn write_array_len(w: &mut Vec<u8>, len: usize) -> crate::encode::Result {
    let len = i32::try_from(len).map_err(|_| crate::error::EncodeError::InvalidLength)?;
    w.extend_from_slice(&len.to_be_bytes());

    Ok(())
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

impl<'p> PacketWriter<'p> for Nbt<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let tag = u8::read(input)?;

        read_payload(input, tag, 0)
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        out.push(self.id());
        self.encode_payload(out)
    }

    fn body_len(&self) -> usize {
        1 + self.payload_len()
    }
}

/// Network NBT that may be absent, sent as a lone `TAG_End`.
#[derive(Debug, Clone, PartialEq, IntoOwned)]
pub struct OptionalNbt<'p>(pub Option<Nbt<'p>>);

impl<'p> PacketWriter<'p> for OptionalNbt<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let tag = u8::read(input)?;
        if tag == 0 {
            return Ok(Self(None));
        }

        let value = read_payload(input, tag, 0)?;

        Ok(Self(Some(value)))
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        match &self.0 {
            Some(value) => value.encode(out),
            None => 0u8.encode(out),
        }
    }

    fn body_len(&self) -> usize {
        match &self.0 {
            Some(value) => value.body_len(),
            None => 1,
        }
    }
}

fn read_payload<'p>(input: &mut &'p [u8], tag: u8, depth: usize) -> DecodeResult<Nbt<'p>> {
    if depth > MAX_DEPTH {
        return Err(DecodeError::InvalidLength);
    }

    let value = match tag {
        1 => Nbt::Byte(u8::read(input)? as i8),
        2 => Nbt::Short(i16::read(input)?),
        3 => Nbt::Int(i32::read(input)?),
        4 => Nbt::Long(i64::read(input)?),
        5 => Nbt::Float(f32::read(input)?),
        6 => Nbt::Double(f64::read(input)?),
        7 => {
            let len = element_count(input, 1)?;
            Nbt::ByteArray(Cow::Borrowed(take(input, len)?))
        }
        8 => Nbt::Str(read_string(input)?),
        9 => Nbt::List(read_list(input, depth)?),
        10 => Nbt::Compound(read_compound(input, depth)?),
        11 => {
            let len = element_count(input, 4)?;
            Nbt::IntArray((0..len).map(|_| i32::read(input)).collect::<DecodeResult<_>>()?)
        }
        12 => {
            let len = element_count(input, 8)?;
            Nbt::LongArray((0..len).map(|_| i64::read(input)).collect::<DecodeResult<_>>()?)
        }
        _ => return Err(DecodeError::EnumOutOfRange),
    };

    Ok(value)
}

fn read_list<'p>(input: &mut &'p [u8], depth: usize) -> DecodeResult<Vec<Nbt<'p>>> {
    let tag = u8::read(input)?;
    let len = element_count(input, min_payload_len(tag))?;

    // Only an empty list may have no element type.
    if tag == 0 && len > 0 {
        return Err(DecodeError::EnumOutOfRange);
    }

    let mut items = Vec::with_capacity(len);

    for _ in 0..len {
        items.push(read_payload(input, tag, depth + 1)?);
    }

    Ok(items)
}

fn read_compound<'p>(input: &mut &'p [u8], depth: usize) -> DecodeResult<Vec<(Cow<'p, str>, Nbt<'p>)>> {
    let mut entries = Vec::new();

    loop {
        let tag = u8::read(input)?;
        if tag == 0 {
            return Ok(entries);
        }

        let name = read_string(input)?;
        let value = read_payload(input, tag, depth + 1)?;
        entries.push((name, value));
    }
}

/// Reads a signed array or list length, refusing one the remaining input
/// cannot hold, so nothing is allocated from an untrusted count.
fn element_count(input: &mut &[u8], min_element_len: usize) -> DecodeResult<usize> {
    let len = usize::try_from(i32::read(input)?).map_err(|_| DecodeError::InvalidLength)?;

    if len.saturating_mul(min_element_len) > input.len() {
        return Err(DecodeError::UnexpectedEof);
    }

    Ok(len)
}

/// The fewest bytes a payload of `tag` takes.
fn min_payload_len(tag: u8) -> usize {
    match tag {
        1 | 10 => 1,
        2 | 8 => 2,
        3 | 5 | 7 | 11 | 12 => 4,
        4 | 6 => 8,
        9 => 5,
        _ => 0,
    }
}

/// Reads a Java modified UTF-8 string. Most strings are plain UTF-8 and
/// are borrowed; only NUL and characters outside the BMP need decoding.
fn read_string<'p>(input: &mut &'p [u8]) -> DecodeResult<Cow<'p, str>> {
    let len = u16::from_be_bytes(take_array(input)?);
    let bytes = take(input, usize::from(len))?;

    if let Ok(text) = str::from_utf8(bytes) {
        return Ok(Cow::Borrowed(text));
    }

    let units = modified_utf8_units(bytes)?;
    let text = String::from_utf16(&units).map_err(|_| DecodeError::InvalidLength)?;

    Ok(Cow::Owned(text))
}

/// Decodes modified UTF-8 into UTF-16 code units: one, two or three bytes
/// each, with NUL as two bytes and other characters as surrogate pairs.
fn modified_utf8_units(bytes: &[u8]) -> DecodeResult<Vec<u16>> {
    let mut units = Vec::with_capacity(bytes.len());
    let mut rest = bytes;

    while let [first, tail @ ..] = rest {
        let first = u16::from(*first);
        let (unit, len) = match (first, tail) {
            (0x01..=0x7f, _) => (first, 1),
            (0xc0..=0xdf, [second, ..]) if second & 0xc0 == 0x80 => (((first & 0x1f) << 6) | u16::from(second & 0x3f), 2),
            (0xe0..=0xef, [second, third, ..]) if second & 0xc0 == 0x80 && third & 0xc0 == 0x80 => {
                (((first & 0x0f) << 12) | (u16::from(second & 0x3f) << 6) | u16::from(third & 0x3f), 3)
            }
            _ => return Err(DecodeError::InvalidLength),
        };

        units.push(unit);
        rest = &rest[len..];
    }

    Ok(units)
}

#[cfg(test)]
#[path = "../../tests/tests_types_nbt.rs"]
mod tests;
