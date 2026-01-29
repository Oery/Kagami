use crate::events::Context;
use crate::packets::*;

pub type PacketEvent<T> = Box<dyn for<'a> Fn(&mut Context<T>) + Send + Sync + 'static>;

// TODO: Find alternative
// -- Generics type can't use the lifetime given by the hrtb inside the type
//
type HandshakeEvent = Box<dyn for<'a> Fn(&mut Context<client::Handshake<'a>>) + Send + Sync + 'static>;
type LoginSuccessEvent = Box<dyn for<'a> Fn(&mut Context<server::LoginSuccess<'a>>) + Send + Sync + 'static>;
type ClientChatEvent = Box<dyn for<'a> Fn(&mut Context<client::Chat<'a>>) + Send + Sync + 'static>;
type ServerChatEvent = Box<dyn for<'a> Fn(&mut Context<server::Chat<'a>>) + Send + Sync + 'static>;

#[derive(Default)]
pub struct PacketEvents {
    pub client_handshake: Vec<HandshakeEvent>,
    pub client_keep_alive: Vec<PacketEvent<client::KeepAlive>>,
    pub client_chat: Vec<ClientChatEvent>,
    pub server_set_compression: Vec<PacketEvent<server::SetCompression>>,
    pub server_login_success: Vec<LoginSuccessEvent>,
    pub server_chat: Vec<ServerChatEvent>,
}
