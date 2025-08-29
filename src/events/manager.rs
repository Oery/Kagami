use crate::events::*;

#[derive(Default)]
pub struct EventManager {
    pub packet_events: PacketEvents,
}

impl EventManager {
    pub fn on_packet<'a, T>(
        &mut self,
        handler: impl for<'b> Fn(&mut Context<T::Item<'b>>) + Send + Sync + 'static,
    ) where
        T: PacketEvent<'a> + 'a,
    {
        T::register(self, Box::new(handler));
    }
}
