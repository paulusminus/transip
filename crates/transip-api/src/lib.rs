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
be used by adding `transip-api` to your depencies in your projects `Cargo.toml`.
Before running a program using this library a private key should be download from the
Transip control panel and stored in the user config directory `transip`.

[`on crates.io`]: https://crates.io/crates/transip-api

```toml
[dependencies]
transip-api = "0.3"
```

# Example

```
    let (file, mut client): (Result<File>, ApiClient) = default_account()?.into();
    assert_eq!(client.api_test()?.as_str(), "pong");
```

*/

pub use crate::account::TransipApiAccount;
pub use crate::api_client::ApiClient;
pub use crate::configuration::configuration_from_environment;
pub use crate::domain::{DnsEntry, TransipApiDomain};
pub use crate::general::TransipApiGeneral;
pub use crate::vps::TransipApiVps;
pub use error::Error;

mod account;
mod api_client;
mod authentication;
mod domain;
mod configuration;
mod error;
mod general;
mod vps;

pub type Result<T> = std::result::Result<T, Error>;
