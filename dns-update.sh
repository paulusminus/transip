#!/bin/bash

export CERTBOT_DOMAIN=paulmin.nl
export CERTBOT_ALL_DOMAINS=paulmin.nl,paulmin.nl
export CERTBOT_VALIDATION=uIDKFLEKFO-srM

/usr/bin/transip-acme-challenge
