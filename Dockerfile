# syntax=docker/dockerfile:1

# chef
FROM rust:1.85.1-slim-bullseye AS chef 
RUN cargo install cargo-chef 
WORKDIR /app

# planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# common build
FROM chef AS builder
# Add libpq-dev for PostgreSQL development files
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# build app
COPY . .
RUN cargo build --release

# runtime - server
FROM debian:bullseye-slim AS server
RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/
ENTRYPOINT ["server"]

# runtime - client
FROM debian:bullseye-slim AS client
COPY --from=builder /app/target/release/client /usr/local/bin/
ENTRYPOINT ["client"]
