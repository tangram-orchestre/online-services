FROM node:20.19.6 AS build

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked
COPY . .

RUN mkdir /opt/openapi && mv private-spec.json /opt/openapi

# Force re-install for heyapi deps
RUN yarn install --locked && yarn generate
RUN yarn lint

# Alpine variant so the compose healthcheck can use wget
FROM joseluisq/static-web-server:2.36.0-alpine

# Expose a /health endpoint for the healthcheck
ENV SERVER_HEALTH=true

COPY --from=build /app/.output/public /public