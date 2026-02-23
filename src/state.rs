use atomic_enum::atomic_enum;
use kagami_macros::Serializable;

use crate::traits::Serializable;

#[atomic_enum]
#[derive(PartialEq, Serializable)]
pub enum McState {
    Handshake,
    Status,
    Login,
    Play,
}
