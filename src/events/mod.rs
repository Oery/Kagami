mod context;
pub use context::*;

mod manager;
pub use manager::*;

mod packet_events;
pub use packet_events::*;

use crate::proxy::Payload;

pub trait ServerPacket<'a>: Payload<'a> {}

pub trait ClientPacket<'a>: Payload<'a> {}
