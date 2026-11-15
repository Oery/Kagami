use crate::error::DecodeError;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum ChatMode {
    #[default]
    Enabled,
    CommandsOnly,
    Hidden,
}

impl TryFrom<i32> for ChatMode {
    type Error = DecodeError;

    fn try_from(val: i32) -> Result<ChatMode, Self::Error> {
        match val {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}
