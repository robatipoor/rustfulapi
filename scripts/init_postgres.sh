#!/usr/bin/env bash

set -euxo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.6.2 sqlx-cli --no-default-features --features postgres,native-tls"
  echo >&2 "to install it."
  exit 1
fi

DB_USER="${POSTGRES_USER:=postgres_user}"
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres_pass}"
DB_NAME="${POSTGRES_DB:=postgres_db}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"
RESTART_CONTAINER="${RESTART_CONTAINER:=false}"

RUNNING_CONTAINER=$(docker ps --filter 'name=postgres' --format '{{.ID}}')

function run_container() {
  docker run \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_NAME}" \
    -p "${DB_PORT}":5432 \
    -d \
    --name "postgres_$(date '+%s')" \
    postgres -N 1000
}

if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a postgres container already running"
  if ${RESTART_CONTAINER}; then
    echo >&2 "kill postgres container"
    docker kill "${RUNNING_CONTAINER}"
    echo >&2 "start new postgres container"
    run_container
  else
    echo >&2 "you can kill container with command :"
    echo >&2 "docker kill ${RUNNING_CONTAINER}"
  fi
else
  run_container
fi

until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run
