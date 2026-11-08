FROM node:24.22.2 AS build

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked
COPY . .

ENV PRIVATE_API_BASE_URL=placeholder
ENV PUBLIC_API_BASE_URL=placeholder

RUN mkdir /opt/openapi && mv public-spec.json /opt/openapi
RUN yarn run openapi-ts

# The install's `nuxt prepare` ran without the sources: generate the types
# again, which the build type-checks against.
RUN yarn nuxt prepare && yarn build
RUN yarn lint

FROM node:24.22.2 AS deps
WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked --production

FROM node:24.22.2-slim

WORKDIR /app
COPY --from=build /app/.output ./
COPY --from=deps  /app/node_modules ./node_modules

EXPOSE 3000
CMD [ "node", "server/index.mjs" ]