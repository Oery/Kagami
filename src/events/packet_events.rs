use crate::events::Context;
use crate::packets::*;

pub type PacketEvent<T> = Box<dyn for<'a> Fn(&mut Context<T>) + Send + Sync + 'static>;

// TODO: Find alternative
// -- Generics type can't use the lifetime given by the hrtb inside the type
//
type HandshakeEvent = Box<dyn for<'a> Fn(&mut Context<Handshake<'a>>) + Send + Sync + 'static>;
type LoginSuccessEvent = Box<dyn for<'a> Fn(&mut Context<LoginSuccess<'a>>) + Send + Sync + 'static>;
type ChatEvent = Box<dyn for<'a> Fn(&mut Context<Chat<'a>>) + Send + Sync + 'static>;

#[derive(Default)]
pub struct PacketEvents {
    pub client_handshake: Vec<HandshakeEvent>,
    pub client_keepalive: Vec<PacketEvent<KeepAlive>>,
    pub client_chat: Vec<ChatEvent>,
    pub server_setcompression: Vec<PacketEvent<SetCompression>>,
    pub server_loginsuccess: Vec<LoginSuccessEvent>,
    pub server_chat: Vec<PacketEvent<ServerChat>>,
}
