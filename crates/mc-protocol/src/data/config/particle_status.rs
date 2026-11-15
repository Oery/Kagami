use crate::error::DecodeError;

#[derive(Debug, Copy, Clone)]
pub enum ParticleStatus {
    All,
    Decreased,
    Minimal,
}

impl TryFrom<i32> for ParticleStatus {
    type Error = DecodeError;

    fn try_from(val: i32) -> Result<ParticleStatus, Self::Error> {
        match val {
            0 => Ok(ParticleStatus::All),
            1 => Ok(ParticleStatus::Decreased),
            2 => Ok(ParticleStatus::Minimal),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}
