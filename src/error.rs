use std::{io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid version")]
    InvalidVersion,
    #[error(transparent)]
    Io(#[from] io::Error),
}
