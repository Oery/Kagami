use kagami_macros::Serializable;

use std::borrow::Cow;
use std::io::Write;
use std::time::Duration;

use nom::IResult;
use nom::bytes::streaming::take;
// use serde::Deserialize;
// use serde::Serialize;
use strum::FromRepr;

use crate::error::PResult;
use crate::packet::*;

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Player {
//     pub id: String,
//     pub name: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Players {
//     pub max: i32,
//     pub online: i32,
//     #[serde(default)]
//     pub sample: Vec<Player>,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Version {
//     pub name: String,
//     pub protocol: i32,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Response {
//     pub description: String,
//     pub players: Players,
//     pub version: Version,
//     pub favicon: String,
// }

#[repr(i32)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum Dimension {
    Nether = -1,
    Overworld = 0,
    End = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum AnimationKind {
    SwingArm,
    TakeDamage,
    LeaveBed,
    EatFood,
    CriticalEffect,
    MagicCriticalEffect,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum ChatPosition {
    Chat,
    System,
    HotBar,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr)]
pub enum PotionEffect {
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneration,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
}

#[derive(Debug, Clone, Serializable)]
#[my_repr(i32)]
pub enum EventKind<'a> {
    CombatEnter,
    CombatEnd {
        duration: Duration,
        entity_id: i32,
    },
    EntityDead {
        #[format = "varint"]
        player_id: i32,
        entity_id: i32,
        message: Cow<'a, str>,
    },
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

pub trait Serializable<'a>: Sized {
    fn serialize(&'a self, payload: &'a mut Vec<u8>) -> PResult<()>;
    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self>;
}

impl Serializable<'_> for Duration {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        let duration = self.as_millis() / 50;
        payload.write(&crate::varint::temp_convert(duration as i32)?)?;
        Ok(())
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = varint_i32(input)?;
        let duration = Duration::from_millis(value as u64 * 50);
        Ok((input, duration))
    }
}

impl Serializable<'_> for Position {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        let val = ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.y as u64 & 0xFFF) << 26)
            | (self.z as u64 & 0x3FFFFFF);

        payload.write(&val.to_be_bytes())?;
        Ok(())
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Position> {
        let (input, bytes) = take(8 as usize)(input)?;
        let val: u64 = u64::from_be_bytes(bytes.try_into().unwrap());

        let mut x = (val >> 38) as i32;
        let mut y = ((val >> 26) & 0xFFF) as i16;
        let mut z = (val & 0x3FFFFFF) as i32;

        if x >= 1 << 25 {
            x -= 1 << 26;
        }
        if y >= 1 << 11 {
            y -= 1 << 12;
        }
        if z >= 1 << 25 {
            z -= 1 << 26;
        }

        Ok((input, Position { x, y, z }))
    }
}

// #[derive(Debug, Serializable)]
// pub enum ActionKind {
//     StartSneaking,
//     StopSneaking,
//     LeaveBed,
//     StartSprinting,
//     StopSprinting,
//     JumpWithHorse {
//         #[format = "varint"]
//         boost: i32,
//     },
//     OpenRiddenHorseInventory,
// }
//
// impl ActionKind {
//     pub fn deserialize(input: &[u8]) -> IResult<&[u8], ActionKind> {
//         let (input, val) = varint_i32(input)?;
//
//         let (input, action) = match val {
//             0 => (input, ActionKind::StartSneaking),
//             1 => (input, ActionKind::StopSneaking),
//             2 => (input, ActionKind::LeaveBed),
//             3 => (input, ActionKind::StartSprinting),
//             4 => (input, ActionKind::StopSprinting),
//             5 => {
//                 let (input, val) = varint_i32(input)?;
//                 (input, ActionKind::JumpWithHorse { boost: val })
//             }
//             6 => (input, ActionKind::OpenRiddenHorseInventory),
//             _ => return Err(nom::Err::Failure(Error::new(input, ErrorKind::Fail))),
//         };
//
//         Ok((input, action))
//     }
// }
