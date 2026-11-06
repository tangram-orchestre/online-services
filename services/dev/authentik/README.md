# Local Authentik

`just dev-authentik` starts the dev stack with a real [Authentik](https://goauthentik.io/) at `https://auth.localhost` instead of `dev/fake-auth`, like in production: the portal redirects to the Authentik login page, and Authentik sends the `X-App-*` headers to the backend.

`just dev` stays the quickest way to work on the portal, switching users by editing `dev/fake-auth/current-user.json`. Use `just dev-authentik` to work on login, sign-up, the emails or the [blueprints](../../../deploy/infra/authentik/blueprints/README.md). Switching from one to the other recreates the backend and portal containers.

## What it runs

- [`docker-compose.yaml`](docker-compose.yaml), added to the main one: Authentik's server, worker and database (kept in `docker-data/authentik-db`), and the Traefik routes going through the [`authentik` middleware](../traefik/dynamic-conf/authentik.yaml) instead of fake-auth. The public API stays open, as in production.
- The production blueprints, except those of `production/` (staging and infrastructure applications, Google), on the `localhost` domain.
- [`blueprints/seed.yaml`](blueprints/seed.yaml): the outpost and a user per role.
- Emails (sign-up confirmation, password recovery) go to MailDev at `https://maildev.localhost`.
- [`branding/`](branding) and two images of the showcase website stand in for the branding images uploaded in production.

## Users

The password of each user is their username. It is also their app password, so tools can authenticate with HTTP basic auth instead of logging in.

| User | Groups | In the portal |
|---|---|---|
| `bureau` | Orga, Bureau | Sees and edits everything |
| `orga` | Orga | Sees everything, read only |
| `membre` | none | Only their profile |
| `dev` | Orga, Devs | Read only, and the API at `https://api.localhost` |
| `akadmin` (password `admin`) | authentik Admins | The Authentik admin |

```sh
curl -u orga:orga https://portal.localhost/api/users/me
just screenshot --user bureau /concerts
```

To start over, stop the stack and delete `docker-data/authentik-db`.
