# Tangram Online Services

![CI Status](https://github.com/tangram-orchestre/showcase-website/actions/workflows/ci.yml/badge.svg)
![Website](https://img.shields.io/badge/Showcase_Website-Online-blue?link=https://www.tangram-orchestre.fr)

This repository contains source code for every online services deployed for the [**Tangram Orchestra**](https://www.tangram-orchestre.fr) non-profit.

## Tech Stack

- 📺 The front-ends are build using [Vue.js](https://vuejs.org/), [Nuxt](https://nuxt.com/) and [TypeScript](https://www.typescriptlang.org/).
- ⚙️ The back-end is built using [Rust](https://www.rust-lang.org/) 🦀 and [poem-openapi](https://crates.io/crates/poem-openapi)
- 🔀 The communication between the backend and the frontend are guaranteed to be type safe using [OpenAPI](https://swagger.io/specification/) and code generation with [heyapi](https://heyapi.dev/).
- 🔒 The authentication is done with [Authentik](https://goauthentik.io/).
- 🛣️ Requests are routed with [Traefik](https://doc.traefik.io/traefik/).
- 🔄 _Staging_ and _Production_ environments are continuously deployed using [Docker](https://www.docker.com/) 🐳 and [GitHub Actions](https://github.com/features/actions) 🐈 to an [OVH](https://www.ovhcloud.com) VPS.

## Repository Content

- [/deploy](deploy): docker-compose files used on production server.
  - [/production](deploy/production/): The production environment, hosts authentication server, showcase website, member portal and other services. Deployed automatically from `master` branch.
  - [/staging](deploy/staging/): The staging environment to test services before releasing to production. Deployed automatically from `develop` branch.
- [/services](services): source code for all services made for Tangram.
  - [/showcase-website](services/showcase-website/): The site deployed at [www.tangram-orchestre.fr](https://www.tangram-orchestre.fr).
  - [/backend](services/backend): The backend for the members portal, deployed at [api.tangram-orchestre.fr](https://api.tangram-orchestre.fr).
  - [/portal](services/portal): The frontend of the members portal, deployed at [api.tangram-orchestre.fr](https://api.tangram-orchestre.fr).
  - [/scripts](services/scripts): Common scripts useful for multiple services.
  - [/openapi](services/openapi): Folder mounted in docker containers that will hold OpenApi generated specs.
  - [/dev](services/dev): Development environment with integrated reverse proxy.

## Development Flow

We use the [Git Flow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) process to organize our work:
 - the `master` branch services are automatically deployed to production at `https://<service>.tangram-orchestre.fr`
   - the `production` branch is updated to the currently deployed commit
 - the `develop` branch services are automatically deployed to staging environment at `https://<service>.staging.tangram-orchestre.fr`
   - the `staging` branch is updated to the currently deployed commit

## Contributing

### Requirements

- [Docker](https://docs.docker.com/engine/install/)
- [mkcert](https://github.com/FiloSottile/mkcert)

### Development Environment

A complete development environment can be spawned through [docker compose](https://docs.docker.com/compose/).

#### Generating dev certificates

First step is to use [mkcert](https://github.com/FiloSottile/mkcert) to generate local development certificates:

```bash
# To be done once when installing mkcert for the first time
mkcert -install

# Generate certificates for localhost
./services/dev/generate-certs.sh
```

**NOTE**: this must be done if new domain are added to the script, which should be pretty rare.

#### Starting the development environment

From the root of the repo simply run:

```bash
./services/dev/run.sh
```

Using this script ensures docker containers will run as your user. This will allow you to modify and remove work files such as `node_modules` or `.nuxt`.

This will spawn all services (backend, portal, database and showcase-website) as well as a reverse proxy to access them.

You can access the services at the following addresses:
 - Showcase Website: https://localhost
 - Backend: https://api.localhost

All apps are configured to hot reload / recompile on change.

### OpenAPI

We use OpenAPI to define the communication protocol between backend and frontends, client libraries are automatically generated.