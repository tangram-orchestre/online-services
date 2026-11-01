#!/usr/bin/bash

set -e

# Only install when dependencies are out of date: `yarn install` always runs
# the `nuxt prepare` postinstall script, which wipes `.nuxt/`. That directory is
# shared through the bind mount with a possibly running `nuxt dev`, so running
# e.g. `just lint-portal` next to `just dev` would break the dev server.
if ! yarn check --integrity >/dev/null 2>&1; then
    yarn install --locked
fi

# Execute provided command
exec "$@"
