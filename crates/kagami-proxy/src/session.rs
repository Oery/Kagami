//! One client's session: the proxy answers the handshake, relays status and
//! login packet by packet, then pumps configuration and play.
//!
//! Compression is mirrored: when the server enables it, the proxy forwards
//! the same threshold to the client, so frames pass through both legs
//! unchanged.

use std::io;

use futures_lite::future::zip;
use glommio::net::TcpStream;
use mc_protocol::packets::client::LoginDisconnect;
use mc_protocol::packets::server::ServerListPing;
use mc_protocol::packets::{Clientbound, Serverbound};
use mc_protocol::state::HandshakeIntent;
use mc_protocol::{Direction, Payload, State};

use crate::Upstream;
use crate::connection::Connection;
use crate::frame::FrameBuffer;
use crate::invalid_data;
use crate::pump::{Wire, pump};
use crate::split::{ReadHalf, WriteHalf};

/// Serves one client connection until either side closes it.
pub async fn serve(client: TcpStream, upstream: &Upstream) -> io::Result<()> {
    let mut client = Connection::new(client)?;

    let Some(Serverbound::ServerListPing(handshake)) = client.read_serverbound(State::Handshake).await? else {
        return Err(invalid_data("expected a handshake"));
    };
    let version = handshake.version;
    let intent = handshake.next_state;

    let server = TcpStream::connect((upstream.host.as_str(), upstream.port)).await?;
    let mut server = Connection::new(server)?;

    // The server sees the address it is reached at, not the proxy's.
    let handshake = ServerListPing { version, addr: upstream.host.as_str().into(), port: upstream.port, next_state: intent };
    server.send(|out| handshake.encode(out)).await?;

    match intent {
        HandshakeIntent::Status => relay_status(&mut client, &mut server).await,
        HandshakeIntent::Login | HandshakeIntent::Transfer => {
            if !login(&mut client, &mut server).await? {
                return Ok(());
            }

            pump_both(client, server).await
        }
    }
}

/// Status is a strict request and response exchange.
async fn relay_status(client: &mut Connection, server: &mut Connection) -> io::Result<()> {
    while forward_serverbound(client, server, State::Status).await? {
        if !forward_clientbound(server, client, State::Status).await? {
            break;
        }
    }

    Ok(())
}

/// Relays login until both sides enter configuration. Returns `false` when
/// login ended without it.
async fn login(client: &mut Connection, server: &mut Connection) -> io::Result<bool> {
    let Some(Serverbound::LoginStart(login_start)) = client.read_serverbound(State::Login).await? else {
        return Err(invalid_data("expected login start"));
    };

    server.send(|out| login_start.encode(out)).await?;

    loop {
        let Some(packet) = server.read_clientbound(State::Login).await? else {
            return Ok(false);
        };

        match packet {
            Clientbound::LoginSuccess(_) => {
                client.send(|out| packet.encode(out)).await?;
                break;
            }
            Clientbound::LoginDisconnect(_) => {
                client.send(|out| packet.encode(out)).await?;
                return Ok(false);
            }
            // The client answers each request with exactly one response.
            Clientbound::LoginPluginRequest(_) | Clientbound::LoginCookieRequest(_) => {
                client.send(|out| packet.encode(out)).await?;

                if !forward_serverbound(client, server, State::Login).await? {
                    return Ok(false);
                }
            }
            Clientbound::EncryptionRequest(_) => {
                return refuse(client, "Kagami does not support online-mode servers yet").await;
            }
            Clientbound::SetCompression(compression) => {
                client.send(|out| compression.encode(out)).await?;

                client.set_compression(compression.threshold);
                server.set_compression(compression.threshold);
            }
            _ => return Err(invalid_data("unexpected login packet")),
        }
    }

    let Some(Serverbound::LoginAcknowledge(acknowledge)) = client.read_serverbound(State::Login).await? else {
        return Err(invalid_data("expected login acknowledgement"));
    };
    server.send(|out| acknowledge.encode(out)).await?;

    Ok(true)
}

/// Tells the client why login stopped, then fails the session with the same reason.
async fn refuse(client: &mut Connection, reason: &str) -> io::Result<bool> {
    let json = format!(r#"{{"text":"{reason}"}}"#);
    let disconnect = LoginDisconnect { reason: json.into() };
    client.send(|out| disconnect.encode(out)).await?;

    Err(io::Error::other(reason.to_owned()))
}

/// Sends the client's next packet to the server. Returns `false` at the end of the stream.
async fn forward_serverbound(client: &mut Connection, server: &mut Connection, state: State) -> io::Result<bool> {
    let Some(packet) = client.read_serverbound(state).await? else {
        return Ok(false);
    };
    server.send(|out| packet.encode(out)).await?;

    Ok(true)
}

/// Sends the server's next packet to the client. Returns `false` at the end of the stream.
async fn forward_clientbound(server: &mut Connection, client: &mut Connection, state: State) -> io::Result<bool> {
    let Some(packet) = server.read_clientbound(state).await? else {
        return Ok(false);
    };
    client.send(|out| packet.encode(out)).await?;

    Ok(true)
}

/// Pumps both directions, starting in configuration, until both have ended.
async fn pump_both(client: Connection, server: Connection) -> io::Result<()> {
    let client = client.into_parts();
    let server = server.into_parts();
    debug_assert_eq!(client.compressed, server.compressed, "compression is mirrored");

    let to_server = Wire { direction: Direction::Serverbound, compressed: client.compressed };
    let to_client = Wire { direction: Direction::Clientbound, compressed: server.compressed };

    let to_server = pump_until_closed(to_server, client.buffer, client.reader, server.writer);
    let to_client = pump_until_closed(to_client, server.buffer, server.reader, client.writer);
    let (to_server, to_client) = zip(to_server, to_client).await;

    to_server.and(to_client)
}

/// When one direction ends, it closes the socket it was writing to. That
/// ends the other direction's read on the same socket, so no read is ever
/// cancelled mid-flight.
async fn pump_until_closed(wire: Wire, mut buffer: FrameBuffer, mut reader: ReadHalf, mut writer: WriteHalf) -> io::Result<()> {
    let pumped = pump(wire, State::Config, &mut buffer, &mut reader, &mut writer).await;

    // The peer may already be gone; the pump's own result is what matters.
    let _ = writer.shutdown().await;

    pumped
}
