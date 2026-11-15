FROM node:24.22.2 AS build

# pnpm, at the version of the packageManager field of package.json
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable pnpm

WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .

RUN mkdir /opt/openapi && mv private-spec.json /opt/openapi

# The install's `nuxt prepare` ran without the sources and the spec: generate
# the types and the API client again, which the build type-checks against.
RUN pnpm nuxt prepare && pnpm generate
RUN pnpm lint

# Alpine variant so the compose healthcheck can use wget
FROM joseluisq/static-web-server:2.36.0-alpine

# Expose a /health endpoint for the healthcheck
ENV SERVER_HEALTH=true

COPY --from=build /app/.output/public /public