FROM node:24.22.2 AS build

# pnpm, at the version of the packageManager field of package.json
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable pnpm

WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .

ENV PRIVATE_API_BASE_URL=placeholder
ENV PUBLIC_API_BASE_URL=placeholder

RUN mkdir /opt/openapi && mv public-spec.json /opt/openapi
RUN pnpm openapi-ts

# The install's `nuxt prepare` ran without the sources: generate the types
# again, which the build type-checks against.
RUN pnpm nuxt prepare && pnpm build
RUN pnpm lint

FROM node:24.22.2 AS deps
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable pnpm
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile --prod

FROM node:24.22.2-slim

WORKDIR /app
COPY --from=build /app/.output ./
COPY --from=deps  /app/node_modules ./node_modules

EXPOSE 3000
CMD [ "node", "server/index.mjs" ]