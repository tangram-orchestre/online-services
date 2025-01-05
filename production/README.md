# Tangram's Production Environment

This folder holds Tangram website production environment.

## Infrastructure

### Docker Compose

We use [docker compose](https://docs.docker.com/compose/) to describe and deploy our services. Each service is put in its separate folder with its own `docker-compose.yaml` configuration.

At the top level a single `docker-compose.yaml` includes the ones in the subfolders to provide grouped action.

- To start the infrastructure simply run:
    ```bash
    docker compose up -d
    ```
- To update containers, update versions in `docker-compose.yaml` and then:
    ```bash
    docker compose pull
    docker compose up -d
    ```

### Automatic deployment

The infrastructure is automatically deployed at each push on master, see [ci.yml](../.github/workflows/ci.yml):
 1. First this folder is `rsynced` to the production server
 2. Then the [update.sh](./update.sh) is run

### Backups

We use [kopia](https://kopia.io/) to backup the configuration to our Google Drive.

### .env files

To keep secrets secret, we use `.env` file that are not tracked by `git`, samples files are commited to know what key are needed.

You can run [check-env-file-coherency.py](scripts/check-env-file-coherency.py) to assert real and samples files are synchronised. Adding the script in your [pre-commit hooks](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) is also a good idea.

### Reverse Proxy

All web trafic arriving to the server comes to 80 (HTTP) and 443 (HTTPS) ports. To properly route this trafic to the correct service we use the [Traefik](https://doc.traefik.io/traefik/) reverse proxy.

It discovers routing configuration via its [dynamic-conf](./traefik/dynamic-conf/) folder and docker labels. Then it routes the trafic based on discovered rules.

### Authentification

To authenticate and authorize requests to our services we use the *Identity Provider* [Authentik](https://goauthentik.io/). This service acts as the single database to store all your user information and permission and control access to application by supporting a variety of protocols (eg. OpenId, SAML...).

Even the organization team logs through Authentik to log into their Google Workspace account.

This simplifies the authentication story for users, only one login for all our services!

### Updates Monitoring

To know when an update is available, we use [Watchtower](https://containrrr.dev/watchtower/), which monitors docker containers versions and sends notfications via email.

## Services

#### [Planka](https://planka.app/)

An open source clone of Trello, used to manage tasks in a Kanban fashion.
