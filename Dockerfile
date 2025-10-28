# --- Adım 1: Builder ---
# Kodumuzu derlemek için Rust'ın resmi imajını kullanıyoruz.
FROM rust:1.78 as builder

WORKDIR /app

# Sadece bağımlılık tanımlarını kopyala. Bu, Docker'ın katman önbelleğini
# en verimli şekilde kullanmasını sağlar. Sadece bağımlılıklar değiştiğinde
# bu adımlar yeniden çalışır.
COPY Cargo.toml Cargo.lock ./
COPY crates crates

# Bağımlılıkları önceden derle.
RUN cargo build --workspace --release

# --- Adım 2: Runner ---
# Sonuç imajımız, çok daha küçük olan Debian "slim" tabanlı olacak.
FROM debian:bookworm-slim

# HTTPS bağlantıları için gerekli olan kök sertifikaları kur.
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Builder aşamasında derlediğimiz tek çalıştırılabilir dosyayı kopyala.
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli

# Uygulamanın çalışması için gerekli olan config dosyasını kopyala.
COPY config.toml .

# Konteyner başladığında varsayılan olarak bu komutu çalıştır.
CMD ["sentiric-cli"]