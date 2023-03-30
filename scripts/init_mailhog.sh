#!/usr/bin/env bash

set -euxo pipefail

RESTART_CONTAINER="${RESTART_CONTAINER:=false}"
RUNNING_CONTAINER=$(docker ps --filter 'name=mailhog' --format '{{.ID}}')

function run_container() {
  docker run -p 1025:1025 -p 8025:8025 -d --name "mailhog_$(date '+%s')" mailhog/mailhog:latest
}

if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a mailhog container already running"
  if ${RESTART_CONTAINER}; then
    echo >&2 "kill mailhog container"
    docker kill "${RUNNING_CONTAINER}"
    echo >&2 "start new mailhog container"
    run_container
  else
    echo >&2 "you can kill container with command :"
    echo >&2 "docker kill ${RUNNING_CONTAINER}"
  fi
else
  run_container
fi
