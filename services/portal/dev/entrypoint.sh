#!/usr/bin/bash

set -e

yarn install --locked

# Execute provided command
exec "$@"
