use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

const HOST: &str = "127.0.0.1:25565";
const PROXY: &str = "127.0.0.1:25566";
const BUFFER_SIZE: usize = 64_000;

struct Context<'a> {
    src: &'a TcpStream,
    dst: &'a TcpStream,
}

impl<'a> Context<'a> {
    fn new(src: &'a TcpStream, dst: &'a TcpStream) -> Self {
        Context { src, dst }
    }
}

async fn handle_data(ctx: &mut Context<'_>, bytes: &[u8]) -> std::io::Result<()> {
    // println!("{:?}", &bytes);
    // TODO: Handle packets

    ctx.dst.write_all(bytes).await?;

    Ok(())
}

async fn handle_stream(ctx: &mut Context<'_>) -> std::io::Result<()> {
    let mut buf = vec![0; BUFFER_SIZE];

    loop {
        match ctx.src.read(&mut buf).await? {
            0 => return Ok(()),
            n => handle_data(ctx, &buf[..n]).await?,
        };
    }
}

async fn handle_client(client_stream: TcpStream) -> std::io::Result<()> {
    let server_stream = TcpStream::connect(HOST).await?;

    let mut client_ctx = Context::new(&client_stream, &server_stream);
    let mut server_ctx = Context::new(&server_stream, &client_stream);

    let _ = futures::try_join!(handle_stream(&mut client_ctx), handle_stream(&mut server_ctx));
    let _ = client_stream.shutdown(std::net::Shutdown::Both);
    let _ = server_stream.shutdown(std::net::Shutdown::Both);

    Ok(())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(PROXY).await?;

    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(handle_client(stream));
    }

    Ok(())
}
