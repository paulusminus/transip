#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![doc(test(attr(deny(warnings))))]

pub use crate::client::Client;
pub use crate::environment::configuration_from_environment;
use authentication::TokenExpiration;
pub use error::Error;

/// See [api specification](https://api.transip.nl/rest/docs.html#header-api-specification)
pub mod api;
mod authentication;
mod base64;
mod client;
mod environment;
mod error;
mod fs;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Configuration {
    fn user_name(&self) -> &str;
    fn private_key_pem_file(&self) -> &str;
    fn token_path(&self) -> &str;
    fn whitelisted_only(&self) -> bool;
    fn read_only(&self) -> bool;
    fn token_expiration(&self) -> TokenExpiration;
}

trait HasName {
    fn name(&self) -> &str;
}

trait HasNames {
    fn names(&self) -> Vec<&str>;
}

impl<T: HasName> HasNames for Vec<T> {
    fn names(&self) -> Vec<&str> {
        self.iter().map(|t| t.name()).collect::<Vec<_>>()
    }
}
