pub use error::Error;

mod account;
mod authentication;
mod api_client;
mod domain;
mod error;
pub mod general;
pub mod prelude;
mod vps;

pub type Result<T> = std::result::Result<T, Error>;
