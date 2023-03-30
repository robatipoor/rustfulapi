#!/usr/bin/env bash

./scripts/init_redis.sh
./scripts/init_mailhog.sh
./scripts/init_postgres.sh
./scripts/init_mockserver.sh
export APP_PROFILE=dev
cargo run
