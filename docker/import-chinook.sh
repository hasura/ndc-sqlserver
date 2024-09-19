#!/usr/bin/env bash

set -e
set -u

# do something on the database
# We need to use the mssql-tools18 version of sqlcmd (ref: https://github.com/microsoft/mssql-docker/issues/892#issuecomment-2248045546)
function sqlcmd {
  /opt/mssql-tools/bin/sqlcmd -C -S localhost -U SA -P "${SA_PASSWORD}" "$@"
}

# wait up to 60s for the database to start
START_TIME="$(date +%s)"
while ! sqlcmd -Q 'SELECT 1'; do
  if [[ "$(( START_TIME + 60 ))" -lt "$(date +%s)" ]]; then
    echo >&2 'Could not connect to the database.'
    exit 1
  fi
done

# import the data
sqlcmd -i /static/chinook-sqlserver.sql || {
  echo >&2 'Failed to import the data.'
  exit 1
}

echo >&2 'Successfully imported the data.'
