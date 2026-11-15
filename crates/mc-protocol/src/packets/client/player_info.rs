//! The player list update of protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;
use uuid::Uuid;

use crate::decode::{MAX_COLLECTION_LEN, Result as DecodeResult, read_length};
use crate::encode::Result as EncodeResult;
use crate::error::EncodeError;
use crate::packets::{PacketWriter, Payload};
use crate::types::{Nbt, ProfileProperty, VarInt};
use crate::{Direction, State};

/// [`PlayerInfoUpdate`] action bits. Every entry carries the fields of each
/// set bit, in bit order.
pub mod player_info_action {
    pub const ADD_PLAYER: u8 = 0x01;
    pub const INITIALIZE_CHAT: u8 = 0x02;
    pub const UPDATE_GAME_MODE: u8 = 0x04;
    pub const UPDATE_LISTED: u8 = 0x08;
    pub const UPDATE_LATENCY: u8 = 0x10;
    pub const UPDATE_DISPLAY_NAME: u8 = 0x20;
    pub const UPDATE_LIST_PRIORITY: u8 = 0x40;
    pub const UPDATE_HAT: u8 = 0x80;
}

use player_info_action::*;

/// A player's chat signing key, as the client registered it with Player Session.
#[derive(Debug, IntoOwned)]
pub struct ChatSession<'p> {
    pub session_id: Uuid,
    /// Milliseconds since the Unix epoch.
    pub expires_at: i64,
    pub public_key: Cow<'p, [u8]>,
    pub key_signature: Cow<'p, [u8]>,
}

impl<'p> PacketWriter<'p> for ChatSession<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let session_id = Uuid::read(input)?;
        let expires_at = i64::read(input)?;
        let public_key = Cow::read(input)?;
        let key_signature = Cow::read(input)?;

        Ok(Self { session_id, expires_at, public_key, key_signature })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.session_id.encode(out)?;
        self.expires_at.encode(out)?;
        self.public_key.encode(out)?;

        self.key_signature.encode(out)
    }

    fn body_len(&self) -> usize {
        16 + 8 + self.public_key.body_len() + self.key_signature.body_len()
    }
}

/// The profile part of [`ADD_PLAYER`]; the UUID is the entry's.
#[derive(Debug, IntoOwned)]
pub struct AddedPlayer<'p> {
    pub name: Cow<'p, str>,
    pub properties: Vec<ProfileProperty<'p>>,
}

impl<'p> PacketWriter<'p> for AddedPlayer<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let name = Cow::read(input)?;
        let properties = Vec::read(input)?;

        Ok(Self { name, properties })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.name.encode(out)?;
        self.properties.encode(out)
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.properties.body_len()
    }
}

/// One player's changes. A field is `Some` exactly when its action is set
/// on the packet. When encoding, fields of unset actions are ignored and a
/// missing field of a set action is an error.
#[derive(Debug, Default, IntoOwned)]
pub struct PlayerInfoEntry<'p> {
    pub uuid: Uuid,
    pub added: Option<AddedPlayer<'p>>,
    /// The inner `None` removes the player's chat session.
    pub chat_session: Option<Option<ChatSession<'p>>>,
    pub game_mode: Option<i32>,
    pub listed: Option<bool>,
    /// Milliseconds; negative when not known yet.
    pub latency: Option<i32>,
    /// The inner `None` restores the player's name.
    pub display_name: Option<Option<Nbt<'p>>>,
    /// Higher priorities are listed first.
    pub list_priority: Option<i32>,
    pub show_hat: Option<bool>,
}

impl<'p> PlayerInfoEntry<'p> {
    fn read(input: &mut &'p [u8], actions: u8) -> DecodeResult<Self> {
        let uuid = Uuid::read(input)?;
        let added = read_if(input, actions, ADD_PLAYER)?;
        let chat_session = read_if(input, actions, INITIALIZE_CHAT)?;
        let game_mode = read_if::<VarInt>(input, actions, UPDATE_GAME_MODE)?.map(|mode| mode.0);
        let listed = read_if(input, actions, UPDATE_LISTED)?;
        let latency = read_if::<VarInt>(input, actions, UPDATE_LATENCY)?.map(|latency| latency.0);
        let display_name = read_if(input, actions, UPDATE_DISPLAY_NAME)?;
        let list_priority = read_if::<VarInt>(input, actions, UPDATE_LIST_PRIORITY)?.map(|priority| priority.0);
        let show_hat = read_if(input, actions, UPDATE_HAT)?;

        Ok(Self { uuid, added, chat_session, game_mode, listed, latency, display_name, list_priority, show_hat })
    }

    fn encode(&self, out: &mut Vec<u8>, actions: u8) -> EncodeResult {
        self.uuid.encode(out)?;
        write_if(out, actions, ADD_PLAYER, self.added.as_ref())?;
        write_if(out, actions, INITIALIZE_CHAT, self.chat_session.as_ref())?;
        write_if(out, actions, UPDATE_GAME_MODE, self.game_mode.map(VarInt).as_ref())?;
        write_if(out, actions, UPDATE_LISTED, self.listed.as_ref())?;
        write_if(out, actions, UPDATE_LATENCY, self.latency.map(VarInt).as_ref())?;
        write_if(out, actions, UPDATE_DISPLAY_NAME, self.display_name.as_ref())?;
        write_if(out, actions, UPDATE_LIST_PRIORITY, self.list_priority.map(VarInt).as_ref())?;
        write_if(out, actions, UPDATE_HAT, self.show_hat.as_ref())?;

        Ok(())
    }

    fn body_len(&self, actions: u8) -> usize {
        let mut length = 16;

        length += len_if(actions, ADD_PLAYER, self.added.as_ref());
        length += len_if(actions, INITIALIZE_CHAT, self.chat_session.as_ref());
        length += len_if(actions, UPDATE_GAME_MODE, self.game_mode.map(VarInt).as_ref());
        length += len_if(actions, UPDATE_LISTED, self.listed.as_ref());
        length += len_if(actions, UPDATE_LATENCY, self.latency.map(VarInt).as_ref());
        length += len_if(actions, UPDATE_DISPLAY_NAME, self.display_name.as_ref());
        length += len_if(actions, UPDATE_LIST_PRIORITY, self.list_priority.map(VarInt).as_ref());
        length += len_if(actions, UPDATE_HAT, self.show_hat.as_ref());

        length
    }
}

fn read_if<'p, T: PacketWriter<'p>>(input: &mut &'p [u8], actions: u8, action: u8) -> DecodeResult<Option<T>> {
    if actions & action == 0 {
        return Ok(None);
    }

    let value = T::read(input)?;

    Ok(Some(value))
}

fn write_if<'p, T: PacketWriter<'p>>(out: &mut Vec<u8>, actions: u8, action: u8, field: Option<&T>) -> EncodeResult {
    if actions & action == 0 {
        return Ok(());
    }

    let Some(value) = field else {
        return Err(EncodeError::MissingField);
    };

    value.encode(out)
}

fn len_if<'p, T: PacketWriter<'p>>(actions: u8, action: u8, field: Option<&T>) -> usize {
    if actions & action == 0 {
        return 0;
    }

    field.map_or(0, PacketWriter::body_len)
}

/// Adds players to the client's player list or changes their entries.
#[derive(Debug, IntoOwned)]
pub struct PlayerInfoUpdate<'p> {
    /// [`player_info_action`] bits, shared by every entry.
    pub actions: u8,
    pub entries: Vec<PlayerInfoEntry<'p>>,
}

impl<'p> Payload<'p> for PlayerInfoUpdate<'p> {
    const DIRECTION: Direction = Direction::Clientbound;
    const ID: VarInt = VarInt(0x47);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        let mut length = 1 + VarInt(self.entries.len() as i32).body_len();

        for entry in &self.entries {
            length += entry.body_len(self.actions);
        }

        Ok(length)
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let actions = u8::read(buf)?;
        let count = read_length(buf, MAX_COLLECTION_LEN)?;
        let mut entries = Vec::new();

        for _ in 0..count {
            let entry = PlayerInfoEntry::read(buf, actions)?;
            entries.push(entry);
        }

        Ok(Self { actions, entries })
    }

    fn encode_payload(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.actions.encode(out)?;
        VarInt(self.entries.len() as i32).encode(out)?;

        for entry in &self.entries {
            entry.encode(out, self.actions)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_player_info.rs"]
mod tests;
