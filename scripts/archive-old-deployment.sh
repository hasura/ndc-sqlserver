#!/usr/bin/env bash
set -e -u -o pipefail

# before we generate a new deployment, save the current one in
# `/static/deployment-snapshots` so we can ensure they can all be read over
# time

CHINOOK_DEPLOYMENT="$1"

CURRENT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" > /dev/null && echo "$PWD")"

SNAPSHOT_DIR="$(realpath ${CURRENT_DIR}/../static/deployment-snapshots)"

# create snapshot dir if does not exist
mkdir -p "$SNAPSHOT_DIR"

if [ -f ${CHINOOK_DEPLOYMENT}new_configuration/configuration.json ]; then
    # create filename from hash of contents
    NEW_DIRECTORY="$(sha256sum "${CHINOOK_DEPLOYMENT}new_configuration/configuration.json" | cut -f1 -d' ')"

    mkdir -p "${SNAPSHOT_DIR}/${NEW_DIRECTORY}"

    # copy current deployment to new filename
    cp -r "${CHINOOK_DEPLOYMENT}new_configuration"* "${SNAPSHOT_DIR}/${NEW_DIRECTORY}"

    rm -r "${CHINOOK_DEPLOYMENT}new_configuration"
else
    # create filename from hash of contents
    NEW_DIRECTORY="$(sha256sum "${CHINOOK_DEPLOYMENT}configuration.json" | cut -f1 -d' ')"

    mkdir -p "${SNAPSHOT_DIR}/${NEW_DIRECTORY}"

    # copy current deployment to new filename
    cp -r "${CHINOOK_DEPLOYMENT}configuration.json"* "${SNAPSHOT_DIR}/${NEW_DIRECTORY}"
fi

