//! One client's session: the proxy answers the handshake, relays status and
//! login packet by packet, then pumps configuration and play.
//!
//! The client leg is never encrypted. Compression is mirrored: when the
//! server enables it, the proxy forwards the same threshold to the client,
//! so frames pass through both legs unchanged.

use std::io;
use std::rc::Rc;

use futures_lite::future::zip;
use glommio::net::TcpStream;
use mc_protocol::packets::client::LoginDisconnect;
use mc_protocol::packets::server::{LoginStart, ServerListPing};
use mc_protocol::packets::{Clientbound, Serverbound};
use mc_protocol::state::HandshakeIntent;
use mc_protocol::{Direction, IntoOwned, Payload, State};

use crate::Config;
use crate::connection::Connection;
use crate::frame::FrameBuffer;
use crate::handlers::Handlers;
use crate::invalid_data;
use crate::online::answer_encryption;
use crate::outlet::{Client, Outlet, Server};
use crate::pump::{Pump, Shared};
use crate::split::ReadHalf;
use crate::stream::Reader;

/// Serves one client connection until either side closes it.
pub async fn serve<S, H: Handlers<S>>(client: TcpStream, config: &Config, shared: &Rc<Shared<S, H>>) -> io::Result<()> {
    let mut client = Connection::new(client)?;

    let Some(Serverbound::ServerListPing(handshake)) = client.read_serverbound(State::Handshake).await? else {
        return Err(invalid_data("expected a handshake"));
    };
    let version = handshake.version;
    let intent = handshake.next_state;

    let upstream = &config.upstream;
    let server = TcpStream::connect((upstream.host.as_str(), upstream.port)).await?;
    let mut server = Connection::new(server)?;

    // The server sees the address it is reached at, not the proxy's.
    let handshake = ServerListPing { version, addr: upstream.host.as_str().into(), port: upstream.port, next_state: intent };
    server.send(|out| handshake.encode(out)).await?;

    match intent {
        HandshakeIntent::Status => relay_status(&mut client, &mut server).await,
        HandshakeIntent::Login | HandshakeIntent::Transfer => {
            if !login(&mut client, &mut server, config).await? {
                return Ok(());
            }

            pump_both(client, server, shared).await
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
async fn login(client: &mut Connection, server: &mut Connection, config: &Config) -> io::Result<bool> {
    let Some(Serverbound::LoginStart(login_start)) = client.read_serverbound(State::Login).await? else {
        return Err(invalid_data("expected login start"));
    };

    // With an account, the server sees that account whatever the client claims.
    let login_start = match &config.account {
        Some(account) => LoginStart { name: account.name.as_str().into(), uuid: account.uuid },
        None => login_start.into_owned(),
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
            // Only the server leg is encrypted, so the client never sees this.
            Clientbound::EncryptionRequest(request) => {
                let Some(account) = &config.account else {
                    return refuse(client, "This server is in online mode: start Kagami with your Minecraft account").await;
                };

                let request = request.into_owned();

                if let Err(error) = answer_encryption(server, &request, account).await {
                    return refuse(client, &error.to_string()).await;
                }
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
    let json = serde_json::json!({ "text": reason }).to_string();
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
async fn pump_both<S, H: Handlers<S>>(client: Connection, server: Connection, shared: &Rc<Shared<S, H>>) -> io::Result<()> {
    let client = client.into_parts();
    let server = server.into_parts();
    debug_assert_eq!(client.compressed, server.compressed, "compression is mirrored");

    let compressed = server.compressed;
    let client_outlet = Client(Rc::new(Outlet::new(client.writer, compressed)));
    let server_outlet = Server(Rc::new(Outlet::new(server.writer, compressed)));

    let mut to_server = Pump::new(Direction::Serverbound, compressed, shared.clone(), client_outlet.clone(), server_outlet.clone());
    let mut to_client = Pump::new(Direction::Clientbound, compressed, shared.clone(), client_outlet.clone(), server_outlet.clone());

    let pumps = async {
        let to_server = pump_until_closed(&mut to_server, client.buffer, client.reader);
        let to_client = pump_until_closed(&mut to_client, server.buffer, server.reader);
        let pumped = zip(to_server, to_client).await;

        client_outlet.0.close();
        server_outlet.0.close();

        pumped
    };
    // A flusher fails only when its socket is gone, which the pumps report.
    let flushers = zip(client_outlet.0.flush_when_woken(), server_outlet.0.flush_when_woken());
    let ((to_server, to_client), _) = zip(pumps, flushers).await;

    to_server.and(to_client)
}

/// When one direction ends, it closes the socket it was writing to. That
/// ends the other direction's read on the same socket, so no read is ever
/// cancelled mid-flight.
async fn pump_until_closed<S, H: Handlers<S>>(
    pump: &mut Pump<S, H>, mut buffer: FrameBuffer, mut reader: Reader<ReadHalf>,
) -> io::Result<()> {
    let pumped = pump.run(&mut buffer, &mut reader).await;

    // The peer may already be gone; the pump's own result is what matters.
    let _ = pump.target().shutdown().await;

    pumped
}
