use std::{io, net::AddrParseError};

use thiserror::Error;
use trust_dns_resolver::error::ResolveError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Ipv4")]
    Ipv4,

    #[error("ACME challenge")]
    AcmeChallege,

    #[error("IO: {0}")]
    IO(#[from] io::Error),

    #[error("Resolve: {0}")]
    Resolve(#[from] ResolveError),

    #[error("")]
    Parse(#[from] AddrParseError),

    #[error("Multiple acme challenges")]
    MultipleAcme,
}
