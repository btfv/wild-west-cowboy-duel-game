FROM rust:1.95.0-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    && rustup target add wasm32-unknown-unknown \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY public ./public

ARG WS_URL
RUN WS_URL=${WS_URL} cargo build --target wasm32-unknown-unknown --release

FROM nginx:1.30.1-alpine-slim

COPY --from=builder /app/public /usr/share/nginx/html
COPY --from=builder /app/target/wasm32-unknown-unknown/release/game.wasm /usr/share/nginx/html/

EXPOSE 80
