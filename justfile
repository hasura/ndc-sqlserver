set shell := ["bash", "-c"]

CONNECTOR_IMAGE_NAME := "ghcr.io/hasura/sqlserver-agent-rs"
CONNECTOR_IMAGE_TAG := "dev"
CONNECTOR_IMAGE := CONNECTOR_IMAGE_NAME + ":" + CONNECTOR_IMAGE_TAG
CHINOOK_DEPLOYMENT := "static/chinook-deployment.json"
SQLSERVER_CONNECTION_STRING := "DRIVER={ODBC Driver 18 for SQL Server};SERVER=127.0.0.1,64003;Uid=SA;Database=Chinook;Pwd=Password!"

# check everything
check: format-check find-unused-dependencies build lint test


# Notes:
# * Building Docker images will not work on macOS.
#   You can use `main` instead, by running:
#     just --set CONNECTOR_IMAGE_TAG dev-main <targets>

# run the connector
run: start-dependencies
  RUST_LOG=INFO \
    OTLP_ENDPOINT=http://localhost:4317 \
    OTEL_SERVICE_NAME=ndc-sqlserver \
    cargo run --release -- serve --configuration {{CHINOOK_DEPLOYMENT}}

# watch the code, then test and re-run on changes
dev: start-dependencies
  RUST_LOG=INFO \
    OTLP_ENDPOINT=http://localhost:4317 \
    OTEL_SERVICE_NAME=ndc-sqlserver \
    cargo watch -i "tests/snapshots/*" \
    -c \
    -x test \
    -x clippy \
    -x 'run -- serve --configuration {{CHINOOK_DEPLOYMENT}}'

# watch the code, then test and re-run config server ron changes
dev-config: start-dependencies
  RUST_LOG=DEBUG \
    OTLP_ENDPOINT=http://localhost:4317 \
    OTEL_SERVICE_NAME=ndc-sqlserver \
    cargo watch -i "tests/snapshots/*" \
    -c \
    -x clippy \
    -x 'run -- configuration serve'

# re-generate the deployment configuration file
generate-chinook-configuration: build start-dependencies
  ./scripts/archive-old-deployment.sh '{{CHINOOK_DEPLOYMENT}}'
  ./scripts/generate-chinook-configuration.sh 'ndc-sqlserver' '{{SQLSERVER_CONNECTION_STRING}}' '{{CHINOOK_DEPLOYMENT}}'

test-introspection:
  #!/bin/bash

  CONFIGURATION_SERVER_URL='http://localhost:9100/'
  curl -fsS "$CONFIGURATION_SERVER_URL" \
    | jq --arg sqlserver_database_url '{{ SQLSERVER_CONNECTION_STRING }}' '. + {"mssql_connection_string": $sqlserver_database_url}' \
    | curl -fsS "$CONFIGURATION_SERVER_URL" -H 'Content-Type: application/json' -d @- \

# watch the code, and re-run on changes
watch-run: start-dependencies
  RUST_LOG=DEBUG \
    cargo watch -i "tests/snapshots/*" \
    -c \
    -x 'run -- serve --configuration {{CHINOOK_DEPLOYMENT}}'

# Run ndc-sqlserver with rust-gdb.
debug: start-dependencies
  cargo build
  RUST_LOG=DEBUG \
    rust-gdb --args target/debug/ndc-sqlserver serve --configuration {{CHINOOK_DEPLOYMENT}}

# Run the server and produce a flamegraph profile
flamegraph: start-dependencies
  RUST_LOG=DEBUG \
    cargo flamegraph --dev -- \
    serve --configuration {{CHINOOK_DEPLOYMENT}}

# build everything
build:
  cargo build --all-targets

# run all tests
test: start-dependencies
  RUST_LOG=DEBUG \
    cargo test

# run a standard request to check the service correctly integrates with the engine
test-integrated:
  curl -X POST \
    -H 'Host: example.hasura.app' \
    -H 'Content-Type: application/json' \
    http://localhost:3000/graphql \
    -d '{ "query": "query { AlbumByID(AlbumId: 1) { Title } } " }'

# run sqlserver
start-dependencies:
  docker compose up --wait sqlserver jaeger

# run prometheus + grafana
start-metrics:
  @echo "http://localhost:3001/ for grafana console"
  docker compose up --wait prometheus grafana

# run the v3 engine binary, pointing it at our connector
run-engine: start-dependencies
  @echo "http://localhost:3000/ for graphiql console"
  @echo "http://localhost:4002/ for jaeger console"
  docker compose up --wait auth-hook
  # Run graphql-engine using static Chinook metadata
  # we expect the `v3-engine` repo to live next door to this one
  RUST_LOG=DEBUG \
    OTLP_ENDPOINT=http://localhost:4317 \
    cargo run --release \
    --manifest-path ../v3-engine/Cargo.toml \
    --bin engine -- \
    --metadata-path ./static/chinook-metadata.json \
    --authn-config-path ./static/auth_config.json

# pasting multiline SQL into `sqlcmd` is a bad time, so here is a script to
# smash a file in for rapid fire application development business value
run-temp-sql:
  docker compose up --wait sqlserver
  docker exec -it ndc-sqlserver-sqlserver-1 /opt/mssql-tools/bin/sqlcmd -S localhost,1433 -U SA -P "Password!" -d "Chinook" -i "/static/temp.sql"

## repl-sqlserver: start a sqlserver docker image and connect to it using sqlcmd
repl-sqlserver:
  #!/usr/bin/env bash
  docker compose up --wait sqlserver
  docker exec -it ndc-sqlserver-sqlserver-1 /opt/mssql-tools/bin/sqlcmd -S localhost,1433 -U SA -P "Password!" -d "Chinook"

# run `clippy` linter
lint *FLAGS:
  cargo clippy {{FLAGS}}

lint-apply *FLAGS:
  cargo clippy --fix {{FLAGS}}

# run rustfmt on everything
format:
  cargo fmt --all

# is everything formatted?
format-check:
  cargo fmt --all -- --check

find-unused-dependencies:
  cargo machete

# check the nix build works
build-with-nix:
  nix build --print-build-logs

# check the docker build works
build-docker-with-nix:
  #!/usr/bin/env bash
  if [[ '{{CONNECTOR_IMAGE_TAG}}' == 'dev' ]]; then
    echo 'nix build | docker load'
    docker load < "$(nix build --no-link --print-out-paths '.#dockerDev')"
  fi
