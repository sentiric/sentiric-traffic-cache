# --- Adım 1: Dependency Planner ---
# Bu ara katman, sadece Cargo.lock dosyasını oluşturmak için kullanılır.
FROM rust:1.78 AS planner
WORKDIR /app
COPY Cargo.toml ./
COPY crates crates
# Bu komut, Cargo.toml'a göre bağımlılıkları çözer ve Cargo.lock oluşturur.
# Ancak tam bir derleme yapmadığı için daha hızlıdır.
RUN cargo fetch

# --- Adım 2: Builder ---
# Bu katman, asıl derleme işlemini yapar.
FROM rust:1.78 AS builder
WORKDIR /app

# Planner'dan Cargo.lock dosyasını ve önceden indirilmiş bağımlılıkları kopyala.
# Bu, 'cargo build' komutunun ağa çıkmasını engeller ve hızı artırır.
COPY --from=planner /app/Cargo.lock ./
COPY --from=planner /usr/local/cargo/registry /usr/local/cargo/registry
COPY Cargo.toml ./
COPY crates crates

# Sadece cli paketini derle. Bu, Docker katman önbelleğini en iyi şekilde kullanır.
RUN cargo build -p sentiric-cli --release

# --- Adım 3: Runner ---
# Sonuç imajımız, çok daha küçük olan Debian "slim" tabanlı olacak.
FROM debian:bookworm-slim

# Gerekli runtime bağımlılıkları
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Builder aşamasında derlediğimiz tek çalıştırılabilir dosyayı kopyala.
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli

# Uygulamanın çalışması için gerekli olan config dosyasını kopyala.
COPY config.toml .

# Konteyner başladığında varsayılan olarak bu komutu çalıştır.
CMD ["sentiric-cli"]