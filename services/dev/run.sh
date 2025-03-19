#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd $SCRIPT_DIR/..

# Generate .env file
cat <<EOF > .env
UID=$(id -u)
GID=$(id -g)
EOF

# Generate certificates if they don't exist
CERT_FILE="./dev/traefik/certs/localhost.pem"
if [ ! -f "$CERT_FILE" ]; then
    ./dev/generate-certs.sh
fi

# Launch Docker Compose environment
docker compose up
