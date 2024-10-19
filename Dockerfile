FROM rust:slim-bookworm AS builder
RUN apt-get update && apt-get install -y clang gcc curl pkg-config openssl libssl3 libssl-dev ca-certificates make

WORKDIR /work

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y openssl libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

RUN mkdir /app/data /app/config
COPY  --from=builder /work/target/release/countryside .
COPY  --from=builder /work/config.example.toml /app/config/config.toml
ENTRYPOINT ["/app/countryside"]
