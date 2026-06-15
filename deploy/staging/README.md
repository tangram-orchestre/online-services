# Tangram's Staging Environment

Deployed from the `develop` branch to `/opt/services-staging/` on the server. Mirrors the production stack on `*.staging.tangram-orchestre.fr` subdomains.

## Differences from production

- **No dedicated infrastructure** — staging piggybacks on the production Traefik, Authentik, and observability stack (services join the shared `services` network).
- **Mail is intercepted** — emails are sent to a [MailDev](https://github.com/maildev/maildev) instance instead of a real SMTP server, accessible at `maildev.tangram-orchestre.fr`.
- **All routes require Authentik** — including the showcase website and its public API, to keep staging private.
- **Includes a `whoami` debug service** — at `whoami.tangram-orchestre.fr` to inspect forwarded headers.

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

Staging has its own PostgreSQL instance (`backend-postgres-staging`) with a daily backup cron job at 2:00 AM, identical to production.
