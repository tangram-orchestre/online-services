#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if ! command -v mkcert &> /dev/null
then
    echo "mkcert could not be found, please install it."
    exit 1
fi

mkcert \
    -cert-file ${SCRIPT_DIR}/traefik/certs/localhost.pem \
    -key-file ${SCRIPT_DIR}/traefik/certs/localhost-key.pem \
    localhost api.localhost portal.localhost traefik.localhost maildev.localhost grafana.localhost
