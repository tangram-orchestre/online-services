#!/usr/bin/env bash

OUT_DIR="./client"
HASH_FILE="$OUT_DIR/hash.md5"
SPEC_FILE="/opt/openapi/spec.json"
CONFIG_FILE="./openapi-ts.config.ts"

mkdir -p "$OUT_DIR"
touch "$HASH_FILE"
touch "$SPEC_FILE"

generate_if_modified () {
    HASH=`cat $HASH_FILE`
    NEW_HASH=$(md5sum "$SPEC_FILE" "$CONFIG_FILE")
    if [ "$HASH" != "$NEW_HASH" ]; then
        echo "Generating client"
        if yarn run openapi-ts -f $CONFIG_FILE; then
            echo "Applying ESLint"
            yarn run openapi-ts:lint:fix
            echo "$NEW_HASH" > "$HASH_FILE"
        fi
    fi
}

# Run generation on start to ensure we're up to date.
generate_if_modified


# Run client code generation when spec.json changes.
(inotifywait -m -e modify "$SPEC_FILE" |
    while read; do
        generate_if_modified
    done)
