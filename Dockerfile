# --- Adım 1: Builder ---
FROM rust:latest AS builder
WORKDIR /app

# DÜZELTME: Tauri bağımlılıklarını bu katmanın en başına taşıyoruz.
# Bu, hem 'build_and_test' hem de 'integration_test' için kullanılacak.
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libsoup2.4-dev \
    libjavascriptcoregtk-4.1-dev \
    && rm -rf /var/lib/apt/lists/*

# Frontend Derleme
COPY web ./web
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    cd web && npm install && npm run build

# Backend Derleme
COPY Cargo.toml Cargo.lock* ./
COPY crates crates
RUN cargo build -p sentiric-cli --release --features sentiric-service/web

# --- Adım 2: Runner ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY --from=builder /app/web/dist ./web/dist
COPY config.toml .

CMD ["sentiric-cli"]