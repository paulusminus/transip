#!/bin/bash

export TRANSIP_API_PRIVATE_KEY=/etc/transip/paulusminus.pem
export TRANSIP_API_TOKEN_PATH=${HOME}/.transip-token.txt
export TRANSIP_API_USERNAME=paulusminus
export TRANSIP_API_LOG_DIR=${HOME}/transip
export TRANSIP_API_READONLY=true
export TRANSIP_API_WHITELISTED_ONLY=false
export TRANSIP_API_TOKEN_EXPIRATION="5 minutes"
export RUST_LOG=info

/usr/bin/transip-api-ping
