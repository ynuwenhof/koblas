mod config;
mod error;

use crate::config::Config;
use crate::error::{AuthError, Error, SocksError};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use clap::{Parser, Subcommand};
use color_eyre::eyre::eyre;
use rand_core::OsRng;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Hash { password: String },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if let Some(Command::Hash { password }) = cli.command {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
        println!("{hash}");
        return Ok(());
    }

    let config_path = cli
        .config
        .or_else(|| dirs::config_dir().map(|p| p.join("koblas").join("koblas.toml")))
        .ok_or_else(|| eyre!("unable to locate config directory"))?;

    let config = if config_path.exists() {
        Config::from_path(config_path).await?
    } else {
        // TODO: We should probably report that no config file was found at the specified path!

        Config::default()
    };

    let listener = TcpListener::bind(config.server.addr).await?;

    let config = Arc::new(config);

    loop {
        let (mut stream, _addr) = listener.accept().await?;
        let config = config.clone();

        tokio::spawn(async move {
            if let Err(_err) = handle(&mut stream, config).await {
                todo!()
            }

            stream.shutdown().await
        });
    }
}

const AUTH_VERSION: u8 = 0x1;
const AUTH_METHOD: u8 = 0x2;
const NO_AUTH_METHOD: u8 = 0x0;
const NO_METHOD: u8 = 0xff;
const SOCKS_VERSION: u8 = 0x5;
const SUCCESS_REPLY: u8 = 0x0;

async fn handle(stream: &mut TcpStream, config: Arc<Config>) -> error::Result<()> {
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

    let method = *buf
        .iter()
        .find(|&&m| {
            m == NO_AUTH_METHOD && !config.server.auth
                || m == AUTH_METHOD && (config.server.auth || !config.users.is_empty())
        })
        .unwrap_or(&NO_METHOD);

    let buf = [SOCKS_VERSION, method];
    stream.write_all(&buf).await?;

    let res = match method {
        AUTH_METHOD => auth(stream, config).await,
        NO_METHOD => return Err(Error::MethodNotFound),
        _ => Ok(()),
    };

    let reply = res.is_err() as u8;
    let buf = [AUTH_VERSION, reply];
    stream.write_all(&buf).await?;
    res?;

    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await?;

    let ver = buf[0];
    if ver != SOCKS_VERSION {
        return Err(Error::InvalidVersion {
            expected: SOCKS_VERSION,
            found: ver,
        });
    }

    let mut reply = SUCCESS_REPLY;
    let res = socks(stream, buf).await;
    if let Err(ref err) = res {
        reply = match err {
            SocksError::InvalidAddr { .. } => 0x8,
            SocksError::InvalidCommand { .. } => 0x7,
            _ => 0x1,
        }
    }

    let buf = [SOCKS_VERSION, reply, 0, IPV4_TYPE, 0, 0, 0, 0, 0, 0];
    stream.write_all(&buf).await?;

    let mut peer = res?;
    io::copy_bidirectional(stream, &mut peer).await?;

    Ok(())
}

async fn auth(stream: &mut TcpStream, config: Arc<Config>) -> Result<(), AuthError> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;

    let ver = buf[0];
    if ver != AUTH_VERSION {
        return Err(AuthError::InvalidVersion {
            expected: AUTH_VERSION,
            found: ver,
        });
    }

    let len = buf[1] as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    let username = String::from_utf8(buf)?;

    let len = stream.read_u8().await? as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    let password = String::from_utf8(buf)?;

    let pass = config
        .users
        .get(&username)
        .ok_or(AuthError::UsernameNotFound(username))?;

    let hash = PasswordHash::new(pass)?;

    Ok(Argon2::default().verify_password(password.as_bytes(), &hash)?)
}

const IPV4_TYPE: u8 = 0x1;
const IPV6_TYPE: u8 = 0x4;
const DOMAIN_TYPE: u8 = 0x3;
const CONNECT_COMMAND: u8 = 0x1;

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
