# RUSTfulapi
Reusable template for building REST Web Services in Rust. Uses [Actix-Web](https://actix.rs/) HTTP web framework and [SQLX](https://github.com/launchbadge/sqlx) Toolkit and [PostgreSQL](https://www.postgresql.org/).

![License](https://img.shields.io/github/license/robatipoor/rustfulapi)
![Lines of code](https://img.shields.io/tokei/lines/github/robatipoor/rustfulapi)
[![Format check](https://github.com/robatipoor/rustfulapi/actions/workflows/format.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/format.yml)
[![Build Check](https://github.com/robatipoor/rustfulapi/actions/workflows/check.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/check.yml)
[![Test](https://github.com/robatipoor/rustfulapi/actions/workflows/test.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/test.yml)
[![Clippy Check](https://github.com/robatipoor/rustfulapi/actions/workflows/clippy.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/clippy.yml)
[![Docker Image](https://github.com/robatipoor/rustfulapi/actions/workflows/build.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/build.yml)
[![Test Coverage](https://github.com/robatipoor/rustfulapi/actions/workflows/coverage.yml/badge.svg)](https://github.com/robatipoor/rustfulapi/actions/workflows/coverage.yml)
[![Codecov](https://codecov.io/gh/robatipoor/rustfulapi/branch/main/graph/badge.svg?token=BIMUKRJPE7)](https://codecov.io/gh/robatipoor/rustfulapi)
[![Dependency status](https://deps.rs/repo/github/robatipoor/rustfulapi/status.svg)](https://deps.rs/repo/github/robatipoor/rustfulapi)
![RUSTfulapi-logo](/static/images/logo.jpg)
### Requirements

- [rust](https://www.rust-lang.org/tools/install)
- [postgres](https://www.postgresql.org/)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
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
./run.sh
# open swagger panel
xdg-open http://127.0.0.1:8080/api/v1/swagger-ui/
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
```bash
export APP_PROFILE=prod # switch to production profile
```
### Update sqlx data json 
```bash
# https://crates.io/crates/sqlx-cli
cargo sqlx prepare --merged -- --all-features

```
### Check code formatting and typo at commit time
```
cp ./scripts/git-hooks ./.git/hooks/
```
### Update ERD (Entity-Relationship Diagram) use [planter](https://github.com/achiku/planter)
```bash
planter postgres://postgres_user:postgres_pass@localhost:5432/postgres_db\?sslmode=disable -o docs/schema.puml
```
## License

Licensed under either of

 * MIT license
   ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

Contributors are welcome! please fork and send pull requests, If you find a bug
or have any ideas on how to improve this project please submit an issue.

See [CONTRIBUTING.md](CONTRIBUTING.md).
