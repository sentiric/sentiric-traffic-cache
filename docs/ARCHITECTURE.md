# sentiric-traffic-cache: Sistem Mimarisi v1.0

## 1. Yüksek Seviye Mimari

Proje, "Merkezi Çekirdek + Çoklu Arayüz" felsefesine sahip bir Cargo Workspace olarak tasarlanmıştır. Tüm iş mantığı, IO ve framework'lerden bağımsız `core` ve bu mantığı hayata geçiren `service` katmanında toplanmıştır. `cli` ve `companion` ise bu servisi farklı ortamlar için başlatan "çalıştırıcılardır".

```mermaid
graph TD
    subgraph Kullanıcı Arayüzleri
        A[💻 Companion App]
        B[🐳 CLI / Docker]
    end

    subgraph Servis Katmanı (sentiric-service)
        direction LR
        S1[🚀 Proxy Motoru (Hyper)]
        S2[🧠 DNS Sunucusu (Trust-DNS)]
        S3[📊 Yönetim API (Warp)]
    end

    C[📚 Çekirdek Mantık (sentiric-core)]

    A -- Başlatır --> S3
    B -- Başlatır --> S1 & S2 & S3

    S1 & S2 & S3 -- Kullanır --> C
```

## 2. Bileşenlerin Sorumlulukları

-   **`crates/core`:** Projenin beynidir. Veri yapılarını (`CacheEntry`, `Stats`), paylaşılan mantığı ve trait'leri içerir. Hiçbir harici IO (dosya yazma, ağ bağlantısı) veya framework (Hyper, Warp) içermez. %100 unit test edilebilir olmalıdır.
-   **`crates/service`:** Projenin motorudur. `core`'dan aldığı yapıları kullanarak asıl işi yapar. Proxy, DNS, Cache ve API servislerini barındırır. Framework'ler ve IO operasyonları burada yaşar.
-   **`crates/cli`:** Sunucu ve Docker ortamları için "başsız" (headless) çalıştırıcıdır. Tek görevi `service` katmanını başlatmaktır.
-   **`crates/companion`:** Son kullanıcılar için masaüstü uygulamasıdır. `service` katmanını bir arka plan görevi olarak başlatır ve işletim sistemiyle (sertifika, proxy ayarları) entegrasyonu sağlar.

## 3. Teknoloji Stack'i
-   **Backend:** Rust, Tokio, Hyper (Proxy), Warp (API), Trust-DNS (DNS), Rustls (TLS)
-   **Frontend:** Preact, Vite, TypeScript
-   **Masaüstü:** Tauri
-   **CI/CD:** GitHub Actions

