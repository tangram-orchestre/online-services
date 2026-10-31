# Deploy

This directory contains the deployment configuration for Tangram's online services. The infrastructure runs entirely on Docker Compose on a single server and is deployed via GitHub Actions.

## Layout

| Path | Content |
|---|---|
| [`infra/`](infra/README.md) | Production-only infrastructure: Traefik, Authentik, Kopia, Dozzle, Watchtower and the maintenance page |
| [`apps/`](apps/) | Showcase website, portal and backend, shared by both environments |
| [`staging/`](staging/README.md) | Staging-only debugging tools (MailDev, whoami) |
| [`production.yaml`](production.yaml) / [`staging.yaml`](staging.yaml) | Top-level compose file of each environment |
| [`production.env`](production.env) / [`staging.env`](staging.env) | Non-secret configuration of each environment (domains, routing, volumes) |
| [`scripts/deploy.sh`](scripts/deploy.sh) | Deploys, rolls back or stops an environment on the server |
| [`ansible/`](ansible/README.md) | Server provisioning |

The apps are written once and parametrised by the environment configuration, so staging runs exactly the same containers as production.

## Environments

| | Production | Staging |
|---|---|---|
| Deployed | on every push to `master` | on demand, from any branch |
| Server folder | `/opt/services` | `/opt/services-staging` |
| Compose project | `services` | `services-staging` |
| Domains | `*.tangram-orchestre.fr` | `*.staging.tangram-orchestre.fr`, all behind Authentik |
| Image tag | `production` | `staging` |

Staging reuses the production Traefik and Authentik through the shared `services` Docker network. Each environment's apps and database sit on their own private network, so they can never reach the other environment.

## How a deploy works

The [deploy workflow](../.github/workflows/deploy.yml) runs these steps:

1. **Build**: the images whose sources changed since the deployed ones (read from their `org.opencontainers.image.revision` label) are built and pushed as `sha-<commit>`. Unchanged images are tagged `sha-<commit>` too, as a registry-side alias, so every commit has a complete set of images. A backend change rebuilds the website and portal as they embed its OpenAPI spec.
2. **Upload**: this folder is rsynced to `<server folder>/releases/<commit>/`.
3. **Deploy**: [`scripts/deploy.sh`](scripts/deploy.sh) switches the server to the release:
   1. pulls the images, before any downtime
   2. stops the apps: Traefik then serves a [maintenance page](infra/maintenance/maintenance.html) (`503`) on their domains
   3. snapshots the databases to `<server folder>/db-dumps/`
   4. starts the release and waits for every container to be healthy (database migrations run here)
   5. checks the main routes answer as expected through Traefik (`DEPLOY_SMOKE_TESTS` in the environment configuration), which catches broken routing that healthchecks cannot see
   6. if anything is not healthy or a route is wrong, restores the snapshots and starts the previous release again, and the workflow fails

Infrastructure updates (Traefik, Authentik…) go through the same steps: the Authentik database is part of the snapshots, and a broken Traefik fails its healthcheck or the route checks. Traefik serves the maintenance page itself, so the page is unavailable for the few seconds Traefik restarts; it only restarts when its image or configuration changes.
4. **Promote**: only once the release is healthy, the `production` / `staging` image tags are moved to it. They therefore always point to what is running.

On the server, `current` points to the running release. Secrets are kept in `shared/`, mirroring the release tree (e.g. `shared/apps/backend/.env`), and linked into each release; each `.env` has a committed `sample.env` listing its keys ([`scripts/check-env-file-coherency.py`](scripts/check-env-file-coherency.py) checks they match).

## Staging

Staging is a single slot: deploying a branch replaces whatever was deployed before, and two staging deploys never run at the same time.

From GitHub, **Actions → Deploy → Run workflow**, then pick:
- the branch to deploy, `environment: staging`, `action: deploy`
- `reset_db` to start from an empty database, filled with the [development seeds](../services/backend/seeds/dev.sql) (useful when a branch's migrations conflict with the staging data)
- `action: stop` to stop staging when it is not needed; its database is kept for the next deploy

## Rolling back

A failed deploy rolls back automatically. To go back to an earlier release by hand, run on the server, as the `deploy` user:

```bash
/opt/services/current/scripts/deploy.sh production rollback <commit> [--restore-db]
```

`<commit>` must be one of the releases kept in `/opt/services/releases/` (the last 5). The databases are kept as they are unless `--restore-db` is passed: it restores the snapshot taken right before the current release was deployed, i.e. the data matching the previous release (anything written since is lost). The rollback itself goes through the same health checks and automatic recovery as a deploy.

## PR checks

Pull requests targeting `master` trigger a [build check](../.github/workflows/check-pr.yml) that builds the changed containers without pushing them, and lints the Ansible playbooks.

## Docker image cleanup

A [scheduled workflow](../.github/workflows/cleanup-docker-image.yml) deletes untagged image versions older than 1 day. Tagged releases are kept, so they stay available to roll back to.

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
