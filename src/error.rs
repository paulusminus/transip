use std::{time::SystemTimeError, net::AddrParseError, str::Utf8Error, num::ParseIntError};

use base64::DecodeError;
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

    #[error("IP 4 address missing")]
    Ipv4,

    #[error("Acme challenge not found")]
    AcmeChallege,

    #[error("Address: {0}")]
    Address(#[from] AddrParseError),

    #[error("Invalid Token")]
    Token,

    #[error("Base64: {0}")]
    Base64Decode(#[from] DecodeError),

    #[error("Utf8: {0}")]
    Utf8Decode(#[from] Utf8Error),

    #[error("Parse Int: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("Api test failed")]
    ApiTest,
}