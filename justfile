set shell := ["bash", "-c"]

CONNECTOR_IMAGE_NAME := "ghcr.io/hasura/sqlserver-agent-rs"
CONNECTOR_IMAGE_TAG := "dev"
CONNECTOR_IMAGE := CONNECTOR_IMAGE_NAME + ":" + CONNECTOR_IMAGE_TAG
POSTGRESQL_CONNECTION_STRING := "sqlserverql://sqlserver:password@localhost:64002"
CHINOOK_DEPLOYMENT := "static/chinook-deployment.json"


# check everything
check: format-check find-unused-dependencies build lint test


# Notes:
# * Building Docker images will not work on macOS.
#   You can use `main` instead, by running:
#     just --set CONNECTOR_IMAGE_TAG dev-main <targets>

# run the connector
run: start-dependencies
  RUST_LOG=INFO \
    cargo run --release -- serve --configuration {{CHINOOK_DEPLOYMENT}}

# run the connector inside a Docker image
run-in-docker: build-docker-with-nix start-dependencies
  #!/usr/bin/env bash
  set -e -u -o pipefail

  configuration_file="$(mktemp)"
  trap 'rm -f "$configuration_file"' EXIT

  echo '> Generating the configuration...'
  docker run \
    --name=sqlserver-ndc-configuration \
    --rm \
    --detach \
    --platform=linux/amd64 \
    --net='sqlserver-ndc_default' \
    --publish='9100:9100' \
    {{CONNECTOR_IMAGE}} \
    configuration serve
  trap 'docker stop sqlserver-ndc-configuration' EXIT
  CONFIGURATION_SERVER_URL='http://localhost:9100/'
  ./scripts/wait-until --timeout=30 --report -- nc -z localhost 9100
  curl -fsS "$CONFIGURATION_SERVER_URL" \
    | jq --arg sqlserver_database_url 'sqlserverql://sqlserver:password@sqlserver' '. + {"sqlserver_database_url": $sqlserver_database_url}' \
    | curl -fsS "$CONFIGURATION_SERVER_URL" -H 'Content-Type: application/json' -d @- \
    > "$configuration_file"

  echo '> Starting the server...'
  docker run \
    --name=sqlserver-ndc \
    --rm \
    --interactive \
    --tty \
    --platform=linux/amd64 \
    --net='sqlserver-ndc_default' \
    --publish='8100:8100' \
    --env=RUST_LOG='INFO' \
    --mount="type=bind,source=${configuration_file},target=/deployment.json,readonly=true" \
    {{CONNECTOR_IMAGE}} \
    serve \
    --configuration='/deployment.json'

# watch the code, then test and re-run on changes
dev: start-dependencies
  RUST_LOG=INFO \
    cargo watch -i "tests/snapshots/*" \
    -c \
    -x test \
    -x clippy \
    -x 'run -- serve --configuration {{CHINOOK_DEPLOYMENT}}'

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

# re-generate the deployment configuration file
generate-chinook-configuration: build
  #!/usr/bin/env bash
  set -e -u

  cargo run --quiet -- configuration serve &
  CONFIGURATION_SERVER_PID=$!
  trap "kill $CONFIGURATION_SERVER_PID" EXIT
  ./scripts/wait-until --timeout=30 --report -- nc -z localhost 9100
  if ! kill -0 "$CONFIGURATION_SERVER_PID"; then
    echo >&2 'The server stopped abruptly.'
    exit 1
  fi
  curl -fsS http://localhost:9100 \
    | jq --arg sqlserver_database_url '{{POSTGRESQL_CONNECTION_STRING}}' '. + {"sqlserver_database_url": $sqlserver_database_url}' \
    | curl -fsS http://localhost:9100 -H 'Content-Type: application/json' -d @- \
    | jq . \
    > '{{CHINOOK_DEPLOYMENT}}'

# run sqlserver
start-dependencies:
  # start sqlserver
  docker compose down -v sqlserver
  docker compose up --wait sqlserver
  sqlcmd -S localhost,64003 -U SA -P "Password!" -i "./static/chinook-sqlserver.sql"

# run prometheus + grafana
start-metrics:
  @echo "http://localhost:3001/ for grafana console"
  docker compose up --wait prometheus grafana

# run the v3 engine binary, pointing it at our connector
run-engine: start-dependencies
  @echo "http://localhost:3000/ for graphiql console"
  @echo "http://localhost:4002/ for jaeger console"
  # Run graphql-engine using static Chinook metadata
  # we expect the `v3-engine` repo to live next door to this one
  RUST_LOG=DEBUG cargo run --release \
    --manifest-path ../v3-engine/Cargo.toml \
    --bin engine -- \
    --metadata-path ./static/chinook-metadata.json

# pasting multiline SQL into `sqlcmd` is a bad time, so here is a script to
# smash a file in for rapid fire application development business value
run-temp-sql:
  docker compose up --wait sqlserver
  sqlcmd -S localhost,64003 -U SA -P "Password!" -d "Chinook" -i "./temp.sql"

## repl-sqlserver: start a sqlserver docker image and connect to it using sqlcmd
repl-sqlserver:
  #!/usr/bin/env bash
  docker compose up -wait sqlserver
  sqlcmd -S localhost,64003 -U SA -P "Password!" -d "Chinook"

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
