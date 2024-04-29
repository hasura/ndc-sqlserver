#!/usr/bin/env bash

set -e

/docker/import-chinook.sh &
exec /opt/mssql/bin/sqlservr "$@"
