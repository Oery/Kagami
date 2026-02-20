use serde::Deserialize;
use serde::Serialize;
use strum::FromRepr;

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
