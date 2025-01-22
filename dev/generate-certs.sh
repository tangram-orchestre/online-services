#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

mkcert \
    -cert-file ${SCRIPT_DIR}/traefik/certs/localhost.pem \
    -key-file ${SCRIPT_DIR}/traefik/certs/localhost-key.pem \
    localhost api.localhost portal.localhost traefik.localhost
