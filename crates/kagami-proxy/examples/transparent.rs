//! Proxies one Minecraft client to an offline-mode server, unchanged.
//!
//! cargo run --release -p kagami-proxy --example transparent -- 127.0.0.1:25566 127.0.0.1:25565

use std::io;
use std::net::SocketAddr;

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: transparent <listen address> <server[:port]>");
        std::process::exit(2);
    };
    let listen: SocketAddr = listen.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid listen address"))?;
    kagami_proxy::run(listen, &server)
}
