#!/usr/bin/env bash
./scripts/init_db.sh
./scripts/init_redis.sh
./scripts/init_mailhog.sh
./scripts/init_mockserver.sh
export $(cat .env | xargs)
cargo run --bin app
