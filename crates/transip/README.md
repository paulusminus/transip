![main](https://github.com/paulusminus/transip-api/actions/workflows/rust.yml/badge.svg)

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
Transip control panel and stored in a file.

[`on crates.io`]: https://crates.io/crates/transip

```toml
[dependencies]
transip = "0.1.0"
```

# Example

```rust
use transip::{configuration_from_environment, Client, api::general::GeneralApi};
mut client = configuration_from_environment().and_then(Client::try_from).unwrap();
let pong = client.api_test().unwrap();
assert_eq!(pong.as_str(), "pong");
```

# Environment variables

The following environment variables should be set!.

## TRANSIP_API_USERNAME

This is the username used in authentication

Example

```bash
export TRANSIP_API_USERNAME=paulusminus
```

## TRANSIP_API_PRIVATE_KEY

This is the name of the file that holds the pem encoded private key used in authentication

Example

```bash
export TRANSIP_API_PRIVATE_KEY=/etc/transip/private.pem
```

## TRANSIP_API_READONLY

Can be 'true' or 'false'. If you wan't to prevent accidental modifications set this to 'true'.

Example

```bash
export TRANSIP_API_READONLY=false
```

## TRANSIP_API_WHITELISTED_ONLY

Can be 'true' or 'false'.

## TRANSIP_API_TOKEN_EXPIRATION

Authentication means receiving a token. The interval in which the received will expired can be controlled.

Example 1

```bash
export TRANSIP_API_TOKEN_EXPIRATION=5 minutes
```

Example 2

```bash
export TRANSIP_API_TOKEN_EXPIRATION=55 seconds
```

Example 3

```bash
export TRANSIP_API_TOKEN_EXPIRATION=1 hour
```
