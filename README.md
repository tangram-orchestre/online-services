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

- [/deploy](deploy): docker-compose files and scripts used on the production server (see [deploy/README.md](deploy/README.md)).
  - [/infra](deploy/infra/): The shared infrastructure: reverse proxy, authentication server, backups, monitoring.
  - [/apps](deploy/apps/): The showcase website, member portal and backend, deployed identically to production and staging.
  - [/staging](deploy/staging/): Debugging tools only available on staging.
- [/services](services): source code for all services made for Tangram (see [services/README.md](services/README.md) for details).

## Development Flow

Work happens on feature branches merged into `master` through pull requests:
 - `master` is automatically deployed to production at `https://<service>.tangram-orchestre.fr`
 - any branch can be deployed on demand to the staging environment at `https://<service>.staging.tangram-orchestre.fr` (see [deploy/README.md](deploy/README.md#staging))
