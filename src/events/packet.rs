use crate::events::*;
use crate::packets::*;

pub trait PacketEvent<'a> {
    type Item<'b>: 'b;

    fn register(em: &mut EventManager, f: Box<dyn for<'b> Fn(&mut Context<Self::Item<'b>>) + Send + Sync + 'static>)
    where
        Self: Sized;
}

impl<'a> PacketEvent<'a> for Handshake<'a> {
    type Item<'b> = Handshake<'b>;

    fn register(em: &mut EventManager, f: Box<dyn for<'b> Fn(&mut Context<Handshake<'b>>) + Send + Sync + 'static>) {
        em.packet_events.client_handshake.push(f);
    }
}

impl<'a> PacketEvent<'a> for KeepAlive {
    type Item<'b> = KeepAlive;

    fn register(em: &mut EventManager, f: Box<dyn for<'b> Fn(&mut Context<KeepAlive>) + Send + Sync + 'static>) {
        em.packet_events.client_keepalive.push(f);
    }
}

impl<'a> PacketEvent<'a> for SetCompression {
    type Item<'b> = SetCompression;

    fn register(em: &mut EventManager, f: Box<dyn for<'b> Fn(&mut Context<SetCompression>) + Send + Sync + 'static>) {
        em.packet_events.server_setcompression.push(f);
    }
}
