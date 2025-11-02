# syntax=docker/dockerfile:1
FROM rust:1.91-slim-bullseye as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev
COPY . .
RUN cargo build --release --bin nof1_cli

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/nof1_cli /usr/local/bin/nof1_cli
ENTRYPOINT ["nof1_cli"]
