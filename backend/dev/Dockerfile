FROM ubuntu:24.04

RUN apt-get update && apt-get install -y curl build-essential libpq-dev bash

# Install rust
ENV RUSTUP_HOME /opt/rustup_home
ENV CARGO_HOME /opt/cargo_home
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.84.0 -y

ENV PATH /opt/cargo_home/bin:$PATH

# Install tools
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall diesel_cli
RUN cargo binstall watchexec-cli
