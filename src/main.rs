mod error;

use crate::error::{Error, SocksError};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let options = SqliteConnectOptions::from_str("")?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    let listener = TcpListener::bind("").await?;

    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let pool = pool.clone();

        tokio::spawn(async move {
            if let Err(_err) = handle(&mut stream, pool).await {
                todo!()
            }

            stream.shutdown().await
        });
    }
}

const IPV4_TYPE: u8 = 0x1;
const IPV6_TYPE: u8 = 0x4;
const DOMAIN_TYPE: u8 = 0x3;
const SOCKS_VERSION: u8 = 0x5;
const SUCCESS_REPLY: u8 = 0x0;
const CONNECT_COMMAND: u8 = 0x1;

async fn handle(stream: &mut TcpStream, _pool: SqlitePool) -> error::Result<()> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;

    let ver = buf[0];
    if ver != SOCKS_VERSION {
        return Err(Error::InvalidVersion {
            expected: SOCKS_VERSION,
            found: ver,
        });
    }

    let len = buf[1] as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;

    // TODO: Handle method specific subnegotiation

    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;

    let ver = buf[0];
    if ver != SOCKS_VERSION {
        return Err(Error::InvalidVersion {
            expected: SOCKS_VERSION,
            found: ver,
        });
    }

    let mut peer = match socks(stream, buf).await {
        Ok(peer) => peer,
        Err(err) => {
            let reply = match err {
                SocksError::InvalidAddr { .. } => 0x8,
                SocksError::InvalidCommand { .. } => 0x7,
                _ => 0x1,
            };

            let buf = [SOCKS_VERSION, reply, 0, IPV4_TYPE, 0, 0, 0, 0, 0, 0];
            stream.write_all(&buf).await?;

            return Err(err.into());
        }
    };

    let buf = [SOCKS_VERSION, SUCCESS_REPLY, 0, IPV4_TYPE, 0, 0, 0, 0, 0, 0];
    stream.write_all(&buf).await?;

    io::copy_bidirectional(stream, &mut peer).await?;

    Ok(())
}

async fn socks(stream: &mut TcpStream, buf: [u8; 4]) -> Result<TcpStream, SocksError> {
    let cmd = buf[1];
    if cmd != CONNECT_COMMAND {
        return Err(SocksError::InvalidCommand {
            expected: CONNECT_COMMAND,
            found: cmd,
        });
    }

    let addr = buf[3];
    let dest = match addr {
        IPV4_TYPE => {
            let mut octets = [0u8; 4];
            stream.read_exact(&mut octets).await?;

            let port = stream.read_u16().await?;
            SocketAddr::new(Ipv4Addr::from(octets).into(), port)
        }
        DOMAIN_TYPE => {
            let len = stream.read_u8().await? as usize;
            let mut buf = vec![0u8; len];
            stream.read_exact(&mut buf).await?;

            let domain = String::from_utf8(buf)?;
            let port = stream.read_u16().await?;

            SocketAddr::from_str(&format!("{domain}:{port}"))?
        }
        IPV6_TYPE => {
            let mut octets = [0u8; 16];
            stream.read_exact(&mut octets).await?;

            let port = stream.read_u16().await?;
            SocketAddr::new(Ipv6Addr::from(octets).into(), port)
        }
        _ => {
            return Err(SocksError::InvalidAddr {
                expected: vec![IPV4_TYPE, DOMAIN_TYPE, IPV6_TYPE],
                found: addr,
            })
        }
    };

    Ok(TcpStream::connect(dest).await?)
}
