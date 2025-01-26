#!/usr/bin/env bash

cargo run --color always &

# Wait for server to be available and retrieve the OpenAPI specs for client
# generation.
bash /opt/scripts/wait-for-it.sh 127.0.0.1:3000 -t 0 -- \
    curl -s \
    -o /opt/openapi/spec.json \
    -X 'GET' 'http://127.0.0.1:3000/api/spec'  \
    -H 'accept: application/json; charset=utf-8'

wait