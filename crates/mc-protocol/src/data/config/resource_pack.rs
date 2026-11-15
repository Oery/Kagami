use crate::error::DecodeError;

#[derive(Debug, Copy, Clone)]
pub enum ResourcePackResult {
    SuccessfullyDownloaded,
    Declined,
    FailedToDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedToReload,
    Discarded,
}

impl TryFrom<i32> for ResourcePackResult {
    type Error = DecodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SuccessfullyDownloaded),
            1 => Ok(Self::Declined),
            2 => Ok(Self::FailedToDownload),
            3 => Ok(Self::Accepted),
            4 => Ok(Self::Downloaded),
            5 => Ok(Self::InvalidUrl),
            6 => Ok(Self::FailedToReload),
            7 => Ok(Self::Discarded),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}
