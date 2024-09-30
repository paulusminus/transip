![main](https://github.com/paulusminus/transip-api/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![docs.rs](https://img.shields.io/docsrs/transip)

# transip

This library crate can be used for calling functions on the [`Transip Api`] endpoint.
Only part of the api is implemented.
The main reason for writing this library is the ability
to [`CRUD`] dns records for a particular domain.
This functionality can be used to respond to
DNS01 challenges from the [`Let's Encrypt`] servers.
This type of challenge is needed to get wildcard certificates.

## Example

```no_run
use transip::{configuration_from_environment, Client, api::general::GeneralApi};

let mut client = configuration_from_environment()
    .and_then(Client::try_from)
    .expect("No cliënt");

let pong = client.api_test().expect("api test failed");
assert_eq!(pong.as_str(), "pong");
```

## Environment variables

The following environment variables should be set!.

### TRANSIP_API_USERNAME

This is the username used in authentication

Example

```bash
export TRANSIP_API_USERNAME=paulusminus
```

### TRANSIP_API_PRIVATE_KEY

This is the name of the file that holds the pem encoded private key used in authentication

Example

```bash
export TRANSIP_API_PRIVATE_KEY=/etc/transip/private.pem
```

### TRANSIP_API_READONLY

Can be 'true' or 'false'.
If you want to prevent accidental modifications set this to 'true'.

Example

```bash
export TRANSIP_API_READONLY=false
```

### TRANSIP_API_IPV6ONLY

Can be 'true' or 'false'. Use true if on a ipv6 only (virtual) machine.

Example

```bash
export TRANSIP_API_IPV6ONLY=false
```

### TRANSIP_API_WHITELISTED_ONLY

Can be 'true' or 'false'.
If you want to access the api on a whitelisted ipaddress set this to 'true'.

### TRANSIP_API_TOKEN_EXPIRATION

Authentication means receiving a token.
The interval in which the received will expired can be controlled.

#### Example 1

```bash
export TRANSIP_API_TOKEN_EXPIRATION=5 minutes
```

#### Example 2

```bash
export TRANSIP_API_TOKEN_EXPIRATION=55 seconds
```

#### Example 3

```bash
export TRANSIP_API_TOKEN_EXPIRATION=1 hour
```

### TRANSIP_API_LOG_DIR

Directory where the rotating log files are written.

#### Example

```bash
export TRANSIP_API_LOG_DIR=/var/log/transip
```

### TRANSIP_API_TOKEN_PATH

Name of the file where the authentication token
received from the endpoint will be written to.

#### Example

```bash
export TRANSIP_API_TOKEN_PATH=/root/.token.txt
```

[`Transip Api`]: https://api.transip.nl
[`CRUD`]: https://en.wikipedia.org/wiki/Create,_read,_update_and_delete
[`Let's Encrypt`]: https://letsencrypt.org
