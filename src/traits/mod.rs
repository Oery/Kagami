use nom::IResult;

use std::fmt::Debug;

use crate::context::ProxyContext;
use crate::error::PResult;
use crate::events::{Context, EventManager};
use crate::packet::Packet;

pub trait Serializable<'a>: Sized {
    fn serialize(&self, payload: &mut Vec<u8>) -> PResult<()>;
    fn deserialize(input: &'a [u8]) -> IResult<&'a [u8], Self>;
}

pub trait SerializablePacket<'a>: Sized + Serializable<'a> {
    fn deserialize_packet(raw_payload: &'a [u8]) -> PResult<Self>;
    fn serialize_packet(&self) -> PResult<Packet<'_>>;
}

pub trait Payload<'a>: Debug + Sized + SerializablePacket<'a> {
    type Item<'b>: 'b;
    type Handler;

    fn has_events(em: &EventManager) -> bool;
    fn register(
        em: &mut EventManager,
        f: Box<dyn for<'b> Fn(&mut Context<Self::Item<'b>>) + Send + Sync + 'static>,
    );
}

pub trait Dispatch<'a>: Sized + Payload<'a> + SerializablePacket<'a> {
    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self>;
}

pub trait ServerPacket<'a>: Payload<'a> {}

pub trait ClientPacket<'a>: Payload<'a> {}
