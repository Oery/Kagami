use nom::bytes::streaming::take;
use serde::{Deserialize, Serialize};

use std::borrow::Cow;
use std::io::Write;
use std::sync::atomic::Ordering;

use crate::context::ProxyContext;
use crate::error::PResult;
use crate::events::*;
use crate::packet::{Packet, short, state, string, varint_i32};
use crate::proxy::Payload;
use crate::state::State;
use crate::varint::temp_convert;

// #[packet(0x05, Play, Client)]
#[derive(Debug)]
pub struct KeepAlive {
    pub id: i32,
}

impl Payload<'_> for KeepAlive {
    type Item<'b> = KeepAlive;
    type Handler = Box<dyn for<'b> Fn(&mut Context<Self::Item<'b>>) + Send + Sync + 'static>;

    fn register(em: &mut EventManager, f: Self::Handler) {
        em.packet_events.client_keepalive.push(f);
    }

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.client_keepalive.is_empty()
    }

    fn deserialize(raw_payload: &[u8]) -> PResult<Self> {
        let (_, id) = varint_i32(raw_payload)?;

        Ok(Self { id })
    }

    fn serialize(&self) -> PResult<Packet<'_>> {
        todo!("Add Serialization support");
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        let mut pctx = Context::new(self, &mut ctx.dst, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.client_keepalive {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}

#[derive(Debug)]
pub struct Handshake<'a> {
    pub protocol_version: i32,
    pub addr: Cow<'a, str>,
    pub port: i16,
    pub next_state: State,
}

impl<'a> Payload<'a> for Handshake<'a> {
    type Item<'b> = Handshake<'b>;
    type Handler = Box<dyn for<'b> Fn(&mut Context<Self::Item<'b>>) + Send + Sync + 'static>;

    fn register(em: &mut EventManager, f: Self::Handler) {
        em.packet_events.client_handshake.push(f);
    }

    fn has_events(_em: &EventManager) -> bool {
        true
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (input, protocol_version) = varint_i32(input)?;
        let (input, addr) = string(input)?;
        let (input, port) = short(input)?;
        let (_, next_state) = state(input)?;

        Ok(Self { protocol_version, addr, port, next_state })
    }

    fn serialize(&self) -> PResult<Packet<'_>> {
        Err(crate::error::PacketError::Unsupported)
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        ctx.state.store(self.next_state, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.src, &mut ctx.dst);

        for event in &ctx.proxy.events.packet_events.client_handshake {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}

#[derive(Debug)]
pub struct SetCompression {
    pub threshold: i32,
}

impl<'a> Payload<'a> for SetCompression {
    type Item<'b> = SetCompression;
    type Handler = PacketEvent<Self>;

    fn register(em: &mut EventManager, f: PacketEvent<Self>) {
        em.packet_events.server_setcompression.push(f);
    }

    fn has_events(_em: &EventManager) -> bool {
        true
    }

    fn serialize(&self) -> PResult<Packet<'_>> {
        Err(crate::error::PacketError::Unsupported)
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (_, threshold) = varint_i32(input)?;
        Ok(Self { threshold })
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        ctx.compress_threshold.store(self.threshold, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.dst, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.server_setcompression {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}

#[derive(Debug)]
pub struct LoginSuccess<'a> {
    pub uuid: Cow<'a, str>,
    pub username: Cow<'a, str>,
}

impl<'a> Payload<'a> for LoginSuccess<'a> {
    type Item<'b> = LoginSuccess<'b>;
    type Handler = Box<dyn for<'b> Fn(&mut Context<LoginSuccess<'b>>) + Send + Sync + 'static>;

    fn register(em: &mut EventManager, f: Self::Handler) {
        em.packet_events.server_loginsuccess.push(f);
    }

    fn has_events(_em: &EventManager) -> bool {
        true
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (input, uuid) = string(input)?;
        let (_, username) = string(input)?;

        Ok(Self { uuid, username })
    }

    fn serialize(&self) -> PResult<Packet<'_>> {
        Err(crate::error::PacketError::Unsupported)
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        ctx.state.store(State::Play, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.dst, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.server_loginsuccess {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}

#[derive(Debug)]
pub struct Chat<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> ClientPacket<'a> for Chat<'a> {}

impl<'a> Payload<'a> for Chat<'a> {
    type Item<'b> = Chat<'b>;
    type Handler = Box<dyn for<'b> Fn(&mut Context<Chat<'b>>) + Send + Sync + 'static>;

    fn register(em: &mut EventManager, f: Self::Handler) {
        em.packet_events.client_chat.push(f);
    }

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.client_chat.is_empty()
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (_, message) = string(input)?;

        Ok(Self { message })
    }

    // TODO: Check for self.message len and log a warning if it's larger than max mc size
    // > Then truncate the string (watchout if json)
    fn serialize(&self) -> PResult<Packet<'_>> {
        let mut raw_payload = vec![];
        raw_payload.write(&temp_convert(self.message.len() as i32)?)?;
        raw_payload.write(self.message.as_bytes())?;
        let raw_payload: Cow<'_, [u8]> = raw_payload.into();

        Ok(Packet { id: 0x01, raw_payload })
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        ctx.state.store(State::Play, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.src, &mut ctx.dst);

        for event in &ctx.proxy.events.packet_events.client_chat {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatComponent {}

#[derive(Debug)]
pub struct ServerChat<'a> {
    pub json: Cow<'a, str>,
    pub position: u8,
    // pub json: ChatComponent,
}

impl<'a> ServerPacket<'a> for ServerChat {}

impl<'a> Payload<'a> for ServerChat {
    type Item<'b> = ServerChat;
    type Handler = Box<dyn for<'b> Fn(&mut Context<ServerChat>) + Send + Sync + 'static>;

    fn register(em: &mut EventManager, f: Self::Handler) {
        em.packet_events.server_chat.push(f);
    }

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.server_chat.is_empty()
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (input, json) = string(input)?;
        let (_, position) = take(1usize)(input)?;
        let position = position[0];
        // let json = serde_json::from_str(&json).expect("String is not valid json");
        let json = json.to_string();

        Ok(Self { json, position })
    }

    fn serialize(&self) -> PResult<Packet<'_>> {
        let mut raw_payload = vec![];
        raw_payload.write(&temp_convert(self.json.len() as i32)?)?;
        raw_payload.write(self.json.as_bytes())?;
        raw_payload.write(&[self.position])?;
        let raw_payload: Cow<'_, [u8]> = raw_payload.into();

        Ok(Packet { id: 0x02, raw_payload })
    }

    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self> {
        ctx.state.store(State::Play, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.dst, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.server_chat {
            event(&mut pctx);
        }

        match pctx.cancel {
            true => None,
            false => Some(pctx.payload),
        }
    }
}
