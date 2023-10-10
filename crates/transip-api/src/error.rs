use std::env::VarError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Key: {0}")]
    Key(&'static str),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Sign")]
    Sign(ring::error::Unspecified),

    #[error("Systemtime: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    #[error("Json: {0}")]
    Json(#[from] ureq::serde_json::Error),

    #[error("Ureq: {0}")]
    Ureq(#[from] Box<ureq::Error>),

    // #[allow(dead_code)]
    #[error("IP 4 address missing")]
    Ipv4,

    #[allow(dead_code)]
    #[error("Acme challenge not found")]
    AcmeChallege,

    #[error("Address: {0}")]
    Address(#[from] std::net::AddrParseError),

    #[error("Invalid Token")]
    Token,

    #[error("Base64: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Utf8: {0}")]
    Utf8Decode(#[from] std::str::Utf8Error),

    #[error("Parse Int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Environment variable not set: {0}")]
    EnvironmentVariable(String),

    #[error("Var: {0}")]
    Var(#[from] VarError),

    #[error("Api test failed")]
    ApiTest,

    #[error("No IP")]
    NoIp,
}
