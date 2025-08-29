use std::ops::Deref;
use std::{fmt, io};

use serde::Deserialize;
use serde::Deserializer;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VarI32(i32);

impl AsRef<i32> for VarI32 {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl Deref for VarI32 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for VarI32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = VarI32;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("varint-encoded i32")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                dbg!(v);
                read_var_i32(v).map(VarI32).map_err(E::custom)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                read_var_i32(v.as_bytes()).map(VarI32).map_err(E::custom)
            }
        }

        deserializer.deserialize_bytes(Visitor)
    }
}

pub fn read_var_i32<R: io::Read>(mut r: R) -> Result<i32, io::Error> {
    let mut val: i32 = 0;
    let mut pos = 0;
    let mut byte;

    loop {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;
        byte = buf[0];
        dbg!(&byte);

        val |= ((byte & 0x7F) as i32) << pos;

        if byte & 0x80 == 0 {
            return Ok(val);
        }

        pos += 7;

        if pos >= 32 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "varint too large",
            ));
        }
    }
}

pub fn temp_convert(mut value: i32) -> io::Result<Vec<u8>> {
    let mut val = Vec::new();

    loop {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;

        if value == 0 && (byte & 0x80) == 0 {
            val.push(byte);
            break;
        }

        byte |= 0x80;
        val.push(byte);

        if value == 0 {
            break;
        }
    }

    Ok(val)
}

//
// impl<W: std::io::Write> VarIntWriter for W {
//     fn write_varint(&mut self, mut value: i32) -> io::Result<()> {
//         loop {
//             let mut temp = (value & SEGMENT_BITS as i32) as u8;
//             value >>= 7;
//
//             if value != 0 {
//                 temp |= CONTINUE_BIT;
//             }
//
//             self.write_all(&[temp])?;
//
//             if value == 0 {
//                 break;
//             }
//         }
//
//         Ok(())
//     }
// }
