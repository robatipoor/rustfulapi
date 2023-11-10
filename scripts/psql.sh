#!/usr/bin/env bash

set -euxo pipefail

DATABASE="postgres"
DB_USER="${DATABASE_USER:=username}"
DB_PASSWORD="${DATABASE_PASSWORD:=password}"
DB_NAME="${DATABASE_NAME:=database_name}"
DB_PORT="${DATABASE_PORT:=5432}"
DB_HOST="${DATABASE_HOST:=localhost}"
RUNNING_CONTAINER=$(docker ps --filter "name=$DATABASE" --format '{{.Names}}')
CONTAINER_NAME="${RUNNING_CONTAINER:-${DATABASE}_container}"

docker exec -it $CONTAINER_NAME psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}"
