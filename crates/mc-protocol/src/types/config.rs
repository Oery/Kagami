use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::Nbt;

#[derive(Debug, IntoOwned)]
pub struct KnownPack<'p> {
    pub namespace: Cow<'p, str>,
    pub id: Cow<'p, str>,
    pub version: Cow<'p, str>,
}

impl<'p> PacketWriter<'p> for KnownPack<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let namespace = Cow::read(buf)?;
        let id = Cow::read(buf)?;
        let version = Cow::read(buf)?;

        Ok(KnownPack { namespace, id, version })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.namespace.encode(out)?;
        self.id.encode(out)?;
        self.version.encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.namespace.body_len() + self.id.body_len() + self.version.body_len()
    }
}

#[derive(Debug, IntoOwned)]
pub struct RegistryEntry<'p> {
    pub id: Cow<'p, str>,
    pub data: Option<Nbt<'p>>,
}

impl<'p> PacketWriter<'p> for RegistryEntry<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let id = Cow::read(buf)?;
        let has_data = bool::read(buf)?;
        let data = match has_data {
            true => Some(Nbt::read(buf)?),
            false => None,
        };

        Ok(RegistryEntry { id, data })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.id.encode(out)?;
        self.data.is_some().encode(out)?;

        if let Some(data) = &self.data {
            data.encode(out)?;
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        let payload = match &self.data {
            Some(data) => data.body_len(),
            None => 0,
        };

        self.id.body_len() + 1 + payload
    }
}

#[derive(Debug, IntoOwned)]
pub struct Tag<'p> {
    pub name: Cow<'p, str>,
    pub entries: Vec<crate::types::VarInt>,
}

impl<'p> PacketWriter<'p> for Tag<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let name = Cow::read(buf)?;
        let entries = <Vec<crate::types::VarInt>>::read(buf)?;

        Ok(Tag { name, entries })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.name.encode(out)?;
        self.entries.encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.entries.body_len()
    }
}

#[derive(Debug, IntoOwned)]
pub struct TaggedRegistry<'p> {
    pub registry: Cow<'p, str>,
    pub tags: Vec<Tag<'p>>,
}

impl<'p> PacketWriter<'p> for TaggedRegistry<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let registry = Cow::read(buf)?;
        let tags = <Vec<Tag>>::read(buf)?;

        Ok(TaggedRegistry { registry, tags })
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.registry.encode(out)?;
        self.tags.encode(out)?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.registry.body_len() + self.tags.body_len()
    }
}
