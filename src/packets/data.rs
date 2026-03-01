use kagami_macros::Serializable;

use std::borrow::Cow;
use std::io::Write;
use std::time::Duration;

use nom::IResult;
use nom::bytes::streaming::take;

use crate::error::PResult;
use crate::packet::*;
use crate::traits::Serializable;

#[derive(Debug, Clone, Copy, Serializable)]
pub enum Dimension {
    Nether = -1,
    Overworld,
    End,
}

#[derive(Debug, Clone, Copy, Serializable)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy, Serializable)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[derive(Debug, Clone, Copy, Serializable)]
pub enum AnimationKind {
    SwingArm,
    TakeDamage,
    LeaveBed,
    EatFood,
    CriticalEffect,
    MagicCriticalEffect,
}

#[derive(Debug, Clone, Copy, Serializable)]
pub enum ChatPosition {
    Chat,
    System,
    HotBar,
}

#[derive(Debug, Clone, Copy, Serializable)]
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

impl Serializable<'_> for Duration {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()> {
        let duration = self.as_millis() / 50;
        payload.write_all(&crate::varint::temp_convert(duration as i32)?)?;
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

        payload.write_all(&val.to_be_bytes())?;
        Ok(())
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Position> {
        let (input, bytes) = take(8_usize)(input)?;
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

#[derive(Debug, Serializable)]
pub enum ActionKind {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    JumpWithHorse,
    OpenRiddenHorseInventory,
}

#[derive(Debug, Serializable)]
pub enum InteractionKind {
    Interact,
    Attack,
    InteractAt { x: f32, y: f32, z: f32 },
}

#[derive(Debug, Serializable)]
pub enum DiggingStatus {
    Started,
    Cancelled,
    Finished,
    DropItem,
    DropStack,
    FinishSlow,
}

#[derive(Debug, Serializable)]
pub enum ChatMode {
    Shown,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, Serializable)]
pub enum ClientStatusAction {
    Respawn,
    RequestStats,
    OpenInventory,
}

// TODO: Use duration for fade
#[derive(Debug, Serializable)]
pub enum GameStateReason {
    InvalidBed {
        x: f32,
    },
    RainingEnd {
        x: f32,
    },
    RainingBegin {
        x: f32,
    },
    ChangeGamemode {
        #[enum_as = "f32"]
        game_mode: GameMode,
    },
    EnterCredits {
        x: f32,
    },
    DemoMessage {
        x: f32,
    },
    ArrowHit {
        x: f32,
    },
    FadeValue {
        darkness: f32,
    },
    FadeTime {
        fade_duration: f32,
    },
    PlayMobAppearance {
        x: f32,
    },
}
