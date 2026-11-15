//! Kagami's proxy engine: sockets, framing and the per-session pumps.
//!
//! The proxy sits between one Minecraft client and one server on the same
//! machine. It answers the handshake itself, relays status and login packet
//! by packet, and then forwards configuration and play frames unchanged.
//! Online-mode servers are joined with the caller's [`Account`].
//! Linux only: it runs on Glommio's io_uring executor, on one thread.

mod cipher;
mod compression;
mod connection;
mod frame;
mod online;
mod pump;
mod session;
mod split;
mod stream;
mod transition;

use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;

use glommio::LocalExecutorBuilder;
use glommio::net::TcpListener;

pub use online::Account;
pub use session::serve;

const DEFAULT_PORT: u16 = 25565;

/// The server the proxy connects each client to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Upstream {
    /// Also sent in the handshake, since some servers route on it.
    pub host: String,
    pub port: u16,
}

impl Upstream {
    /// Parses `host`, `host:port` or `[ipv6]:port`; the port defaults to 25565.
    pub fn parse(server: &str) -> io::Result<Self> {
        let (host, port) = match server.rsplit_once(':') {
            Some((host, port)) if !host.contains(':') || host.starts_with('[') => (host, Some(port)),
            _ => (server, None),
        };
        let host = host.trim_start_matches('[').trim_end_matches(']');

        let port = match port {
            Some(port) => port.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid server port"))?,
            None => DEFAULT_PORT,
        };

        Ok(Self { host: host.to_owned(), port })
    }
}

/// What the proxy listens on and connects to.
pub struct Config {
    pub listen: SocketAddr,
    pub upstream: Upstream,
    /// Needed for online-mode servers; offline servers accept any name.
    pub account: Option<Account>,
}

/// Proxies every client that connects to `config.listen`, forever. Starts
/// its own single-threaded executor and blocks the calling thread.
pub fn run(config: Config) -> io::Result<()> {
    let executor = LocalExecutorBuilder::default().name("kagami").make()?;

    executor.run(accept_forever(Rc::new(config)))
}

async fn accept_forever(config: Rc<Config>) -> io::Result<()> {
    let listener = TcpListener::bind(config.listen)?;

    loop {
        let client = listener.accept().await?;
        let config = config.clone();

        // Server list pings open their own short connections next to a game session.
        glommio::spawn_local(async move {
            if let Err(error) = serve(client, &config).await {
                eprintln!("kagami: session ended: {error}");
            }
        })
        .detach();
    }
}

pub(crate) fn invalid_data(error: impl Into<Box<dyn Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

#[cfg(test)]
#[path = "../tests/tests_upstream.rs"]
mod tests;
