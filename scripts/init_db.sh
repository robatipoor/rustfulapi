#!/usr/bin/env bash

set -euxo pipefail

DATABASE="postgres"
DB_USER="${DATABASE_USER:=username}"
DB_PASSWORD="${DATABASE_PASSWORD:=password}"
DB_NAME="${DATABASE_NAME:=database_name}"
DB_PORT="${DATABASE_PORT:=5432}"
DB_HOST="${DATABASE_HOST:=localhost}"
CONTAINER_NAME="${DATABASE}_container"
RESTART_CONTAINER="${RESTART_CONTAINER:=false}"
RUNNING_CONTAINER=$(docker ps --filter "name=$DATABASE" --format '{{.Names}}')
CONTAINER_NAME="${RUNNING_CONTAINER:-${DATABASE}_container}"


function run_container() {
  docker run \
    --rm \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_NAME}" \
    -p "${DB_PORT}":5432 \
    -d \
    --name "$CONTAINER_NAME" \
    ${DATABASE} \
    -N 1000 # maximum number of allowed connections
}

if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a database container $RUNNING_CONTAINER already running"
  if ${RESTART_CONTAINER}; then
    echo >&2 "kill database container"
    docker kill "${RUNNING_CONTAINER}"
    sleep 2
    echo >&2 "start new database container"
    run_container
  else
    echo >&2 "you can kill container with command :"
    echo >&2 "docker kill ${RUNNING_CONTAINER}"
  fi
else
  run_container
fi

export PGPASSWORD="${DB_PASSWORD}"
until docker exec $CONTAINER_NAME psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
  echo >&2 "Database is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Database is up and running on port ${DB_PORT} - running migrations now!"

DATABASE_URL=${DATABASE}://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

cargo run --bin migration -- refresh -u $DATABASE_URL
