mod error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let listener = TcpListener::bind("").await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;

        tokio::spawn(async move { stream.shutdown().await });
    }
}
