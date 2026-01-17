FROM node:20.16.0 AS build

WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked
COPY . .

ENV PRIVATE_API_BASE_URL=placeholder
ENV PUBLIC_API_BASE_URL=placeholder

RUN mkdir /opt/openapi && mv public-spec.json /opt/openapi
RUN yarn run openapi-ts

RUN yarn build
RUN yarn lint

FROM node:20.16.0 AS deps
WORKDIR /app
COPY package.json yarn.lock ./
RUN yarn install --locked --production

FROM node:20.16.0-slim

WORKDIR /app
COPY --from=build /app/.output ./
COPY --from=deps  /app/node_modules ./node_modules

EXPOSE 3000
CMD [ "node", "server/index.mjs" ]