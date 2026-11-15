use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;
use uuid::Uuid;

use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::Either;

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct ProfileProperty<'p> {
    pub name: Cow<'p, str>,
    pub value: Cow<'p, str>,
    pub signature: Option<Cow<'p, str>>,
}

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct GameProfile<'p> {
    pub uuid: Uuid,
    pub username: Cow<'p, str>,
    pub properties: Vec<ProfileProperty<'p>>,
}

impl<'p> PacketWriter<'p> for GameProfile<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let uuid = Uuid::read(buf)?;
        let username = Cow::read(buf)?;
        let properties = <Vec<ProfileProperty>>::read(buf)?;

        Ok(GameProfile { uuid, username, properties })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.uuid.encode(out)?;
        self.username.encode(out)?;
        self.properties.encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.uuid.body_len() + self.username.body_len() + self.properties.body_len()
    }
}

impl<'p> PacketWriter<'p> for ProfileProperty<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let name = Cow::read(buf)?;
        let value = Cow::read(buf)?;
        let signature = Option::read(buf)?;

        Ok(ProfileProperty { name, value, signature })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.name.encode(out)?;
        self.value.encode(out)?;
        self.signature.encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.value.body_len() + self.signature.body_len()
    }
}

/// A profile the client may have to complete by asking Mojang, with
/// overrides of its skin. Shown by player heads and mannequins.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct ResolvableProfile<'p> {
    pub profile: Either<GameProfile<'p>, PartialProfile<'p>>,
    pub skin: SkinPatch<'p>,
}

/// A profile known only by name, by UUID, or by its properties.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct PartialProfile<'p> {
    pub name: Option<Cow<'p, str>>,
    pub uuid: Option<Uuid>,
    pub properties: Vec<ProfileProperty<'p>>,
}

/// Texture identifiers that replace the profile's own.
#[derive(Clone, Debug, Default, PartialEq, PacketWriter, IntoOwned)]
pub struct SkinPatch<'p> {
    pub body: Option<Cow<'p, str>>,
    pub cape: Option<Cow<'p, str>>,
    pub elytra: Option<Cow<'p, str>>,
    /// Whether the model has slim arms.
    pub slim: Option<bool>,
}
