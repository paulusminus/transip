#!/bin/bash

export TRANSIP_API_PRIVATE_KEY=/etc/transip/home.pem
export TRANSIP_API_USERNAME=paulusminus
export TRANSIP_API_LOG_DIR=${HOME}/transip
export TRANSIP_API_TOKEN_PATH=${HOME}/.transip-token.txt
export TRANSIP_API_READONLY=false
export TRANSIP_API_WHITELISTED_ONLY=true
export TRANSIP_API_TOKEN_EXPIRATION="5 minutes"
export RUST_LOG=info
export CERTBOT_DOMAIN=paulmin.nl
export CERTBOT_ALL_DOMAINS=paulmin.nl,paulmin.nl
export CERTBOT_VALIDATION=uIDKFLEKFO-srM

/usr/bin/transip-acme-challenge
