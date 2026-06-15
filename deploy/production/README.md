# Tangram's Production Environment

This folder holds the production environment for Tangram's online services, deployed on `tangram-orchestre.fr`.

For CI/CD pipeline details and the backup explorer tool, see the [deploy README](../README.md).

## Infrastructure

### Docker Compose

We use [Docker Compose](https://docs.docker.com/compose/) to describe and deploy our services. Each service lives in its own subfolder with a dedicated `docker-compose.yaml`. The top-level [`docker-compose.yaml`](./docker-compose.yaml) includes all of them, providing grouped actions.

- Start everything:
    ```bash
    docker compose up -d
    ```
- Update containers:
    ```bash
    docker compose pull
    docker compose up -d
    ```

An `unhealthy_container_restarter` service runs alongside the stack and automatically restarts any container that enters an unhealthy state.

### `.env` files

Secrets are stored in `.env` files that are **not tracked by git**. Each service that needs one has a committed `sample.env` listing the required keys.

Run [`check-env-file-coherency.py`](scripts/check-env-file-coherency.py) to verify that `.env` and `sample.env` files stay in sync. Adding this script to your [pre-commit hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) is recommended.

### Reverse Proxy

All incoming web traffic on ports 80 (HTTP) and 443 (HTTPS) is routed by [Traefik](https://doc.traefik.io/traefik/). HTTP is automatically redirected to HTTPS. TLS certificates are obtained via Let's Encrypt (ACME HTTP challenge).

Traefik discovers routing rules from two sources:
- Docker labels on each service container
- Static YAML files in [`traefik/dynamic-conf/`](./traefik/dynamic-conf/)

The dashboard is exposed at `traefik.tangram-orchestre.fr`, protected by Authentik.

### Authentication

We use [Authentik](https://goauthentik.io/) as our Identity Provider. It stores user information and controls access to applications via protocols like OpenID Connect and SAML.

Traefik's [`authentik` forward-auth middleware](./traefik/dynamic-conf/authentik-middleware.yaml) protects private services (portal, Traefik dashboard, Kopia, Alloy). Authentik forwards user identity headers (`X-App-UserId`, `X-App-Email`, `X-App-Groups`, etc.) to downstream services.

The organization team also uses Authentik to log into their Google Workspace account, providing a single login for all services.

Authentik is available at `auth.tangram-orchestre.fr` and runs its own PostgreSQL database.

### Observability

Monitoring is handled by a Grafana-based stack:

| Service | Role |
|---|---|
| [Grafana](https://grafana.com/) | Dashboards and alerting (`grafana.tangram-orchestre.fr`) |
| [Alloy](https://grafana.com/oss/alloy/) | Telemetry collector, receives logs and metrics from all services |
| [Loki](https://grafana.com/oss/loki/) | Log aggregation |
| [Prometheus](https://prometheus.io/) | Metrics storage (2 week retention, 2 GB cap) |
| [Node Exporter](https://github.com/prometheus/node_exporter) | Host-level metrics |
| [cAdvisor](https://github.com/google/cadvisor) | Container-level metrics |

The backend sends logs to Alloy via OTLP, and Traefik exports metrics and logs to Alloy as well.

### Update Monitoring

[Watchtower](https://containrrr.dev/watchtower/) runs in **monitor-only** mode — it does not auto-update containers but sends notifications when new images are available.

## Services

### Routing

| Subdomain | Service | Auth |
|---|---|---|
| `www.tangram-orchestre.fr` | Showcase Website | Public |
| `www.tangram-orchestre.fr/api/public` | Backend (public API) | Public |
| `portal.tangram-orchestre.fr` | Portal | Authentik |
| `portal.tangram-orchestre.fr/api`, `api.tangram-orchestre.fr` | Backend (private API) | Authentik |
| `auth.tangram-orchestre.fr` | Authentik | — |
| `grafana.tangram-orchestre.fr` | Grafana | Grafana built-in |
| `kopia.tangram-orchestre.fr` | Kopia | Authentik |
| `traefik.tangram-orchestre.fr` | Traefik Dashboard | Authentik |
| `alloy.tangram-orchestre.fr` | Alloy | Authentik |

Bare `tangram-orchestre.fr` is redirected to `www.tangram-orchestre.fr`.

### Showcase Website

The public-facing website, built with [Nuxt](https://nuxt.com/). Its container image is `ghcr.io/tangram-orchestre/showcase-website:production`.

### Portal

The member portal, also built with Nuxt and protected behind Authentik. Its container image is `ghcr.io/tangram-orchestre/portal:production`.

### Backend

A Rust API server backed by PostgreSQL. It exposes both public endpoints (for the showcase website) and private endpoints (for the portal). Its container image is `ghcr.io/tangram-orchestre/backend:production`.

## Backups

Backups operate in two layers: **PostgreSQL daily dumps** and **Kopia snapshots to Google Drive**.

### PostgreSQL cron backups

Both the backend and Authentik databases are backed up daily at **2:00 AM** by dedicated sidecar containers that run `crond`. Each sidecar mounts the same [`postgres-backup-crontab`](./backend/postgres-backup-crontab) crontab:

```cron
0 2 * * * pg_dump -Fc > /opt/db-backups/backup.dump
```

The dumps use PostgreSQL's **custom format** (`-Fc`), which supports selective restore and compression. Each dump overwrites the previous one — point-in-time history is handled by Kopia.

| Database | Sidecar service | Dump location on host |
|---|---|---|
| Backend (`portal`) | `backend-postgres-backup` | `/opt/db-backups/backend-postgres/backup.dump` |
| Authentik (`authentik`) | `authentik-postgres-backup` | `/opt/db-backups/authentik-db/backup.dump` |

### Kopia snapshots

[Kopia](https://kopia.io/) runs as a server and takes a **daily snapshot at 5:00 AM** (after the database dumps have completed at 2 AM). It backs up the following directories to Google Drive via rclone:

| Source (read-only mount) | Contents |
|---|---|
| `/opt/docker-volumes` | All persistent service data (Authentik, Grafana, Traefik certs, etc.) |
| `/opt/services` | The deployed compose files and configuration |
| `/opt/db-backups` | The PostgreSQL dumps from both databases |

The Kopia web UI is available at `kopia.tangram-orchestre.fr`, protected by Authentik.

To explore a backup dump locally, see the [Backup Explorer](../README.md#backup-explorer-backup-explorer) in the deploy README.
