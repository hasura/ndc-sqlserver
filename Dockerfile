# See https://github.com/LukeMathWalker/cargo-chef
FROM rust:1.74.0 as chef

WORKDIR app

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      lsb-release lld protobuf-compiler libssl-dev ssh git pkg-config curl jq

RUN echo $(lsb_release -rs)

# Add SQL Server ODBC Driver 17 for Ubuntu 18.04
RUN curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
RUN curl https://packages.microsoft.com/config/ubuntu/22.04/prod.list > /etc/apt/sources.list.d/mssql-release.list
RUN apt-get update
RUN ACCEPT_EULA=Y apt-get install -y --allow-unauthenticated msodbcsql18

ENV CARGO_HOME=/app/.cargo

RUN cargo install cargo-chef just grcov

RUN mkdir -p -m 0700 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts

###
# Plan recipe
FROM chef AS planner

ENV CARGO_HOME=/app/.cargo
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

COPY . .
RUN --mount=type=ssh cargo chef prepare --recipe-path recipe.json

###
# Build recipe
FROM chef AS builder

# Use lld
ENV CARGO_HOME=/app/.cargo
ENV PATH="$PATH:$CARGO_HOME/bin"
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN --mount=type=ssh cargo chef cook --release --all-targets --recipe-path recipe.json
RUN --mount=type=ssh cargo chef cook --all-targets --recipe-path recipe.json

# Copies the source after building dependencies to allow caching
COPY . .

###
# Builds the application
FROM builder AS built
# Build the app
RUN cargo build --release

###
# Ship the app in an image with `curl` and very little else
FROM ubuntu:jammy

# Install `curl` for health checks
RUN set -ex; \
    apt-get update -q; \
    apt-get install -q -y curl

# Install ndc-sqlserver
COPY --from=built /app/target/release/ndc-sqlserver /usr/local/bin
ENTRYPOINT ["ndc-sqlserver"]
