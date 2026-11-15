//! Proxies one Minecraft client to a server, unchanged.
//!
//! cargo run --release -p kagami-proxy --example transparent -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::env;
use std::io;
use std::net::SocketAddr;

use kagami_proxy::{Account, Config, Upstream};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: transparent <listen address> <server[:port]>");
        std::process::exit(2);
    };

    let listen: SocketAddr = listen.parse().map_err(|_| invalid_input("invalid listen address"))?;
    let upstream = Upstream::parse(&server)?;
    let account = account_from_env()?;

    kagami_proxy::run(Config { listen, upstream, account })
}

/// The account from the environment, when its token is set.
fn account_from_env() -> io::Result<Option<Account>> {
    let Ok(access_token) = env::var("KAGAMI_ACCESS_TOKEN") else {
        return Ok(None);
    };

    let uuid = env::var("KAGAMI_UUID").map_err(|_| invalid_input("KAGAMI_UUID is not set"))?;
    let uuid = uuid.parse().map_err(|_| invalid_input("KAGAMI_UUID is not a UUID"))?;
    let name = env::var("KAGAMI_NAME").map_err(|_| invalid_input("KAGAMI_NAME is not set"))?;

    Ok(Some(Account { access_token, uuid, name }))
}

fn invalid_input(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.to_owned())
}
