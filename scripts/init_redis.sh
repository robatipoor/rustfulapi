#!/usr/bin/env bash

set -euxo pipefail

DB_USER="${REDIS_USER:=redis_user}"
DB_PASSWORD="${REDIS_PASSWORD:=''}"
DB_NAME="${REDIS_DB:=''}"
DB_PORT="${REDIS_PORT:=6379}"
RESTART_CONTAINER="${RESTART_CONTAINER:=false}"
RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')

function run_container() {
  docker run \
    -e REDIS_USER="${DB_USER}" \
    -e REDIS_PASSWORD="${DB_PASSWORD}" \
    -e DB_NAME="${DB_NAME}" \
    -p "${DB_PORT}":6379 \
    -d \
    --name "redis_$(date '+%s')" \
    redis:latest
}

if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a redis container already running"
  if ${RESTART_CONTAINER}; then
    echo >&2 "kill redis container"
    docker kill "${RUNNING_CONTAINER}"
    echo >&2 "start new redis container"
    run_container
  else
    echo >&2 "you can kill container with command :"
    echo >&2 "docker kill ${RUNNING_CONTAINER}"
  fi
else
  run_container
fi
