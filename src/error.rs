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

    #[error("Json: {0}")]
    Json(#[from] ureq::serde_json::Error),

    #[error("Ureq: {0}")]
    Ureq(#[from] ureq::Error),
}