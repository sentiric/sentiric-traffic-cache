# --- Adım 1: Dependency Planner ---
FROM rust:latest AS planner 
WORKDIR /app
COPY Cargo.toml ./
COPY crates crates
RUN cargo fetch

# --- Adım 2: Builder ---
FROM rust:latest AS builder
WORKDIR /app

# --- Frontend Derleme Adımları ---
# Node.js'i kur
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs
# Web dosyalarını kopyala
COPY web ./web
# Bağımlılıkları kur ve arayüzü derle
RUN cd web && npm install && npm run build

# --- Backend Derleme Adımları ---
COPY --from=planner /app/Cargo.lock ./
COPY --from=planner /usr/local/cargo/registry /usr/local/cargo/registry
COPY Cargo.toml ./
COPY crates crates
# Sadece cli paketini derle ve 'web' feature'ını aktive et.
RUN cargo build -p sentiric-cli --release --features sentiric-service/web
# --- Adım 3: Runner ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY config.toml .
# ÖNEMLİ: Derlenmiş web arayüzünü kopyalamıyoruz, çünkü artık binary'nin içinde gömülü!
CMD ["sentiric-cli"]