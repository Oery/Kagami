use std::borrow::Cow;
use std::sync::atomic::Ordering;

use crate::context::ProxyContext;
use crate::error::PResult;
use crate::events::*;
use crate::packet::{short, state, string, varint_i32};
use crate::proxy::Payload;
use crate::state::State;

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

    fn serialize(&self, _buf: &mut [u8]) -> PResult<()> {
        // write_varint_i32(buf, id)?;
        // write_varint_i32(raw_payload, payload.id)?;
        //
        todo!("Add Serialization support");
    }

    fn dispatch(self, ctx: &mut ProxyContext) {
        let mut packet_ctx = Context::new(self, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.client_keepalive {
            event(&mut packet_ctx);
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

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.client_handshake.is_empty()
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (input, protocol_version) = varint_i32(input)?;
        let (input, addr) = string(input)?;
        let (input, port) = short(input)?;
        let (_, next_state) = state(input)?;

        Ok(Self { protocol_version, addr, port, next_state })
    }

    fn serialize(&self, _buf: &mut [u8]) -> PResult<()> {
        todo!("Add Serialization support");
    }

    fn dispatch(self, ctx: &mut ProxyContext) {
        ctx.state.store(self.next_state, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.client_handshake {
            event(&mut pctx);
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

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.server_setcompression.is_empty()
    }

    fn serialize(&self, _buf: &mut [u8]) -> PResult<()> {
        todo!("Add Serialization support");
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (_, threshold) = varint_i32(input)?;
        Ok(Self { threshold })
    }

    fn dispatch(self, ctx: &mut ProxyContext) {
        ctx.compress_threshold.store(self.threshold, Ordering::Relaxed);
        println!("Compression was set to {}", self.threshold);
        let mut pctx = Context::new(self, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.server_setcompression {
            event(&mut pctx);
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

    fn has_events(em: &EventManager) -> bool {
        !em.packet_events.server_loginsuccess.is_empty()
    }

    fn deserialize(input: &'a [u8]) -> PResult<Self> {
        let (input, uuid) = string(input)?;
        let (_, username) = string(input)?;

        Ok(Self { uuid, username })
    }

    fn serialize(&self, _buf: &mut [u8]) -> PResult<()> {
        Ok(())
    }

    fn dispatch(self, ctx: &mut ProxyContext) {
        ctx.state.store(State::Play, Ordering::Relaxed);
        let mut pctx = Context::new(self, &mut ctx.src);

        for event in &ctx.proxy.events.packet_events.server_loginsuccess {
            event(&mut pctx);
        }
    }
}
