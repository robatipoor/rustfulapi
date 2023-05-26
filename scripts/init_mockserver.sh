#!/usr/bin/env bash

set -euxo pipefail

PORT="${MOCK_SERVER_PORT:=8026}"
CONFIG_PATH="${MOCK_SERVER_CONFIG_PATH:=/expections/init.json}"
SCRIPT_PATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)"
RESTART_CONTAINER="${RESTART_CONTAINER:=false}"
RUNNING_CONTAINER=$(docker ps --filter 'name=mockserver' --format '{{.ID}}')

function run_container() {
  docker run -v "${SCRIPT_PATH}"/mockserver-exceptions:/exceptions \
    -e MOCKSERVER_LOG_LEVEL=DEBUG \
    -e MOCKSERVER_INITIALIZATION_JSON_PATH="$CONFIG_PATH" \
    -e SERVER_PORT="${PORT}" \
    -p "${PORT}":"${PORT}" \
    -d \
    --name "mockserver_$(date '+%s')" \
    mockserver/mockserver:latest
}

if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a mockserver container already running"
  if ${RESTART_CONTAINER}; then
    echo >&2 "kill mockserver container"
    docker kill "${RUNNING_CONTAINER}"
    echo >&2 "start new redis container"
    run_container
  else
    echo >&2 "update exceptions"
    curl -X PUT -H "Content-Type: application/json" \
    -d @"${SCRIPT_PATH}"/mockserver-expections/init.json \
    localhost:"${PORT}"/mockserver/expectation
    echo >&2 "you can kill container with command :"
    echo >&2 "docker kill ${RUNNING_CONTAINER}"
  fi
else
  run_container
fi
