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
- [/services](services): source code for all services made for Tangram (see [services/README.md](services/README.md) for details).

## Development Flow

We use the [Git Flow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) process to organize our work:
 - the `master` branch services are automatically deployed to production at `https://<service>.tangram-orchestre.fr`
   - the `production` branch is updated to the currently deployed commit
 - the `develop` branch services are automatically deployed to staging environment at `https://<service>.staging.tangram-orchestre.fr`
   - the `staging` branch is updated to the currently deployed commit
