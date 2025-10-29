# --- Adım 1: Builder ---
FROM rust:latest AS builder
WORKDIR /app

# --- Frontend Derleme ---
# Önce sadece bağımlılık dosyalarını kopyala
COPY web/package.json web/package-lock.json* ./web/
# Node.js'i kur ve bağımlılıkları yükle
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    cd web && npm ci

# Sonra frontend kaynak kodunu kopyala ve derle
COPY web ./web
RUN cd web && npm run build

# --- Backend Derleme ---
# Backend bağımlılıklarını ve kaynak kodunu kopyala
COPY Cargo.toml Cargo.lock* ./
COPY crates crates

# Projeyi derle
RUN cargo build -p sentiric-cli --release

# --- Adım 2: Runner ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Derlenmiş binary'yi ve web dosyalarını kopyala
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY --from=builder /app/web/dist ./web/dist
COPY config.toml .

CMD ["sentiric-cli"]