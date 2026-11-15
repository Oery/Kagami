use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;
use uuid::Uuid;

use crate::PacketWriter;
use crate::error::DecodeError;

#[derive(Debug, IntoOwned)]
pub struct ProfileProperty<'p> {
    pub name: Cow<'p, str>,
    pub value: Cow<'p, str>,
    pub signature: Option<Cow<'p, str>>,
}

#[derive(Debug, IntoOwned)]
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
