use std::borrow::Cow;
use std::io::{Read, Write};
use std::str::from_utf8;

use flate2::read::ZlibDecoder;
use nom::IResult;
use nom::Parser;
use nom::bytes::streaming::take;
use nom::error::{Error, ErrorKind};
use nom::multi::many_m_n;
use serde::de::DeserializeOwned;

use crate::error::PResult;
use crate::traits::Serializable;

#[derive(Debug, Default)]
pub struct Packet<'a> {
    pub id: i32,
    pub raw_payload: Cow<'a, [u8]>,
}

pub fn int(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, bytes) = take(4_usize)(input)?;
    let val = i32::from_le_bytes(bytes.try_into().unwrap());

    Ok((input, val))
}

pub fn varint_i32(mut input: &[u8]) -> IResult<&[u8], i32> {
    let mut val: i32 = 0;
    let mut pos = 0;

    loop {
        let (new_input, byte) = take(1_usize)(input)?;
        input = new_input;
        val |= ((byte[0] & 0x7F) as i32) << pos;

        if byte[0] & 0x80 == 0 {
            return Ok((input, val));
        }

        pos += 7;

        if pos >= 32 {
            return Err(nom::Err::Failure(Error { input, code: ErrorKind::TooLarge }));
        }
    }
}

pub fn short(input: &[u8]) -> IResult<&[u8], i16> {
    let (input, bytes) = take(2_usize)(input)?;
    let val = i16::from_be_bytes(bytes.try_into().unwrap());

    Ok((input, val))
}

// TODO: Test this deserializer
pub fn float(input: &[u8]) -> IResult<&[u8], f32> {
    let (input, bytes) = take(4_usize)(input)?;
    let val = f32::from_be_bytes(bytes.try_into().unwrap());

    Ok((input, val))
}

pub fn double(input: &[u8]) -> IResult<&[u8], f64> {
    let (input, bytes) = take(8_usize)(input)?;
    let val = f64::from_be_bytes(bytes.try_into().unwrap());

    Ok((input, val))
}

pub fn string(input: &[u8]) -> IResult<&[u8], Cow<'_, str>> {
    let (input, length) = varint_i32(input)?;
    let (input, content) = take(length as usize)(input)?;

    let Ok(string) = from_utf8(content) else {
        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail)));
    };

    Ok((input, Cow::from(string)))
}

// TODO: Handle that unwrap
pub fn json<T: DeserializeOwned>(input: &[u8]) -> IResult<&[u8], T> {
    let (input, json) = string(input)?;
    let v: T = serde_json::from_str(json.as_ref()).unwrap();

    Ok((input, v))
}

pub fn packet(input: &[u8], cmp: i32) -> IResult<&[u8], Packet<'_>> {
    // println!("INPUT: {input:?}");
    let (input, length) = varint_i32(input)?;
    let (input, packet) = take(length as usize)(input)?;

    // println!("a Packet: {length}");

    // TODO: Load CMP here
    // -> It would avoid loading it when a packet is not full

    // Compression is off
    if cmp == -1 {
        let (raw_payload, id) = varint_i32(packet)?;
        let raw_payload: Cow<'_, [u8]> = raw_payload.into();
        return Ok((input, Packet { id, raw_payload }));
    }

    // println!("Compression is enabled. Checking size...");

    let (data, size) = varint_i32(packet)?;
    // println!("Size: {size:?}");

    // Packet too small, not compressed
    if size == 0 {
        // println!("Packet was not compressed");
        let (raw_payload, id) = varint_i32(data)?;
        let raw_payload: Cow<'_, [u8]> = raw_payload.into();
        // println!("Packet: {id}");
        return Ok((input, Packet { id, raw_payload }));
    }

    // println!("Bytes: {:?}", &data[0..10]);

    let (_, (id, raw_payload)) = compressed_packet(data, size as usize)?;

    Ok((input, Packet { id, raw_payload: raw_payload.into() }))
}

pub fn compressed_packet(input: &[u8], size: usize) -> IResult<&[u8], (i32, Vec<u8>)> {
    // println!("Decompressing Packet of size {size}, Input size: {}", input.len());
    let mut e = ZlibDecoder::new(input);
    let mut data = vec![0; size];
    e.read_exact(&mut data).expect("Failed to read until end");

    let (raw_payload, id) = varint_i32(&data).expect("Failed to read id for compressed packet");
    // println!("Decompressed packet with id 0x{id:2X}");

    Ok((input, (id, raw_payload.to_owned())))
}

// TODO: Support Format && EnumAs
impl<'a, T: Serializable<'a>> Serializable<'a> for Vec<T> {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        payload.write_all(&crate::varint::temp_convert(self.len() as i32)?)?;

        for x in self {
            x.serialize(payload)?;
        }

        Ok(())
    }

    fn deserialize(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (input, n) = varint_i32(input)?;

        let (input, my_vec) = many_m_n(n as usize, n as usize, T::deserialize).parse(input)?;
        Ok((input, my_vec))
    }
}

impl<'a> Serializable<'a> for std::borrow::Cow<'a, str> {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        payload.write_all(&crate::varint::temp_convert(self.len() as i32)?)?;
        payload.write_all(self.as_bytes())?;

        Ok(())
    }

    fn deserialize(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        string(input)
    }
}

pub fn i32_to_varint(mut value: i32) -> Vec<u8> {
    let mut result = Vec::new();

    loop {
        let mut byte = (value & 0x7F) as u8; // Extract the lowest 7 bits
        value >>= 7; // Shift the value to process the next 7 bits

        if value != 0 {
            byte |= 0x80; // Set the continuation bit if more bytes follow
        }

        result.push(byte);

        if value == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::varint_i32;

    #[test]
    fn test_varint_i32_parsing() {
        let bytes = [128, 2];
        let result = varint_i32(&bytes).expect("Failed to parse varint").1;
        assert_eq!(result, 256);
    }

    #[test]
    fn test_varint_i32_parsing_long() {
        let bytes = [252, 174, 2];
        let result = varint_i32(&bytes).expect("Failed to parse varint").1;
        assert_eq!(result, 38_780);
    }
}
