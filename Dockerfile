# --- Adım 1: Builder ---
# Rust'ın resmi imajını kullanarak kodumuzu derleyeceğiz.
FROM rust:1.78 as builder

# Gerekli build araçları
RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app

# Önce sadece bağımlılıkları kopyalayıp derle.
# Bu sayede kod değişmediği sürece Docker bu katmanı cache'den kullanır,
# her seferinde tüm bağımlılıkları indirmek zorunda kalmayız.
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./crates ./crates
# Boş bir main.rs oluşturarak bağımlılıkları derlemeye zorluyoruz.
RUN mkdir -p ./crates/cli/src && echo "fn main() {}" > ./crates/cli/src/main.rs
RUN cargo build --release -p sentiric-cli --jobs $(nproc)

# Şimdi asıl kodumuzu kopyala ve tekrar derle.
# Sadece kod değiştiğinde bu adım çalışır.
COPY ./crates/cli/src ./crates/cli/src
RUN cargo build --release -p sentiric-cli --jobs $(nproc)


# --- Adım 2: Runner ---
# Sonuç imajımız çok daha küçük olan Debian "slim" tabanlı olacak.
FROM debian:bookworm-slim

# Sertifika yetkililerini kur (HTTPS bağlantıları için gerekli)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Builder aşamasında derlediğimiz tek çalıştırılabilir dosyayı kopyala.
COPY --from=builder /app/target/release/sentiric-cli /usr/local/bin/sentiric-cli

# Uygulamanın çalışması için gerekli olan config dosyasını kopyala.
COPY ./config.toml /app/config.toml

# Varsayılan olarak bu komutu çalıştır.
CMD ["sentiric-cli"]