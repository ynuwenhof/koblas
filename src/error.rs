use argon2::password_hash;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Auth(AuthError),
    InvalidVersion { expected: u8, found: u8 },
    Io(io::Error),
    MethodNotFound,
    Socks(SocksError),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(err) => err.fmt(f),
            Self::InvalidVersion { expected, found } => {
                write!(
                    f,
                    "invalid protocol version (expected {expected}, found {found})"
                )
            }
            Self::Io(err) => err.fmt(f),
            Self::MethodNotFound => write!(f, "no applicable method not found"),
            Self::Socks(err) => err.fmt(f),
        }
    }
}

impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<SocksError> for Error {
    fn from(err: SocksError) -> Self {
        Self::Socks(err)
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidVersion { expected: u8, found: u8 },
    Io(io::Error),
    Password(password_hash::Error),
    UsernameNotFound(String),
    Utf8(FromUtf8Error),
}

impl error::Error for AuthError {}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVersion { expected, found } => {
                write!(
                    f,
                    "invalid authentication version (expected {expected}, found {found})"
                )
            }
            Self::Io(err) => err.fmt(f),
            Self::Password(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
            Self::UsernameNotFound(username) => write!(f, "username `{username}` not found",),
        }
    }
}

impl From<io::Error> for AuthError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<password_hash::Error> for AuthError {
    fn from(err: password_hash::Error) -> Self {
        Self::Password(err)
    }
}

impl From<FromUtf8Error> for AuthError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

#[derive(Debug)]
pub enum SocksError {
    InvalidAddr { expected: Vec<u8>, found: u8 },
    InvalidCommand { expected: u8, found: u8 },
    Io(io::Error),
    Utf8(FromUtf8Error),
}

impl error::Error for SocksError {}

impl Display for SocksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddr { expected, found } => {
                let expected = expected.iter().join(", ");

                write!(f, "invalid addr type (expected {expected}, found {found})")
            }
            Self::InvalidCommand { expected, found } => {
                write!(f, "invalid command (expected {expected}, found {found})")
            }
            Self::Io(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for SocksError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<FromUtf8Error> for SocksError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}
