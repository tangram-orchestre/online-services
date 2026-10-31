# Tangram's Staging Environment

Deployed on demand from any branch to `/opt/services-staging/` on the server (see the [deploy README](../README.md#staging) for how to deploy, reset or stop it). It runs the same [apps](../apps/) as production, configured by [`staging.env`](../staging.env), on `*.staging.tangram-orchestre.fr` subdomains.

This folder only holds the staging-only debugging tools, included by [`staging.yaml`](../staging.yaml).

## Differences from production

- **No dedicated infrastructure**: staging piggybacks on the production Traefik and Authentik (through the shared `services` network). Its apps and database are on their own private network.
- **Mail is intercepted**: emails are sent to a [MailDev](https://github.com/maildev/maildev) instance instead of a real SMTP server, accessible at `maildev.tangram-orchestre.fr`.
- **All routes require Authentik**: including the showcase website and its public API, to keep staging private.
- **The database is seeded**: the backend loads the idempotent [development seeds](../../services/backend/seeds/dev.sql) at startup.
- **Includes a `whoami` debug service**: at `whoami.tangram-orchestre.fr` to inspect forwarded headers.

## Routing

| Subdomain | Service |
|---|---|
| `www.staging.tangram-orchestre.fr` | Showcase Website (staging) |
| `www.staging.tangram-orchestre.fr/api/public` | Backend public API (staging) |
| `portal.staging.tangram-orchestre.fr` | Portal (staging) |
| `api.staging.tangram-orchestre.fr` | Backend API (staging) |
| `maildev.tangram-orchestre.fr` | MailDev |
| `whoami.tangram-orchestre.fr` | Whoami |

## Database

Staging has its own PostgreSQL instance, stored in `/opt/docker-volumes/backend-staging-db`, with a daily backup cron job at 2:00 AM identical to production. It is snapshotted before each deploy and restored if the deploy fails, like production.
