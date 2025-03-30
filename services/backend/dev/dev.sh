#!/usr/bin/env bash

cargo run --color always &

# Wait for server to be available and retrieve the OpenAPI specs for client
# generation.
bash /opt/scripts/wait-for-it.sh 127.0.0.1:3000 -t 0 -- bash ./dev/fetch-api-specs.sh

wait
