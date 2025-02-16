FROM rust:1.84.1-bullseye AS build
WORKDIR /opt/backend

# Small trick to cache dependencies using docker layer caching
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --locked --release

COPY . .
RUN cargo build --locked --release

FROM gcr.io/distroless/cc-debian12 AS backend
WORKDIR /opt/backend
COPY --from=build /opt/backend/target/release/backend /backend
ENTRYPOINT ["/backend"]