#!/usr/bin/env bash
# Regenerates the SchemaSpy documentation in /output whenever the database
# schema changes (migrations applied by the backend at startup or through
# `just diesel`). With --once, generates it once and exits.
#
# Connection settings come from the standard PG* environment variables.
set -u

OUTPUT=/output
INTERVAL=3

# Changes whenever a table, column, constraint, index, trigger or function of
# the public schema changes.
FINGERPRINT_QUERY="
SELECT md5(coalesce(string_agg(item, '|' ORDER BY item), '')) FROM (
  SELECT format('column %s.%s %s %s %s', table_name, column_name, data_type, is_nullable, column_default)
    FROM information_schema.columns WHERE table_schema = 'public'
  UNION ALL
  SELECT format('constraint %s %s %s', conrelid::regclass, conname, pg_get_constraintdef(oid))
    FROM pg_constraint WHERE connamespace = 'public'::regnamespace
  UNION ALL
  SELECT format('index %s', pg_get_indexdef(indexrelid))
    FROM pg_index i JOIN pg_class c ON c.oid = i.indrelid
    WHERE c.relnamespace = 'public'::regnamespace
  UNION ALL
  SELECT format('trigger %s', pg_get_triggerdef(oid))
    FROM pg_trigger WHERE NOT tgisinternal
  UNION ALL
  SELECT format('function %s %s', proname, md5(prosrc))
    FROM pg_proc WHERE pronamespace = 'public'::regnamespace
) AS schema(item);"

generate() {
  local tmp
  tmp=$(mktemp -d)
  # The image launcher writes to $SCHEMASPY_OUTPUT.
  if ! SCHEMASPY_OUTPUT=$tmp /usr/local/bin/schemaspy -t pgsql11 -host "$PGHOST" \
    -port "${PGPORT:-5432}" -db "$PGDATABASE" -u "$PGUSER" -p "$PGPASSWORD" -s public; then
    rm -rf "$tmp"
    return 1
  fi
  # Replace the whole output, so that dropped tables don't linger. Only the
  # content is copied: the temporary directory is private, the output must stay
  # readable by the viewer.
  find "$OUTPUT" -mindepth 1 -delete
  cp -R "$tmp"/* "$OUTPUT/"
  chmod 755 "$OUTPUT"
  rm -rf "$tmp"
}

fingerprint() {
  psql -XAtq -c "$FINGERPRINT_QUERY" 2>/dev/null
}

until fingerprint >/dev/null; do
  echo "Waiting for the database..."
  sleep "$INTERVAL"
done

if [ "${1:-}" = "--once" ]; then
  generate
  exit
fi

last=""
while true; do
  current=$(fingerprint)
  if [ -n "$current" ] && [ "$current" != "$last" ]; then
    echo "Schema changed, regenerating the documentation..."
    generate && last=$current
  fi
  sleep "$INTERVAL"
done
