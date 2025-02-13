use std::{env::VarError, str::ParseBoolError};

use thiserror::Error;

/// All failable functions in this crate should use this Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("Key: {0}")]
    Key(&'static str),

    #[error("Rejected: {0}")]
    Rejected(String),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Sign")]
    Sign(ring::error::Unspecified),

    #[error("Systemtime: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    //    #[error("Json: {0}")]
    //    Json(#[from] ureq::serde_json::Error),
    #[error("Ureq: {0}")]
    Ureq(#[from] ureq::Error),

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

    #[error("Parse Expiration: {0}")]
    ParseExpiration(&'static str),

    #[error("Parse Dns entry: {0}")]
    ParseDnsEntry(&'static str),

    #[error("Parse Mailbox entry: {0}")]
    ParseMailboxEntry(String),

    #[error("Parse Mail forward entry: {0}")]
    ParseMailForwardEntry(String),

    #[error("Environment variable not set: {0}")]
    EnvironmentVariable(String),

    #[error("Var: {0}")]
    Var(#[from] VarError),

    #[error("Api test failed")]
    ApiTest,

    #[error("No IP")]
    NoIp,

    #[error("Parse: {0}")]
    ParseBoolean(#[from] ParseBoolError),

    #[error("Strum: {0}")]
    Strum(#[from] strum::ParseError),

    #[error("Serialization: {0}")]
    Serialization(Box<dyn std::error::Error>),
}

pub(crate) trait ResultExt<T, E>
where
    E: Into<Error>,
{
    fn err_into(self) -> Result<T, Error>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Into<Error>,
{
    fn err_into(self) -> Result<T, Error> {
        self.map_err(Into::into)
    }
}
