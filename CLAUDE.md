# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

Monorepo for Tangram Orchestra's web presence. Contains three services (backend, showcase website, members portal) with shared infrastructure. All work happens under `services/`, deployment configs live in `deploy/`.

## Commands

All tasks run via `just` from the `services/` directory.

```bash
just dev                      # Start complete local dev environment
just build-env                # Build Docker images
just lint                     # Lint all three services
just lint-fix                 # Auto-fix lint issues in all services
just lint-backend
just lint-showcase-website
just lint-portal
just diesel <cmd>             # Run Diesel CLI (e.g. migration run/revert/generate)
just generate-certs           # Regenerate mkcert HTTPS certificates
```

Individual service commands (run from their respective directories):

```bash
# Backend
cargo fmt
cargo clippy -- -D warnings
cargo run

# Portal / Showcase Website
yarn dev
yarn build
yarn lint
yarn lint:fix
yarn openapi-ts               # Regenerate API client from OpenAPI spec
```

There are no unit tests — linting is the primary code quality gate.

## Architecture

### Tech Stack

| Service | Framework | Language | Notes |
|---|---|---|---|
| `backend` | Poem + poem-openapi | Rust | PostgreSQL via Diesel ORM |
| `showcase-website` | Nuxt 3 + Tailwind | TypeScript | Public-facing site |
| `portal` | Nuxt 4 + Vuetify 3 | TypeScript | Members-only admin portal |

### Type-Safe API Contract

The backend auto-generates two OpenAPI specs at startup:
- `/public/spec` → consumed by showcase-website
- `/spec` → consumed by portal

Frontend API clients are generated from these specs via `@hey-api/openapi-ts` (`yarn openapi-ts`). The generated clients live in `services/*/client/`. When backend schemas change, regenerate clients before linting.

### Backend Structure (`services/backend/src/`)

- `main.rs` — app init, routes, OpenTelemetry setup
- `settings.rs` — all config via `APP_*` env vars
- `public.rs` — public API endpoints (no auth)
- `private/` — private endpoints (auth required): concerts, locations, pieces, semesters, users
- `models/` — Diesel database models
- `schema.rs` — Diesel schema (auto-generated, do not edit manually)
- `migrations/` — Diesel migrations

### Frontend Structure

Both Nuxt apps follow the same pattern:
- `client/` — auto-generated API client (do not edit manually)
- `*-spec.json` — OpenAPI spec snapshot used for code generation

Portal uses Nuxt 4 conventions: source lives in `app/` (pages, components, assets).

### Local Dev Environment

`services/docker-compose.yaml` runs everything:
- **Traefik** terminates TLS and routes `*.localhost` domains
- **fake-auth** — mock Authentik OIDC for local development
- **postgres** — PostgreSQL 17, seeded from `backend/seeds/dev.sql`
- **pgweb** at `https://pgweb.localhost`
- **maildev** at `https://maildev.localhost`
- **lgtm** (Grafana LGTM stack) at `https://grafana.localhost`

Containers run as your local UID/GID (set in `.env`) so generated files (node_modules, .nuxt) are editable without sudo.

## Deployment

- `master` → production at `tangram-orchestre.fr` (auto-deployed by CI)
- `develop` → staging at `*.staging.tangram-orchestre.fr` (auto-deployed by CI)

CI builds only changed services (path-filtered), pushes images to GHCR, rsyncs deploy configs to OVH VPS, then SSHs in to run `update.sh`. OpenAPI specs are regenerated from backend on every CI run and compared via md5sum to decide whether frontends need rebuilding.

`production` and `staging` git branches are force-pushed by CI to track the currently deployed commit.
