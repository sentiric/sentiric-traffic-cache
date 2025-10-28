# --- Adım 1: Builder ---
# Kodumuzu derlemek için Rust'ın resmi imajını kullanıyoruz.
# Dockerfile yazım kurallarına uygun olarak büyük harf kullanıyoruz.
FROM rust:1.78 AS builder

WORKDIR /app

# Sadece bağımlılık tanımlarını kopyala.
COPY Cargo.toml Cargo.lock ./
COPY crates crates

# Bağımlılıkları önceden derle. Bu, Docker katman önbelleğini en verimli
# şekilde kullanır. Sadece bağımlılıklar değiştiğinde bu adım yeniden çalışır.
# Önce boş bir src oluşturarak sadece bağımlılıkların derlenmesini sağlıyoruz.
RUN mkdir -p crates/cli/src && echo "fn main() {}" > crates/cli/src/main.rs
RUN cargo build --workspace --release

# Şimdi asıl kodumuzu kopyala. Sadece kod değiştiğinde bu adım çalışır.
COPY crates crates
# Önbelleği temizle ve sadece cli paketini yeniden derle. Bu, en güncel kodun
# kullanılmasını garanti eder ve daha hızlıdır.
RUN cargo clean -p sentiric-cli && cargo build -p sentiric-cli --release

# --- Adım 2: Runner ---
# Sonuç imajımız, çok daha küçük olan Debian "slim" tabanlı olacak.
FROM debian:bookworm-slim

# HTTPS bağlantıları için gerekli olan kök sertifikaları kur.
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Builder aşamasında derlediğimiz tek çalıştırılabilir dosyayı kopyala.
# Derlenen dosyanın tam yolunu belirtiyoruz.
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli

# Uygulamanın çalışması için gerekli olan config dosyasını kopyala.
COPY config.toml .

# Konteyner başladığında varsayılan olarak bu komutu çalıştır.
CMD ["sentiric-cli"]