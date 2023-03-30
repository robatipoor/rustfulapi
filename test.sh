#!/usr/bin/env bash
# By default the Rust test harness hides output from test execution to keep
# results readable. The nocapture flag disables that behavior.
./scripts/init_redis.sh
./scripts/init_mailhog.sh
./scripts/init_postgres.sh
./scripts/init_mockserver.sh
export APP_PROFILE=test
cargo test -- --nocapture --color=always
