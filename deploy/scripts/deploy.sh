#!/usr/bin/env bash
#
# Switches an environment to a release, with a database snapshot and an
# automatic rollback if the release does not come up healthy.
#
# Usage:
#   deploy.sh <production|staging> deploy <sha> [--reset-db]
#   deploy.sh <production|staging> rollback <sha> [--restore-db]
#   deploy.sh staging stop
#
# Releases live in <root>/releases/<sha> (rsynced by CI), <root>/current points
# to the running one, secrets live in <root>/shared (mirroring the release tree)
# and database snapshots in <root>/db-dumps/<service>/<tag>.dump.
#
# A switch goes through these steps:
#   1. pull/build the images of the target release (no downtime yet)
#   2. stop the apps: Traefik serves the maintenance page meanwhile
#   3. snapshot the databases
#   4. start the target release and wait for every container to be healthy
#   5. on failure: restore the snapshots and start the previous release again
#
# --reset-db (staging only) recreates empty databases before starting.
# --restore-db restores the snapshots taken right before the current release
# was deployed, i.e. the data matching the previous release.

set -euo pipefail

KEEP_RELEASES=5
KEEP_DUMPS=10
WAIT_TIMEOUT=300
# Services never stopped during a switch, so the maintenance page stays up
ALWAYS_RUNNING="traefik maintenance"

usage() {
    sed -n '6,9p' "$0" | sed 's/^# //' >&2
    exit 64
}

log() {
    echo "==> $*"
}

die() {
    echo "ERROR: $*" >&2
    exit 1
}

ENV_NAME=${1:-}
ACTION=${2:-}
case "$ENV_NAME" in
    production) ROOT=/opt/services ;;
    staging) ROOT=/opt/services-staging ;;
    *) usage ;;
esac
# Overridable to test the script outside the server
ROOT=${DEPLOY_ROOT:-$ROOT}
RELEASES=$ROOT/releases
SHARED=$ROOT/shared
DUMPS=$ROOT/db-dumps

if [ -f ~/.docker_env ]; then
    # shellcheck source=/dev/null
    source ~/.docker_env
fi

# Runs docker compose for the release in $1, with its environment configuration.
compose() {
    local dir=$1
    shift
    (
        cd "$dir"
        set -a
        # shellcheck source=/dev/null
        source "./$ENV_NAME.env"
        # shellcheck source=/dev/null
        source ./release.env
        set +a
        docker compose -f "$ENV_NAME.yaml" "$@"
    )
}

# Reads a variable from the environment configuration of the release in $1.
config() {
    (
        set -a
        # shellcheck source=/dev/null
        source "$1/$ENV_NAME.env"
        eval "echo \"\${$2:-}\""
    )
}

current_release() {
    if [ -L "$ROOT/current" ]; then
        readlink -f "$ROOT/current"
    fi
}

# Links the secrets and pins the image tag of the release in $1.
prepare_release() {
    local dir=$1 sha=$2
    [ -d "$dir" ] || die "release $dir does not exist"

    if [ -d "$SHARED" ]; then
        (cd "$SHARED" && find . -name .env -type f) | while read -r file; do
            mkdir -p "$(dirname "$dir/$file")"
            ln -sfn "$SHARED/$file" "$dir/$file"
        done
    fi
    echo "IMAGE_TAG=sha-$sha" >"$dir/release.env"
    check_secrets "$dir"
}

env_keys() {
    cut -d= -f1 "$1" | grep -v '^[[:space:]]*$' | sort
}

# Fails before any downtime if a secret file of the environment is missing or
# has other keys than its sample.env, instead of starting services with empty
# secrets. Only the folders included by the environment's compose file count.
check_secrets() {
    local dir=$1 folder secrets errors=0 folders=()
    mapfile -t folders < <(sed -n 's|^ *- *\./\(.*\)/[^/]*\.yaml *$|\1|p' "$dir/$ENV_NAME.yaml")
    # The root secrets only hold the UID of the rootless Docker socket
    # shellcheck disable=SC2016
    if grep -q '${UID}' "$dir/$ENV_NAME.yaml"; then
        folders+=(.)
    fi
    for folder in "${folders[@]}"; do
        [ -f "$dir/$folder/sample.env" ] || continue
        secrets=${folder#.}/.env
        secrets=${secrets#/}
        if [ ! -f "$dir/$secrets" ]; then
            echo "missing secrets: $SHARED/$secrets" >&2
            errors=1
        elif [ "$(env_keys "$dir/$secrets")" != "$(env_keys "$dir/$folder/sample.env")" ]; then
            echo "secrets $SHARED/$secrets do not have the keys of $folder/sample.env" >&2
            errors=1
        fi
    done
    [ $errors = 0 ] || die "fix the secrets in $SHARED, nothing was changed"
}

ensure_network() {
    # Shared by the production and staging stacks
    docker network inspect services >/dev/null 2>&1 || docker network create services >/dev/null
}

# Starts the databases of the release in $1 and dumps them with tag $2.
dump_databases() {
    local dir=$1 tag=$2 svc file
    # shellcheck disable=SC2046
    compose "$dir" up -d --no-deps --wait --wait-timeout "$WAIT_TIMEOUT" $(config "$dir" DEPLOY_DATABASES)
    for svc in $(config "$dir" DEPLOY_DATABASES); do
        mkdir -p "$DUMPS/$svc"
        file=$DUMPS/$svc/$tag.dump
        log "Snapshotting $svc to $file"
        # Dumped with the database container's own tools so versions always match
        # shellcheck disable=SC2016
        compose "$dir" exec -T "$svc" sh -c 'pg_dump -U "$POSTGRES_USER" -d "$POSTGRES_DB" -Fc' >"$file.tmp"
        mv "$file.tmp" "$file"
    done
}

# Recreates the databases of the release in $1, then restores the dumps with
# tag $2 into them (or leaves them empty when $2 is empty).
recreate_databases() {
    local dir=$1 tag=$2 svc file
    for svc in $(config "$dir" DEPLOY_DATABASES); do
        file=$DUMPS/$svc/$tag.dump
        if [ -n "$tag" ] && [ ! -s "$file" ]; then
            die "no snapshot $file to restore"
        fi
        log "Recreating database of $svc${tag:+ from $file}"
        # shellcheck disable=SC2016
        compose "$dir" exec -T "$svc" sh -c '
            psql -q -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d postgres \
                -c "DROP DATABASE IF EXISTS \"$POSTGRES_DB\" WITH (FORCE)" \
                -c "CREATE DATABASE \"$POSTGRES_DB\" OWNER \"$POSTGRES_USER\""'
        if [ -n "$tag" ]; then
            # shellcheck disable=SC2016
            compose "$dir" exec -T "$svc" sh -c \
                'pg_restore -U "$POSTGRES_USER" -d "$POSTGRES_DB" --exit-on-error --single-transaction' <"$file"
        fi
    done
}

# Stops every service of the release in $1 except the databases and the ones
# serving the maintenance page, so nothing writes to the databases.
stop_database_clients() {
    local dir=$1 svc keep services=()
    keep=" $ALWAYS_RUNNING $(config "$dir" DEPLOY_DATABASES) "
    for svc in $(compose "$dir" config --services); do
        [[ $keep == *" $svc "* ]] || services+=("$svc")
    done
    # Without arguments `stop` would stop everything
    if [ ${#services[@]} -gt 0 ]; then
        compose "$dir" stop "${services[@]}"
    fi
}

# Starts the release in $1 and waits for all its containers to be healthy.
start_release() {
    compose "$1" up -d --remove-orphans --wait --wait-timeout "$WAIT_TIMEOUT"
}

cleanup() {
    local current dir
    current=$(current_release)
    log "Cleaning up old releases, snapshots and images"
    # shellcheck disable=SC2012
    ls -1dt "$RELEASES"/*/ | tail -n +$((KEEP_RELEASES + 1)) | while read -r dir; do
        [ "$(readlink -f "$dir")" = "$current" ] || rm -rf "$dir"
    done
    for dir in "$DUMPS"/*/; do
        [ -d "$dir" ] || continue
        # shellcheck disable=SC2012
        ls -1t "$dir"*.dump 2>/dev/null | tail -n +$((KEEP_DUMPS + 1)) | xargs -r rm -f
    done
    # Recent images are kept so rolling back does not need to pull them again
    docker image prune -af --filter until=168h >/dev/null
}

# Switches the environment to release $1, snapshotting the databases with tag
# $2. $3 is the snapshot tag to restore before starting (empty for none) and
# $4 is "reset" to start from empty databases.
switch_release() {
    local sha=$1 dump_tag=$2 restore_tag=$3 reset=$4
    local target=$RELEASES/$sha previous source

    exec 9>"$ROOT/.deploy.lock"
    flock 9

    previous=$(current_release)
    prepare_release "$target" "$sha"
    ensure_network

    log "Pulling and building images of $sha"
    compose "$target" pull --quiet --ignore-buildable
    compose "$target" build --quiet

    # The running release knows best what is running; on a first deploy only
    # the target release is available.
    source=${previous:-$target}

    log "Stopping apps, the maintenance page is served meanwhile"
    # shellcheck disable=SC2046
    compose "$source" stop $(config "$source" DEPLOY_STOPPED_SERVICES)

    dump_databases "$source" "$dump_tag"

    if [ -n "$restore_tag" ] || [ "$reset" = reset ]; then
        stop_database_clients "$source"
        recreate_databases "$source" "$restore_tag"
    fi

    log "Starting release $sha"
    if start_release "$target"; then
        ln -sfn "releases/$sha" "$ROOT/current"
        cleanup
        log "$ENV_NAME is now running $sha"
        return 0
    fi

    log "Release $sha is not healthy:"
    compose "$target" ps -a || true
    # shellcheck disable=SC2046
    compose "$target" logs --tail 50 $(config "$target" DEPLOY_STOPPED_SERVICES) || true

    if [ -z "$previous" ]; then
        die "first deploy failed, there is no previous release to roll back to"
    fi

    log "Rolling back to $(basename "$previous")"
    stop_database_clients "$target"
    recreate_databases "$target" "$dump_tag"
    if ! start_release "$previous"; then
        compose "$previous" ps -a || true
        die "ROLLBACK FAILED: $ENV_NAME needs manual intervention (snapshots tagged $dump_tag)"
    fi
    die "deploy of $sha failed, $ENV_NAME was rolled back to $(basename "$previous")"
}

case "$ACTION" in
    deploy)
        SHA=${3:-}
        [ -n "$SHA" ] || usage
        RESET=
        if [ "${4:-}" = --reset-db ]; then
            [ "$ENV_NAME" = staging ] || die "--reset-db is only allowed on staging"
            RESET=reset
        fi
        switch_release "$SHA" "pre-deploy-$SHA" "" "$RESET"
        ;;
    rollback)
        SHA=${3:-}
        [ -n "$SHA" ] || usage
        [ -d "$RELEASES/$SHA" ] || die "unknown release $SHA, available: $(ls "$RELEASES")"
        RESTORE=
        if [ "${4:-}" = --restore-db ]; then
            CURRENT=$(current_release)
            [ -n "$CURRENT" ] || die "no current release, nothing to restore"
            RESTORE="pre-deploy-$(basename "$CURRENT")"
        fi
        switch_release "$SHA" "pre-rollback-$SHA" "$RESTORE" ""
        ;;
    stop)
        [ "$ENV_NAME" = staging ] || die "only staging can be stopped"
        CURRENT=$(current_release)
        [ -n "$CURRENT" ] || die "staging has never been deployed"
        log "Stopping staging (its database is kept)"
        compose "$CURRENT" down --remove-orphans
        ;;
    *)
        usage
        ;;
esac
