#!/bin/bash

export TRANSIP_API_PRIVATE_KEY=/etc/transip/paulusminus.pem
export TRANSIP_API_TOKEN_PATH=${HOME}/.transip-token.txt
export TRANSIP_API_USERNAME=paulusminus
export TRANSIP_API_LOG_DIR=${HOME}/transip
export RUST_LOG=trace

/usr/bin/transip-api-ping
