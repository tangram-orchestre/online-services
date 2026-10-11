# Deploy

This directory contains the deployment configuration for Tangram's online services. The infrastructure runs entirely on Docker Compose and is automatically deployed via GitHub Actions.

## Environments

### Production ([`production/`](production/README.md))

Deployed from the `master` branch to `/opt/services/` on the server. Hosts the live services at `tangram-orchestre.fr`.

See [production/README.md](production/README.md) for infrastructure, services, and backup details.

### Staging ([`staging/`](staging/README.md))

Deployed from the `develop` branch to `/opt/services-staging/` on the server. Mirrors the production stack on `*.staging.tangram-orchestre.fr` subdomains, reusing the production Traefik, Authentik, and observability infrastructure.

See [staging/README.md](staging/README.md) for details.

## CI/CD

The project follows a two-branch workflow: `develop` (staging) and `master` (production). All CI/CD is defined in [`.github/workflows/`](../.github/workflows/).

### Build & Deploy pipeline

On every push to `develop` or `master`, the [CI/CD workflow](../.github/workflows/ci.yml) runs:

| Branch | Image tag | Deploy source | Deploy target | Update script |
|---|---|---|---|---|
| `master` | `production` | `deploy/production/` | `/opt/services/` | [`production/update.sh`](production/update.sh) |
| `develop` | `staging` | `deploy/staging/` | `/opt/services-staging/` | [`staging/update.sh`](staging/update.sh) |

Steps:

1. **Build containers** — Only the services whose source files changed are rebuilt, detected via [`dorny/paths-filter`](https://github.com/dorny/paths-filter). The backend is built first, then its OpenAPI spec is extracted; the portal and showcase website are only rebuilt if their own code changed **or** if the OpenAPI spec they depend on has changed (compared via md5sum).
2. **Push to GHCR** — Built images are pushed to `ghcr.io/tangram-orchestre/` and tagged `production` or `staging`.
3. **Rsync deploy folder** — The matching deploy directory is rsynced to the server (excluding `.env` files).
4. **Run update script** — On the server, `update.sh` pulls the latest images and restarts containers.
5. **Update marker branch** — A `production` or `staging` branch is force-pushed to track what's currently deployed.

### Automated pull requests

- **Push to `develop`** → a draft PR "[Deploy develop to master](../.github/workflows/auto-pr-deploy-develop-to-master.yml)" is automatically created.
- **Push to `master`** → a draft PR "[Merge back master to develop](../.github/workflows/auto-pr-merge-back-master-to-develop.yml)" is created if there are differences (e.g. hotfixes applied directly on `master`).

### PR checks

Non-draft pull requests targeting `develop` or `master` trigger a [build check](../.github/workflows/check-pr.yml) that builds the containers without pushing, to validate the build passes.

### Docker image cleanup

A [scheduled workflow](../.github/workflows/cleanup-docker-image.yml) runs daily and deletes container images older than 1 day that are not tagged `production` or `staging`.

## Backup Explorer (`backup-explorer/`)

A local-only tool to inspect PostgreSQL backup dumps. Usage:

```bash
just explore_backup path/to/backup.dump
```

This spins up a temporary Docker Compose stack that:
1. Starts a fresh PostgreSQL instance
2. Restores the dump into it using `pg_restore`
3. Opens [pgweb](https://github.com/sosedoff/pgweb) at `http://localhost:8081` so you can browse tables and run queries

Press any key in the terminal to tear it down.
