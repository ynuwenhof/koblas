mod error;

use crate::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let listener = TcpListener::bind("").await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            handle(&mut stream).await;
            stream.shutdown().await
        });
    }
}

const SOCKS_VERSION: u8 = 0x5;

async fn handle(stream: &mut TcpStream) -> error::Result<()> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;

    let version = buf[0];
    if version != SOCKS_VERSION {
        return Err(Error::InvalidVersion);
    }

    let len = buf[1] as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;

    Ok(())
}
