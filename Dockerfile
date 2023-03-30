# build stage
FROM rust:latest as builder
WORKDIR /workspace
RUN apt-get update && apt-get install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# deploy stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends openssl ca-certificates && apt-get clean
# create workspace directory
WORKDIR /workspace

COPY static static
COPY settings settings

# copy binary and configuration files
COPY --from=builder /workspace/target/release/app .

# expose port
EXPOSE 8080
ENV APP_PROFILE prod
ENV RUST_LOG info
# run the binary
ENTRYPOINT ["./app"]
