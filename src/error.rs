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
    #[error("unsupported command")]
    UnsupportedCommand = 0x7,
}
