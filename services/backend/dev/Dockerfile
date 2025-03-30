FROM ubuntu:24.04

ARG UID=1000
ARG GID=1000

RUN apt-get update && apt-get install -y curl build-essential libpq-dev bash pkg-config

# Install rust
ENV RUSTUP_HOME=/opt/rustup_home
ENV CARGO_HOME=/opt/cargo_home
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.92.0 -y

ENV PATH=/opt/cargo_home/bin:$PATH

# Install tools
RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo install watchexec-cli

# Set env variables
ENV CARGO_TARGET_DIR=/opt/cargo_target_dir
RUN mkdir -p ${CARGO_TARGET_DIR}

ENV CARGO_REGISTRY_DIR=${CARGO_HOME}/registry
RUN mkdir -p ${CARGO_REGISTRY_DIR}

# Set permissions
RUN chown -R ${UID}:${GID} ${CARGO_HOME} ${CARGO_TARGET_DIR}
