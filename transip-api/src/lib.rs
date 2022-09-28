/*!
This crate provides a library for calling Rest Api functions on the Transip endpoint.
More information on the rest api can be obtained from [Transip API](https://api.transip.nl).
Only part of the api is implemented. The main reason for writing this library is the ability
to CRUD dns records for a particular domain. This functionality is used to respond to DNS01 challenges
from the [Let's Encrypt](https://letsencrypt.org) servers.
This type of challenge is needed to get wildcard certificates.

# Usage

This crate is [on crates.io](https://crates.io/crates/transip-api) and can
be used by adding `transip-api` to your depencies in your projects `Cargo.toml`.
Before running a program using this library a private key should be download from the
Transip control panel and stored in the user config directory `transip`.

```toml
[dependencies]
transip-api = "0.3"
```

# Example

```rust
use std::error::Error;
use std::fs::File;
use transip_api::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let (file, mut client): (Result<File>, ApiClient) = default_account()?.into();

    assert!(client.api_test()?.as_str(), "pong");

    Ok(())
}
```

*/

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
