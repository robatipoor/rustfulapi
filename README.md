# 🦀 RUSTfulapi
Reusable template for building REST Web Services in Rust. Uses [Axum](https://github.com/tokio-rs/axum) HTTP web framework and [SeaORM](https://github.com/SeaQL/sea-orm) ORM and [PostgreSQL](https://www.postgresql.org/).

![License](https://img.shields.io/github/license/robatipoor/rustfulapi)

[![Lines Of Code](https://tokei.rs/b1/github/robatipoor/rustfulapi?category=lines)](https://github.com/robatipoor/rustfulapi)
[![Format check](https://github.com/robatipoor/rustfulapi/actions/workflows/code-style-checker.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/code-style-checker.yml)
[![Build Check](https://github.com/robatipoor/rustfulapi/actions/workflows/build-checker.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/build-checker.yml)
[![Test](https://github.com/robatipoor/rustfulapi/actions/workflows/test.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/test.yml)
[![Clippy Check](https://github.com/robatipoor/rustfulapi/actions/workflows/code-linter.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/code-linter.yml)
[![Docker Image](https://github.com/robatipoor/rustfulapi/actions/workflows/image-builder.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/image-builder.yml)
[![Test Coverage](https://github.com/robatipoor/rustfulapi/actions/workflows/test-coverage.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/test-coverage.yml)
[![Codecov](https://codecov.io/gh/robatipoor/rustfulapi/branch/main/graph/badge.svg?token=BIMUKRJPE7)](https://codecov.io/gh/robatipoor/rustfulapi)
[![Dependency status](https://deps.rs/repo/github/robatipoor/rustfulapi/status.svg)](https://deps.rs/repo/github/robatipoor/rustfulapi)
![RUSTfulapi-logo](/static/images/_logo.jpg)
### Requirements

- [rust](https://www.rust-lang.org/tools/install)
- [postgres](https://www.postgresql.org/)
- [docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

### How to use this template

To use this template as your project starting point, click "Use this template" at the top of this page, or click [here](https://github.com/robatipoor/rustfulapi/generate).

### Feature highlights

* Authentication. Based on [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
* Layered configuration. Based on [config-rs](https://github.com/mehcode/config-rs)
* Logs. Based on [tracing](https://github.com/tokio-rs/tracing)
* OpenAPI documentation [utoipa](https://github.com/juhaku/utoipa)
* Error handling
* Pagination
* Profile base 
* E2E Tests
* Postgres admin [pgAdmin](https://www.pgadmin.org/)
* CI based on Github actions
* Sentry error tracking
* Nginx as reverse proxy and secure connections with SSL certificates [Nginx](https://www.nginx.com/)
* Dependabot configuration

### Running locally

```bash
./run
# open swagger panel
xdg-open http://127.0.0.1:8080/swagger-ui/
# manually testing your API routes with curl commands
curl -X GET http://127.0.0.1:8080/api/v1/server/health_check
```
### Running via docker

```bash
cd ./docker/dev/ && ./up.sh
```
### Running tests
Some of the integration tests use Docker to spin up dependencies on demand (e.g., a postgres db),so please ensure Docker is installed before running the tests.
```
./test.sh
```
![RUSTfulapi grid](https://codecov.io/gh/robatipoor/rustfulapi/branch/main/graphs/tree.svg?token=BIMUKRJPE7)
### Configuration
This project uses [config-rs](https://github.com/mehcode/config-rs) to manage configuration.
#### Configure with toml files
```bash
settings
├── base.toml # default config file 
├── dev.toml # development config file 
├── prod.toml # production config file
└── test.toml # test config file

```
#### Configure with environment variables
```bash
export APP_SERVER__PORT=8080
export APP_SERVER__ADDR=127.0.0.1
```
#### Switching profiles
Before running the application, export this variable:
```bash
export APP_PROFILE=prod # Switch to production profile
```
### Check code formatting and typo at commit time
```
cp ./scripts/git-hooks/* ./.git/hooks/
```
### Migrate database
```
cargo run --bin migration -- up -u $DATABASE_URL
```
### Update ERD (Entity-Relationship Diagram) use [planter](https://github.com/achiku/planter)
```bash
planter postgres://username:password@localhost:5432/database_name\?sslmode=disable -o docs/schema.puml
```

## Run tests
```sh
# Execute all test projects.
./test
```

## License

Licensed under either of

 * MIT license
   ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

Contributors are welcome! please fork and send pull requests, If you find a bug
or have any ideas on how to improve this project please submit an issue.

See [CONTRIBUTING.md](CONTRIBUTING.md).
