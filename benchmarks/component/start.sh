#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

# prints its arguments to STDERR in green
function info {
  echo >&2 "> $(tput setaf 2)$*$(tput sgr0)"
}

if [[ -e ./agent.pid ]]; then
  info 'Stopping the agent from a previous run'
  AGENT_PID="$(< ./agent.pid)"
  kill "$AGENT_PID" && wait "$AGENT_PID" || :
  rm ./agent.pid
fi

info 'Building the agent'
cargo build --release -p ndc-sqlserver

info 'Starting the dependencies'
(cd ../.. && docker compose up --wait jaeger) # avoid port clobbering by re-using Jaeger
docker compose up --wait sqlserver grafana
SQLSERVER_SOCKET="$(docker compose port sqlserver 1433 | cut -f2 -d":")"

info 'Generating the deployment configuration'
mkdir -p generated
cargo run -p ndc-sqlserver --quiet --release -- configuration serve &
AGENT_PID=$!
echo "$AGENT_PID" > ./agent.pid
../../scripts/wait-until --timeout=30 --report -- cargo run -p ndc-sqlserver --quiet -- check-health --port=9100
if ! kill -0 "$AGENT_PID"; then
  echo >&2 'The agent stopped abruptly. Take a look at agent.log for details.'
  exit 1
fi

SQLSERVER_CONNECTION_STRING="DRIVER={ODBC Driver 18 for SQL Server};SERVER=127.0.0.1,${SQLSERVER_SOCKET};Uid=SA;Database=Chinook;Pwd=Password!"

echo "${SQLSERVER_CONNECTION_STRING}"

../../scripts/new-configuration.sh localhost:9100 "${SQLSERVER_CONNECTION_STRING}" \
  > ./generated/deployment.json
kill "$AGENT_PID" && wait "$AGENT_PID" || :
rm -f ./agent.pid

info 'Starting the agent'
if nc -z localhost 8100; then
  echo >&2 'ERROR: There is already an agent running on port 8100.'
  exit 1
fi

OTEL_EXPORTER_OTLP_TRACES_ENDPOINT='http://localhost:4317' \
  OTEL_SERVICE_NAME='ndc-sqlserver' \
  cargo run -p ndc-sqlserver --quiet --release -- \
    serve --configuration=./generated/deployment.json \
  >& agent.log &
AGENT_PID=$!
echo "$AGENT_PID" > ./agent.pid
echo >&2 "The agent is running with PID ${AGENT_PID}"
../../scripts/wait-until --timeout=30 --report -- cargo run -p ndc-sqlserver --quiet -- check-health

if ! kill -0 "$AGENT_PID"; then
  echo >&2 'The agent stopped abruptly. Take a look at agent.log for details.'
  exit 1
fi
