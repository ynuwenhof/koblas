use std::io::ErrorKind;
use std::{io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid version")]
    InvalidVersion,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Request(#[from] RequestError),
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("general error")]
    General = 0x1,
    #[error("network unreachable")]
    NetworkUnreachable = 0x3,
    #[error("host unreachable")]
    HostUnreachable,
    #[error("connection refused")]
    ConnectionRefused,
    #[error("unsupported command")]
    UnsupportedCommand = 0x7,
    #[error("unsupported address")]
    UnsupportedAddress,
}

impl From<io::Error> for RequestError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NetworkUnreachable => Self::NetworkUnreachable,
            ErrorKind::HostUnreachable => Self::HostUnreachable,
            ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            _ => Self::General,
        }
    }
}
