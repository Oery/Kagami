use std::{fmt, sync::atomic::AtomicI32};

use async_std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    sync::Arc,
};

use crate::{
    proxy::Proxy,
    state::{AtomicState, State},
};

#[derive(Debug, PartialEq)]
pub enum Source {
    Client,
    Server,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct ProxyContext<'a> {
    pub src: BufferedStream<'a>,
    pub dst: BufferedStream<'a>,
    pub source: Source,
    pub compress_threshold: Arc<AtomicI32>,
    pub state: Arc<AtomicState>,
    pub proxy: Arc<Proxy>,
}

pub struct BufferedStream<'a> {
    pub reader: BufReader<&'a TcpStream>,
    pub writer: BufWriter<&'a TcpStream>,
}

impl<'a> BufferedStream<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream),
            writer: BufWriter::new(stream),
        }
    }
}

impl<'a> ProxyContext<'a> {
    pub fn new(client: &'a TcpStream, server: &'a TcpStream, proxy: Arc<Proxy>) -> (Self, Self) {
        let state = Arc::new(AtomicState::new(State::Handshake));
        let compress_threshold = Arc::new(AtomicI32::new(-1));

        let client_ctx = ProxyContext {
            src: BufferedStream::new(client),
            dst: BufferedStream::new(server),
            source: Source::Client,
            compress_threshold: compress_threshold.clone(),
            state: state.clone(),
            proxy: proxy.clone(),
        };

        let server_ctx = ProxyContext {
            src: BufferedStream::new(server),
            dst: BufferedStream::new(client),
            source: Source::Server,
            compress_threshold,
            state,
            proxy,
        };

        (client_ctx, server_ctx)
    }
}
