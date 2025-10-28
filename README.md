# 🚀 sentiric-traffic-cache

[![Continuous Integration](https://github.com/sentiric/sentiric-traffic-cache/actions/workflows/ci.yml/badge.svg)](https://github.com/sentiric/sentiric-traffic-cache/actions/workflows/ci.yml)

**Görünmez Ağ Asistanı.** Geliştirme iş akışlarını hızlandırmak ve ağ yönetimini basitleştirmek için tasarlanmış akıllı, evrensel bir önbellekleme katmanı.

## 🏃‍♂️ Hızlı Başlangıç

### Docker (Tavsiye Edilen)
En güncel Docker imajı her zaman GitHub Container Registry'de mevcuttur:
```bash
docker pull ghcr.io/sentiric/sentiric-traffic-cache:latest
```

### Çalıştırılabilir Dosyalar
Linux ve Windows için önceden derlenmiş çalıştırılabilir dosyalara [**Sürümler (Releases) Sayfasından**](https://github.com/sentiric/sentiric-traffic-cache/releases) ulaşabilirsiniz.

## 💻 Geliştirme Ortamı

Bu proje, tutarlı ve tekrarlanabilir bir ortam sağlamak amacıyla Docker içinde derlenip çalıştırılacak şekilde tasarlanmıştır.

### Ön Gereksinimler

-   [Docker](https://www.docker.com/products/docker-desktop/)
-   Docker Compose

### Uygulamayı Çalıştırma

1.  Repo'yu klonlayın:
    ```bash
    git clone https://github.com/sentiric/sentiric-traffic-cache.git
    cd sentiric-traffic-cache
    ```

2.  Uygulamayı Docker Compose ile başlatın:
    ```bash
    docker compose up --build
    ```

> **Önemli:** Projeye katkıda bulunmadan önce, lütfen geliştirme felsefemizi ve standartlarımızı özetleyen [**Geliştirme Kılavuzu'nu**](docs/DEVELOPMENT_GUIDE.md) okuyun.

---