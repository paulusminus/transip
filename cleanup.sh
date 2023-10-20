#!/bin/bash

export RUST_LOG=info
export CERTBOT_DOMAIN=paulmin.nl
export CERTBOT_ALL_DOMAINS=paulmin.nl,paulmin.nl
export CERTBOT_AUTH_OUTPUT=Ok

/usr/bin/transip-acme-challenge --cleanup
