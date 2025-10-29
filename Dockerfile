# --- Adım 1: Builder ---
FROM rust:latest AS builder
WORKDIR /app

# Frontend Derleme
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs
COPY web ./web
RUN cd web && npm install && npm run build

# Backend Derleme
COPY Cargo.toml Cargo.lock* ./
COPY crates crates
# DÜZELTME: Artık 'features' flag'ine ihtiyacımız yok.
RUN cargo build -p sentiric-cli --release

# --- Adım 2: Runner ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Gerekli dosyaları kopyala
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY --from=builder /app/web/dist ./web/dist # <--- KRİTİK DEĞİŞİKLİK
COPY config.toml .

CMD ["sentiric-cli"]