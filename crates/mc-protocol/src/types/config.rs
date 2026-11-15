use futures_lite::AsyncWriteExt;

use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::Nbt;

#[derive(Debug)]
pub struct KnownPack<'p> {
    pub namespace: &'p str,
    pub id: &'p str,
    pub version: &'p str,
}

impl<'p> PacketWriter<'p> for KnownPack<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let namespace: &str = <&str>::read(buf)?;
        let id: &str = <&str>::read(buf)?;
        let version: &str = <&str>::read(buf)?;

        Ok(KnownPack { namespace, id, version })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.namespace.write(w).await?;
        self.id.write(w).await?;
        self.version.write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.namespace.body_len() + self.id.body_len() + self.version.body_len()
    }
}

#[derive(Debug)]
pub struct RegistryEntry<'p> {
    pub id: &'p str,
    pub data: Option<Nbt<'p>>,
}

impl<'p> PacketWriter<'p> for RegistryEntry<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let id: &str = <&str>::read(buf)?;
        let has_data = bool::read(buf)?;
        let data = match has_data {
            true => Some(Nbt::read(buf)?),
            false => None,
        };

        Ok(RegistryEntry { id, data })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.id.write(w).await?;
        self.data.is_some().write(w).await?;
        if let Some(data) = &self.data {
            data.write(w).await?;
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

#[derive(Debug)]
pub struct Tag<'p> {
    pub name: &'p str,
    pub entries: Vec<crate::types::VarInt>,
}

impl<'p> PacketWriter<'p> for Tag<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let name: &str = <&str>::read(buf)?;
        let entries = <Vec<crate::types::VarInt>>::read(buf)?;

        Ok(Tag { name, entries })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.name.write(w).await?;
        self.entries.write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.entries.body_len()
    }
}

#[derive(Debug)]
pub struct TaggedRegistry<'p> {
    pub registry: &'p str,
    pub tags: Vec<Tag<'p>>,
}

impl<'p> PacketWriter<'p> for TaggedRegistry<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let registry: &str = <&str>::read(buf)?;
        let tags = <Vec<Tag>>::read(buf)?;

        Ok(TaggedRegistry { registry, tags })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.registry.write(w).await?;
        self.tags.write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.registry.body_len() + self.tags.body_len()
    }
}
