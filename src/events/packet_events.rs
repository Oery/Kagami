use crate::events::Context;
use crate::packets::*;

type HandshakeEvent = Box<dyn for<'a> Fn(&mut Context<Handshake<'a>>) + Send + Sync + 'static>;
type KeepAliveEvent = Box<dyn for<'a> Fn(&mut Context<KeepAlive>) + Send + Sync + 'static>;
type SetCompressionEvent = Box<dyn for<'a> Fn(&mut Context<SetCompression>) + Send + Sync + 'static>;
type LoginSuccessEvent = Box<dyn for<'a> Fn(&mut Context<LoginSuccess<'a>>) + Send + Sync + 'static>;

#[derive(Default)]
pub struct PacketEvents {
    pub client_handshake: Vec<HandshakeEvent>,
    pub client_keepalive: Vec<KeepAliveEvent>,
    pub server_setcompression: Vec<SetCompressionEvent>,
    pub server_loginsuccess: Vec<LoginSuccessEvent>,
}
