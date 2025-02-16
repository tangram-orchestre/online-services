#!/usr/bin/env bash

OUT_DIR="/opt/openapi/out"
HASH_FILE="$OUT_DIR/spec.json.md5"
SPEC_FILE="/opt/openapi/spec.json"

mkdir -p "$OUT_DIR"
touch "$HASH_FILE"
touch "$SPEC_FILE"

export JAVA_OPTS="-Dlog.level=warn"

generate_if_modified () {
   HASH=`cat $HASH_FILE`
    NEW_HASH=$(md5sum "$SPEC_FILE")
    if [ "$HASH" != "$NEW_HASH" ]; then
        if /usr/local/bin/docker-entrypoint.sh generate \
            -i "$SPEC_FILE" \
            -g rust \
            -o out \
            --minimal-update; then
            echo "$NEW_HASH" > "$HASH_FILE"
        fi
    fi
}

# Run generation on start to ensure we're up to date.
generate_if_modified

# Run client code generation when spec.json changes.
inotifywait -m -e modify "$SPEC_FILE" |
    while read; do
        generate_if_modified
    done
