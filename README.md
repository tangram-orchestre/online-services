# Tangram Online Services

This repository contains source code for every online services deployed for the **Tangram Orchestra** non-profit.

## Repository layout

- [/deploy](deploy)
  - [/production](deploy/production/) : The production environment, hosts authentication server, showcase website, member portal and other services. Deployed automatically from `master` branch.
  - [/staging](deploy/staging/) : A staging environment to test *almost* services before releasing to production. Deployed automatically from `develop` branch.
- [/showcase-website](showcase-website/) : The site deployed at [www.tangram-orchestre.fr](https://www.tangram-orchestre.fr).
- [/backend](backend) : The backend for the members portal, deployed at [api.tangram-orchestre.fr](https://api.tangram-orchestre.fr).
- [/scripts](scripts) : Common scripts useful for multiple services.
- [/openapi](scripts) : Folder mounted in docker containers that will hold OpenApi generated specs.
- [/dev](scripts) : Development environment with integrated reverse proxy.

## Development Flow

We use the [Git Flow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow) process to organize our work:
 - the `master` branch is automatically deployed to production at [https://*.tangram-orchestre.fr]
   - the `production` branch is updated to the currently deployed commit
 - the `develop` branch is automatically deployed to staging environment at [https://*.staging.tangram-orchestre.fr]
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
./dev/generate-certs.sh
```

**NOTE**: this must be done if new domain are added to the script, which should be pretty rare.

#### Starting the development environment

From the root of the repo simply run:

```bash
./dev/run.sh
```

Using this script ensures docker containers will run as your user. This will allow you to modify and remove work files such as `node_modules` or `.nuxt`.

This will spawn all services (backend, portal, database and showcase-website) as well as a reverse proxy to access them.

You can access the services at the following addresses:
 - Showcase Website: https://localhost
 - Backend: https://api.localhost

All apps are configured to hot reload / recompile on change.

### OpenAPI

We use OpenAPI to define the communication protocol between backend and frontends, client libraries are automatically generated.