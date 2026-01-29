use crate::context::ProxyContext;
use crate::error::{KResult, PResult, PacketError};
use crate::events::{Context, EventManager};
use crate::packet::{Packet, packet};
use crate::packets::{Chat, Handshake, LoginSuccess, ServerChat, SetCompression};
use crate::state::McState;
use crate::varint::temp_convert;

use std::io::Write;

use async_std::io::WriteExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::net::Shutdown;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tracing::info;

const HOST: &str = "127.0.0.1:25565";
const PROXY: &str = "127.0.0.1:25566";
const BUFFER_SIZE: usize = 10_000_000; // TODO: Measure what is needed

use std::fmt::Debug;

// TODO: Unstable
// -- Check unstable associated type defaults feature flag

pub trait Payload<'a>: Debug + Sized {
    type Item<'b>: 'b;
    type Handler;

    fn deserialize(raw_payload: &'a [u8]) -> PResult<Self>;
    fn serialize(&self) -> PResult<Packet<'_>>;
    fn has_events(em: &EventManager) -> bool;
    fn dispatch(self, ctx: &mut ProxyContext) -> Option<Self>;
    fn register(
        em: &mut EventManager,
        f: Box<dyn for<'b> Fn(&mut Context<Self::Item<'b>>) + Send + Sync + 'static>,
    );
}

// TODO: Use a RingBuffer
// -- This would remove the need of copying bytes in an acc buffer

async fn handle_payload<'a, T: Payload<'a>>(
    ctx: &mut ProxyContext<'_>,
    packet: &'a Packet<'a>,
) -> PResult<()> {
    if !T::has_events(&ctx.proxy.events) {
        write_packet(ctx, packet).await?;
        return Ok(());
    }

    let payload = T::deserialize(&packet.raw_payload)?;

    if let Some(payload) = payload.dispatch(ctx) {
        let packet = payload.serialize()?;
        write_packet(ctx, &packet).await?;
    }

    Ok(())
}

async fn handle_packet(ctx: &mut ProxyContext<'_>, packet: &Packet<'_>) -> Result<(), PacketError> {
    // println!("=========================================");
    // println!("Received Packet: {packet:?}");
    let state = ctx.state.mc_state.load(Ordering::Relaxed);
    // println!("State: {state:?}, Source: {:?}", ctx.source);

    use crate::context::Source::*;

    match (packet.id, &ctx.source, state) {
        (0x00, Client, McState::Handshake) => handle_payload::<Handshake>(ctx, packet).await,
        (0x01, Client, McState::Play) => handle_payload::<Chat>(ctx, packet).await,
        (0x02, Server, McState::Login) => handle_payload::<LoginSuccess>(ctx, packet).await,
        (0x03, Server, McState::Login) => handle_payload::<SetCompression>(ctx, packet).await,
        (0x02, Server, McState::Play) => handle_payload::<ServerChat>(ctx, packet).await,
        (0x46, Server, McState::Play) => handle_payload::<SetCompression>(ctx, packet).await,

        _ => Err(PacketError::UnknownPacket),
    }
}

fn next_packet<'a, 'b>(ctx: &ProxyContext<'_>, input: &'b [u8]) -> PResult<Option<(&'b [u8], Packet<'b>)>> {
    let cmp = ctx.state.compress_threshold.load(Ordering::Relaxed);

    match packet(input, cmp) {
        Ok(data) => Ok(Some(data)),
        Err(nom::Err::Incomplete(_)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

async fn write_packet(ctx: &mut ProxyContext<'_>, packet: &Packet<'_>) -> std::io::Result<()> {
    let packet_id = temp_convert(packet.id)?;
    let packet_len = packet.raw_payload.len() + packet_id.len();
    let threshold = ctx.state.compress_threshold.load(Ordering::Relaxed);

    // println!("Writing Packet");

    // FIXME: Ugly Workaround
    // -- SetCompression is sent to the client after the state has changed
    // This causes the packet to be compressed before comp was enabled for client
    //  and results in a client crash
    // -- (Maybe set a flag to keep track of the state the packet arrived in)
    let state = ctx.state.mc_state.load(Ordering::Relaxed);
    let is_setcompression = packet.id == 3 && state == McState::Login;

    if threshold == -1 || is_setcompression {
        ctx.dst.writer.write(&temp_convert(packet_len as i32)?).await?;
        ctx.dst.writer.write(&packet_id).await?;
        ctx.dst.writer.write(&packet.raw_payload).await?;
        return Ok(());
    }

    // println!("PACKET_LENGTH={packet_len} | THRESHOLD={threshold}");
    if packet_len < threshold as usize {
        let packet_len = packet_len + 1;
        // println!("Writing Packet with ID = {} and size {packet_len}", packet.id);
        ctx.dst.writer.write(&temp_convert(packet_len as i32)?).await?;
        ctx.dst.writer.write(&temp_convert(0)?).await?;
        ctx.dst.writer.write(&packet_id).await?;
        ctx.dst.writer.write(&packet.raw_payload).await?;
        return Ok(());
    }

    // println!("Writing Compressed Packet");

    // FIXME: Where is uncompressed size ?
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&packet_id)?;
    e.write_all(&packet.raw_payload)?;
    let compressed = e.finish()?;

    let data_len = temp_convert(packet_len as i32)?;
    let packet_len = data_len.len() + compressed.len();

    // println!("Uncompressed size is {}", packet_len);
    // dbg!(&data_len);

    ctx.dst.writer.write(&temp_convert(packet_len as i32)?).await?;
    ctx.dst.writer.write(&data_len).await?;
    ctx.dst.writer.write(&compressed).await?;
    ctx.dst.writer.flush().await?;

    Ok(())
}

async fn parse_buf(ctx: &mut ProxyContext<'_>, bytes: &[u8], buf_acc: &mut Vec<u8>) -> KResult<()> {
    buf_acc.reserve(bytes.len());
    buf_acc.extend_from_slice(bytes);

    let mut bytes = buf_acc.as_slice();

    while let Some((slice, packet)) = next_packet(ctx, bytes)? {
        match handle_packet(ctx, &packet).await {
            Err(PacketError::Deserializing(e)) => return Err(PacketError::Deserializing(e).into()),
            Err(_) => write_packet(ctx, &packet).await?,
            Ok(_) => {}
        }

        bytes = slice;
    }

    let consumed = buf_acc.len() - bytes.len();
    buf_acc.drain(..consumed);

    ctx.dst.writer.flush().await?;
    ctx.src.writer.flush().await?;

    Ok(())
}

// TODO: Maybe rename this with a shorter name
async fn handle_stream(ctx: &mut ProxyContext<'_>) -> KResult<()> {
    let mut buf = vec![0; BUFFER_SIZE];
    let mut buf_acc = Vec::with_capacity(BUFFER_SIZE);

    loop {
        match ctx.src.reader.read(&mut buf).await? {
            0 => return Ok(()),
            n => parse_buf(ctx, &buf[..n], &mut buf_acc).await?,
        };
    }
}

#[derive(Default)]
pub struct Proxy {
    pub events: EventManager,
}

impl Proxy {
    pub fn new() -> Proxy {
        Proxy { ..Default::default() }
    }

    pub async fn run(self) -> KResult<()> {
        let listener = TcpListener::bind(PROXY).await?;
        let proxy_ref = Arc::new(self);

        while let Ok((client, _)) = listener.accept().await {
            let proxy = proxy_ref.clone();
            task::spawn(Proxy::on_client_join(client, proxy));
        }

        Ok(())
    }

    // TODO: Result to Handle
    // -- Handle this result to know why the connection ended, could be closed normally or
    // forcefully closed by an issue on our side, an io error, etc...

    pub async fn on_client_join(client: TcpStream, proxy: Arc<Proxy>) -> KResult<()> {
        let server = TcpStream::connect(HOST).await?;
        let _ = server.set_nodelay(true)?;
        let _ = client.set_nodelay(true)?;

        let (mut client_ctx, mut server_ctx) = ProxyContext::new(&client, &server, proxy);
        if let Err(e) = futures::try_join!(handle_stream(&mut client_ctx), handle_stream(&mut server_ctx)) {
            eprintln!("Unexpected Error: {e:?}");
        }
        // if let Err(KagamiError::PacketError(PacketError::Deserializing(nom::Err::Failure(e)))) =
        //     futures::try_join!(handle_stream(&mut client_ctx), handle_stream(&mut server_ctx))
        // {
        //     eprintln!("Unexpected Error: {:?}\n{:?}", e.code, e.input);
        // }

        let _ = client.shutdown(Shutdown::Both);
        let _ = server.shutdown(Shutdown::Both);

        info!("Connection closed");

        Ok(())
    }
}
