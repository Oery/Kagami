use crate::error::EncodeError;

pub type Result<T = ()> = std::result::Result<T, EncodeError>;
