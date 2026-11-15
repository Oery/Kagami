use crate::error::DecodeError;

#[derive(Debug, Copy, Clone)]
pub enum Hand {
    Left,
    Right,
}

impl TryFrom<i32> for Hand {
    type Error = DecodeError;

    fn try_from(val: i32) -> Result<Hand, Self::Error> {
        match val {
            0 => Ok(Hand::Left),
            1 => Ok(Hand::Right),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }
}
