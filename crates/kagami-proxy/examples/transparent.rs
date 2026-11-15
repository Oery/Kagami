//! Proxies one Minecraft client to a server, unchanged.
//!
//! cargo run --release -p kagami-proxy --example transparent -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::io;
use std::net::SocketAddr;

use kagami_proxy::{Account, Config, Upstream};

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: transparent <listen address> <server[:port]>");
        std::process::exit(2);
    };

    let listen: SocketAddr = listen.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid listen address"))?;
    let upstream = Upstream::parse(&server)?;
    let account = Account::from_env()?;

    kagami_proxy::run(Config { listen, upstream, account }, (), ())
}
