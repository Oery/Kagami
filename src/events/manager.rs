use crate::events::Context;
use crate::packets::*;
use crate::traits::Payload;

// Struct holding all events
include!(concat!(env!("OUT_DIR"), "/packet_events.rs"));

#[derive(Default)]
pub struct EventManager {
    pub packet_events: PacketEvents,
}

impl EventManager {
    pub fn on_packet<'a, T>(
        &mut self,
        handler: impl for<'b> Fn(&mut Context<T::Item<'b>>) + Send + Sync + 'static,
    ) where
        T: Payload<'a> + 'a,
    {
        T::register(self, Box::new(handler));
    }
}
