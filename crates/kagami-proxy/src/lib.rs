//! Kagami's proxy engine: sockets, framing and the per-session pumps.
//!
//! The proxy sits between one Minecraft client and one server on the same
//! machine. It answers the handshake itself, relays status and login packet
//! by packet, and then forwards configuration and play frames unchanged.
//! Online-mode servers are joined with the caller's [`Account`].
//!
//! Handlers run inline on the frames they want. Anything that takes time
//! is a task spawned in the client's [`Session`], cancelled when it ends.
//! Linux only: it runs on Glommio's io_uring executor, on one thread.

mod cipher;
mod compression;
mod connection;
mod frame;
mod handlers;
mod online;
mod outlet;
mod pump;
mod serve;
mod session;
mod split;
mod stream;
mod subscription;
mod tasks;
mod transition;

use std::cell::RefCell;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::rc::Rc;

use glommio::LocalExecutorBuilder;
use glommio::net::TcpListener;

use crate::serve::serve;
use crate::session::Shared;

pub use handlers::{Action, Context, Handlers, Interest};
pub use online::Account;
pub use outlet::{Client, Server};
pub use session::Session;
pub use subscription::Subscription;
pub use tasks::TaskHandle;

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

/// Proxies every client that connects to `config.listen`, forever, running
/// `handlers` on the packets they want. Starts its own single-threaded
/// executor and blocks the calling thread.
pub fn run<S: 'static, H: Handlers<S> + 'static>(config: Config, state: S, handlers: H) -> io::Result<()> {
    let shared = Rc::new(Shared::new(state));
    let handlers = Rc::new(RefCell::new(handlers));
    let executor = LocalExecutorBuilder::default().name("kagami").make()?;

    executor.run(accept_forever(Rc::new(config), shared, handlers))
}

async fn accept_forever<S: 'static, H: Handlers<S> + 'static>(
    config: Rc<Config>, shared: Rc<Shared<S>>, handlers: Rc<RefCell<H>>,
) -> io::Result<()> {
    let listener = TcpListener::bind(config.listen)?;

    loop {
        let client = listener.accept().await?;
        let config = config.clone();
        let shared = shared.clone();
        let handlers = handlers.clone();

        // Server list pings open their own short connections next to a game session.
        glommio::spawn_local(async move {
            if let Err(error) = serve(client, &config, &shared, &handlers).await {
                eprintln!("kagami: session ended: {error}");
            }
        })
        .detach();
    }
}

/// Builds handlers' context outside a proxy, for tests of handler chains.
#[doc(hidden)]
pub mod testing {
    use std::rc::Rc;

    use glommio::net::{TcpListener, TcpStream};

    use crate::handlers::Context;
    use crate::outlet::{Client, Outlet, Server};
    use crate::session::{Session, Shared};
    use crate::split::split;
    use crate::stream::Writer;

    /// A session over `state` whose sockets nobody reads. Call it inside a
    /// Glommio executor.
    pub async fn session<S>(state: S) -> Session<S> {
        let client = Client(outlet().await);
        let server = Server(outlet().await);

        Session::new(client, server, Rc::new(Shared::new(state)))
    }

    /// Runs `f` with the context handlers get in `session`.
    pub fn with_context<S, R>(session: &Session<S>, f: impl FnOnce(&mut Context<'_, S>) -> R) -> R {
        session.with_context(f)
    }

    async fn outlet() -> Rc<Outlet> {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback port is free");
        let address = listener.local_addr().expect("a bound listener has an address");
        let stream = TcpStream::connect(address).await.expect("the listener accepts");
        let (_, writer) = split(stream);

        Rc::new(Outlet::new(Writer::new(writer), false))
    }
}

pub(crate) fn invalid_data(error: impl Into<Box<dyn Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

#[cfg(test)]
#[path = "../tests/tests_upstream.rs"]
mod tests;
