#!/bin/bash

export TRANSIP_API_PRIVATE_KEY=/etc/transip/paulusminus.pem
export TRANSIP_API_USERNAME=paulusminus
export TRANSIP_API_LOG_DIR=${HOME}/transip
export RUST_LOG=info
export CERTBOT_DOMAIN=paulmin.nl
export CERTBOT_ALL_DOMAINS=paulmin.nl,paulmin.nl

/usr/bin/transip-dns-challenge
