# --- Adım 1: Dependency Planner ---
# DÜZELTME: Rust sürümünü en güncel kararlı sürüme yükseltiyoruz.
FROM rust:stable AS planner 
WORKDIR /app
COPY Cargo.toml ./
COPY crates crates
RUN cargo fetch

# --- Adım 2: Builder ---
# DÜZELTME: Rust sürümünü en güncel kararlı sürüme yükseltiyoruz.
FROM rust:stable AS builder
WORKDIR /app

COPY --from=planner /app/Cargo.lock ./
COPY --from=planner /usr/local/cargo/registry /usr/local/cargo/registry
COPY Cargo.toml ./
COPY crates crates

# Sadece cli paketini derle.
RUN cargo build -p sentiric-cli --release

# --- Adım 3: Runner ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY config.toml .

CMD ["sentiric-cli"]