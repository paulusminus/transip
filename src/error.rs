use std::time::SystemTimeError;

use ring::error::Unspecified;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Key: {0}")]
    Key(String),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Sign")]
    Sign(Unspecified),

    #[error("Systemtime: {0}")]
    SystemTime(#[from] SystemTimeError),
}