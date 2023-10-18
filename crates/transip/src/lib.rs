/*!
This crate provides a library for calling Rest Api functions on the Transip endpoint.
More information on the rest api can be obtained from [`Transip API`].
Only part of the api is implemented. The main reason for writing this library is the ability
to [`CRUD`] dns records for a particular domain. This functionality can be used to respond to DNS01 challenges
from the [`Let's Encrypt`] servers.
This type of challenge is needed to get wildcard certificates.

[`Transip API`]: https://api.transip.nl
[`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
[`Let's Encrypt`]: https://letsencrypt.org

# Usage

This crate is [`on crates.io`] and can
be used by adding `transip` to your depencies in your projects `Cargo.toml`.
Before running a program using this library a private key should be download from the
Transip control panel and stored in the user config directory `transip`.

[`on crates.io`]: https://crates.io/crates/transip-api

```toml
[dependencies]
transip-api = "0.3"
```

# Example

```
    # use transip::{configuration_from_environment, Client, Error};
    mut client = configuration_from_environment().and_then(Client::try_from)?;
    let pong = client.api_test()?;
    assert_eq!(pong.as_str(), "pong");
    # Ok::<(), Error>(())
```

*/

pub use crate::client::Client;
pub use crate::environment::configuration_from_environment;
use authentication::TokenExpiration;
pub use error::Error;

pub mod api;
mod authentication;
mod base64;
mod client;
mod environment;
mod error;
mod fs;
pub mod vps;

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
