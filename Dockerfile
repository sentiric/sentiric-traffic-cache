# --- Adım 1: Builder ---
FROM rust:latest AS builder
WORKDIR /app

# --- Frontend Derleme ---
# Önce sadece bağımlılık dosyalarını kopyala
COPY web/package.json web/package-lock.json* ./web/
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    cd web && npm ci

# Sonra frontend kaynak kodunu kopyala ve derle
COPY web ./web
RUN cd web && npm run build

# --- Backend Derleme ---
# Önce sadece bağımlılık dosyalarını kopyala
COPY Cargo.toml Cargo.lock* ./
# Boş bir lib.rs dosyası ile sahte bir proje oluşturarak sadece bağımlılıkları derle
RUN mkdir -p crates/cli/src && echo "fn main() {}" > crates/cli/src/main.rs
RUN cargo build -p sentiric-cli --release

# Sonra backend kaynak kodunu kopyala ve tam derlemeyi yap
COPY crates crates
RUN cargo build -p sentiric-cli --release

# --- Adım 2: Runner ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli
COPY --from=builder /app/web/dist ./web/dist
COPY config.toml .

CMD ["sentiric-cli"]