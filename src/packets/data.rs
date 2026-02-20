use std::io::Write;
use std::time::Duration;

use nom::IResult;
use nom::bytes::streaming::take;
use serde::Deserialize;
use serde::Serialize;
use strum::FromRepr;

use crate::error::PResult;
use crate::packet::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub description: String,
    pub players: Players,
    pub version: Version,
    pub favicon: String,
}

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

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

pub trait Serializable: Sized {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()>;
    fn deserialize(input: &[u8]) -> IResult<&[u8], Self>;
}

impl Serializable for Duration {
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

impl Serializable for Position {
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
