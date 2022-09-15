use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Ipv4")]
    Ipv4,

    #[error("ACME challenge")]
    AcmeChallege,

    #[error("IO: {0}")]
    IO(#[from] io::Error),
}