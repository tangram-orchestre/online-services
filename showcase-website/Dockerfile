FROM oven/bun:1.2.4 AS build

WORKDIR /app
COPY package.json bun.lock ./
RUN bun install --locked
COPY . .

ENV PRIVATE_API_BASE_URL=placeholder
ENV PUBLIC_API_BASE_URL=placeholder

RUN mkdir /opt/openapi && mv spec.json /opt/openapi
RUN bun run openapi-ts

RUN bun build
RUN bun lint

FROM oven/bun:1.2.4-slim

WORKDIR /app
COPY --from=build /app/.output ./

EXPOSE 3000
CMD [ "bun", "server/index.mjs" ]