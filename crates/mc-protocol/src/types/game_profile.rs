use futures_lite::AsyncWriteExt;
use uuid::Uuid;

use crate::PacketWriter;
use crate::error::DecodeError;

#[derive(Debug)]
pub struct ProfileProperty<'p> {
    pub name: &'p str,
    pub value: &'p str,
    pub signature: Option<&'p str>,
}

#[derive(Debug)]
pub struct GameProfile<'p> {
    pub uuid: Uuid,
    pub username: &'p str,
    pub properties: Vec<ProfileProperty<'p>>,
}

impl<'p> PacketWriter<'p> for GameProfile<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let uuid = Uuid::read(buf)?;
        let username: &str = <&str>::read(buf)?;
        let properties = <Vec<ProfileProperty>>::read(buf)?;

        Ok(GameProfile { uuid, username, properties })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.uuid.write(w).await?;
        self.username.write(w).await?;
        self.properties.write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.uuid.body_len() + self.username.body_len() + self.properties.body_len()
    }
}

impl<'p> PacketWriter<'p> for ProfileProperty<'p> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let name: &str = <&str>::read(buf)?;
        let value: &str = <&str>::read(buf)?;
        let signature: Option<&str> = <Option<&str>>::read(buf)?;

        Ok(ProfileProperty { name, value, signature })
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.name.write(w).await?;
        self.value.write(w).await?;
        self.signature.write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.value.body_len() + self.signature.body_len()
    }
}
