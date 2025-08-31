# Services

This folder contains, the source code of all custom services made for Tangram.

## Layout

- [/showcase-website](showcase-website/): The site deployed at [www.tangram-orchestre.fr](https://www.tangram-orchestre.fr).
- [/backend](backend): The backend for the members portal, deployed at [api.tangram-orchestre.fr](https://api.tangram-orchestre.fr).
- [/portal](portal): The frontend of the members portal, deployed at [portal.tangram-orchestre.fr](https://portal.tangram-orchestre.fr).
- [/scripts](scripts): Common scripts useful for multiple services.
- [/openapi](openapi): Folder mounted in docker containers that will hold OpenApi generated specs.
- [/dev](dev): Development environment with integrated reverse proxy.

## Contributing

### Requirements

- [just](https://github.com/casey/just): a command runner, similar to `make`.
- [Docker](https://docs.docker.com/engine/install/): to build and run services in containers.
  - [Docker Compose](https://docs.docker.com/compose/install/): to orchestrate multiple containers.
- [mkcert](https://github.com/FiloSottile/mkcert): a simple tool to make locally-trusted development certificates.

### Development Environment

#### Starting the development environment

A complete development environment can be spawned running `just dev` from the `services` folder.

Using this script ensures docker **containers will run as your user**. This will allow you to **modify and remove work files** such as `node_modules` or `.nuxt`.

This will spawn all services (`backend`, `portal`, `database` and `showcase-website`) as well as a reverse proxy to access them.

You can access the services at the following addresses:
 - `Showcase Website`: https://localhost
 - `Backend`: https://api.localhost
 - `Portal`: https://portal.localhost

All apps are configured to hot reload / recompile on change.

#### Generating dev certificates

The development environment uses HTTPS for all services, we use `mkcert` to generate locally-trusted certificates.

This is automatically done when running `just dev` for the first time, but if you need to regenerate certificates you can run:

```bash
just generate-certs
```

**NOTE**: this must also be done manually if new domains are added to the script, which should be pretty rare.

#### Linting

We use linters and formatters for all services, you can run them all using:

```bash
just lint
```

To attempt to automatically fix issues, you can run:

```bash
just lint-fix
```

You can also run linters individually in each service.

```bash
just lint-backend
just lint-fix-backend
# ect...
```

### OpenAPI

We use OpenAPI to define the communication protocol between backend and frontends, client libraries are automatically generated and updated on backend update.
