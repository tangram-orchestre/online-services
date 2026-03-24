FROM node:20.19.6 AS build

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked
COPY . .

RUN mkdir /opt/openapi && mv private-spec.json /opt/openapi

# Force re-install for heyapi deps
RUN yarn install --locked && yarn generate
RUN yarn lint

FROM joseluisq/static-web-server:2.36.0

COPY --from=build /app/.output/public /public