//! Player login, tab profile and local skin metadata codecs.

use futures_lite::AsyncWrite;
use uuid::Uuid;

use crate::State;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::{PacketWriter, Payload};
use crate::types::{ProfileProperty, VarInt};

pub const PLAYER_ENTITY_ID: i32 = 1;

pub struct PlayLogin<'p> {
    pub entity_id: i32,
    pub hardcore: bool,
    pub dimension_names: Vec<&'p str>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: &'p str,
    pub hashed_seed: i64,
    pub game_mode: VarInt,
    pub previous_game_mode: VarInt,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforces_secure_chat: bool,
}

impl Default for PlayLogin<'_> {
    fn default() -> Self {
        Self {
            entity_id: PLAYER_ENTITY_ID,
            hardcore: false,
            dimension_names: vec!["minecraft:overworld"],
            max_players: 20,
            view_distance: 8,
            simulation_distance: 8,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: "minecraft:overworld",
            hashed_seed: 0,
            game_mode: VarInt(0),
            previous_game_mode: VarInt(0),
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            portal_cooldown: VarInt(0),
            sea_level: VarInt(63),
            enforces_secure_chat: false,
        }
    }
}

impl<'p> Payload<'p> for PlayLogin<'p> {
    const ID: VarInt = VarInt(0x32);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        let mut length = 0;
        length += self.entity_id.body_len();
        length += self.hardcore.body_len();
        length += self.dimension_names.body_len();
        length += VarInt(self.max_players).body_len();
        length += VarInt(self.view_distance).body_len();
        length += VarInt(self.simulation_distance).body_len();
        length += self.reduced_debug_info.body_len();
        length += self.enable_respawn_screen.body_len();
        length += self.do_limited_crafting.body_len();
        length += self.dimension_type.body_len();
        length += self.dimension_name.body_len();
        length += self.hashed_seed.body_len();
        length += self.game_mode.body_len();
        length += self.previous_game_mode.body_len();
        length += self.is_debug.body_len();
        length += self.is_flat.body_len();
        length += self.has_death_location.body_len();
        length += self.portal_cooldown.body_len();
        length += self.sea_level.body_len();
        length += false.body_len();
        length += self.enforces_secure_chat.body_len();
        Ok(length)
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let entity_id = i32::read(buf)?;
        let hardcore = bool::read(buf)?;
        let dimension_names = <Vec<&str>>::read(buf)?;
        let max_players = VarInt::read(buf)?.0;
        let view_distance = VarInt::read(buf)?.0;
        let simulation_distance = VarInt::read(buf)?.0;
        let reduced_debug_info = bool::read(buf)?;
        let enable_respawn_screen = bool::read(buf)?;
        let do_limited_crafting = bool::read(buf)?;
        let dimension_type = VarInt::read(buf)?;
        let dimension_name = <&str>::read(buf)?;
        let hashed_seed = i64::read(buf)?;
        let game_mode = VarInt::read(buf)?;
        let previous_game_mode = VarInt::read(buf)?;
        let is_debug = bool::read(buf)?;
        let is_flat = bool::read(buf)?;
        let has_death_location = bool::read(buf)?;
        if has_death_location {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        let portal_cooldown = VarInt::read(buf)?;
        let sea_level = VarInt::read(buf)?;
        let _online_mode = bool::read(buf)?;
        let enforces_secure_chat = bool::read(buf)?;

        Ok(Self {
            entity_id,
            hardcore,
            dimension_names,
            max_players,
            view_distance,
            simulation_distance,
            reduced_debug_info,
            enable_respawn_screen,
            do_limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_game_mode,
            is_debug,
            is_flat,
            has_death_location,
            portal_cooldown,
            sea_level,
            enforces_secure_chat,
        })
    }

    async fn write_payload<W>(&self, w: &mut W) -> EncodeResult
    where
        W: AsyncWrite + Unpin,
    {
        self.entity_id.write(w).await?;
        self.hardcore.write(w).await?;
        self.dimension_names.write(w).await?;
        VarInt(self.max_players).write(w).await?;
        VarInt(self.view_distance).write(w).await?;
        VarInt(self.simulation_distance).write(w).await?;
        self.reduced_debug_info.write(w).await?;
        self.enable_respawn_screen.write(w).await?;
        self.do_limited_crafting.write(w).await?;
        self.dimension_type.write(w).await?;
        self.dimension_name.write(w).await?;
        self.hashed_seed.write(w).await?;
        self.game_mode.write(w).await?;
        self.previous_game_mode.write(w).await?;
        self.is_debug.write(w).await?;
        self.is_flat.write(w).await?;
        self.has_death_location.write(w).await?;
        self.portal_cooldown.write(w).await?;
        self.sea_level.write(w).await?;
        true.write(w).await?;
        self.enforces_secure_chat.write(w).await?;

        Ok(())
    }
}

/// Play `Player Info Update` (`0x47`) with the `Add Player` (`0x01`) and
/// `Update Listed` (`0x08`) actions. Same properties as `Login Success`;
/// this is the packet the client renders the skin from.
#[derive(Debug)]
pub struct PlayerInfoUpdate<'p> {
    pub uuid: Uuid,
    pub username: &'p str,
    pub properties: Vec<ProfileProperty<'p>>,
}

impl<'p> Payload<'p> for PlayerInfoUpdate<'p> {
    const ID: VarInt = VarInt(0x47);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        let add = self.username.body_len() + self.properties.body_len();
        Ok(1 + VarInt(1).body_len() + 16 + add + 1)
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        if u8::read(buf)? != 0x09 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        if VarInt::read(buf)?.0 != 1 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        let uuid = Uuid::read(buf)?;
        let username = <&str>::read(buf)?;
        let properties = <Vec<ProfileProperty>>::read(buf)?;
        if !bool::read(buf)? {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }

        Ok(Self { uuid, username, properties })
    }

    async fn write_payload<W>(&self, w: &mut W) -> EncodeResult
    where
        W: AsyncWrite + Unpin,
    {
        // Add Player | Update Listed.
        0x09u8.write(w).await?;
        VarInt(1).write(w).await?;
        self.uuid.write(w).await?;
        // Add Player entry: name + properties, no top-level UUID echo.
        self.username.write(w).await?;
        self.properties.write(w).await?;
        // Update Listed: visible on the tab list.
        true.write(w).await?;

        Ok(())
    }
}

/// Play `Set Entity Metadata` (`0x65`) carrying only the Avatar
/// `Displayed Skin Parts` byte (index 16). The client renders jacket /
/// sleeves / pants / hat / cape layers from this, not from the
/// `Client Information` packet itself.
#[derive(Debug)]
pub struct SetEntityMetadata {
    pub entity_id: i32,
    pub skin_parts: u8,
}

impl<'p> Payload<'p> for SetEntityMetadata {
    const ID: VarInt = VarInt(0x65);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        Ok(VarInt(self.entity_id).body_len() + 1 + VarInt(0).body_len() + 1 + 1)
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let entity_id = VarInt::read(buf)?.0;
        if u8::read(buf)? != 16 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        if VarInt::read(buf)?.0 != 0 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        let skin_parts = u8::read(buf)?;
        if u8::read(buf)? != 0xFF {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }

        Ok(Self { entity_id, skin_parts })
    }

    async fn write_payload<W>(&self, w: &mut W) -> EncodeResult
    where
        W: AsyncWrite + Unpin,
    {
        VarInt(self.entity_id).write(w).await?;
        // index 16, type Byte (0), value, terminator.
        16u8.write(w).await?;
        VarInt(0).write(w).await?;
        self.skin_parts.write(w).await?;
        0xFFu8.write(w).await?;

        Ok(())
    }
}

/// Play `Set Entity Metadata` (`0x65`) carrying only the Player absorption
/// amount (index 17, Float), shown as golden hearts on the owner's HUD.
#[derive(Debug, PartialEq)]
pub struct SetAbsorption {
    pub entity_id: i32,
    pub absorption: f32,
}

impl<'p> Payload<'p> for SetAbsorption {
    const ID: VarInt = VarInt(0x65);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        Ok(VarInt(self.entity_id).body_len() + 1 + VarInt(3).body_len() + 4 + 1)
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let entity_id = VarInt::read(buf)?.0;
        if u8::read(buf)? != 17 || VarInt::read(buf)?.0 != 3 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        let absorption = f32::read(buf)?;
        if u8::read(buf)? != 0xFF {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        Ok(Self { entity_id, absorption })
    }

    async fn write_payload<W>(&self, w: &mut W) -> EncodeResult
    where
        W: AsyncWrite + Unpin,
    {
        VarInt(self.entity_id).write(w).await?;
        17u8.write(w).await?;
        VarInt(3).write(w).await?;
        self.absorption.write(w).await?;
        0xFFu8.write(w).await
    }
}
