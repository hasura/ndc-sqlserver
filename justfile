POSTGRESQL_CONNECTION_STRING := "postgresql://postgres:password@localhost:64002"

# this is hardcoded in the V3 metadata
POSTGRES_DC_PORT := "8081"

# watch the multitenant code and re-run on changes
dev: start-docker
  RUST_LOG=DEBUG \
    cargo watch -i "tests/snapshots/*" \
    -c \
    -x test \
    -x clippy \
    -x 'run -- --deployments-dir static/deployments/'

# run postgres + jaeger
start-docker:
  # start jaeger, configured to listen to V3
  docker compose -f ../v3-experiments/crates/engine/services/dev.docker-compose.yaml up -d jaeger
  # start our local postgres
  docker compose up --wait

# run the regular V3 binary, pointing it at our multitenant agent
run-v3: start-docker
  @echo "http://localhost:3000/ for graphiql console"
  @echo "http://localhost:4002/ for jaeger console"
  # Run graphql-engine using static Chinook metadata
  # we expect the `v3-experiments` repo to live next door to this one
  RUST_LOG=DEBUG cargo run --release \
    --manifest-path ../v3-experiments/Cargo.toml \
    --bin engine -- \
    --data-connectors-config ./static/data-connectors-config-example-for-multitenant.json \
    --metadata-path ./static/metadata-example.json \
    --secrets-path ./static/secrets-example.json

# run the V3 multitenant binary, pointing it at our multitenant agent
run-v3-multitenant: start-docker
  @echo "http://localhost:4002/ for jaeger console"
  # Run graphql-engine using static Chinook metadata
  # we expect the `v3-experiments` repo to live next door to this one
  # we should also set up --otlp-endpoint to point at Jaeger
  RUST_LOG=DEBUG cargo run --release \
    --manifest-path ../v3-experiments/Cargo.toml \
    --bin multitenant -- \
    --metadata-dir ../v3-experiments/metadata/ \

# run-postgres-ndc, pointing it at local postgres etc
run-postgres-ndc: start-docker
  RUST_LOG=DEBUG \
    PORT={{POSTGRES_DC_PORT}} \
    cargo run --release \
    --bin postgres-multitenant-ndc -- \
    --deployments-dir ./static/deployments/

# start a postgres docker image and connect to it using psql
repl-postgres:
  @docker compose up --wait postgres
  psql {{POSTGRESQL_CONNECTION_STRING}}

# run a standard request to check multitenant is working
test-multitenant:
  curl -X POST \
    -H 'Host: example.hasura.app' \
      -H 'Content-Type: application/json' \
    http://localhost:3000/graphql \
    -d '{ "query": "query { AlbumByID(AlbumId: 1) { Title } } " }'

# run all tests
test: start-docker
  RUST_LOG=DEBUG \
    cargo test

# run `clippy` linter
lint +FLAGS:
  cargo clippy {{FLAGS}}

lint-apply +FLAGS:
  cargo clippy --fix {{FLAGS}}

# run rustfmt on everything
format:
  cargo fmt --all

# is everything formatted?
format-check:
  cargo fmt --all -- --check

