# sentiric-traffic-cache: Geliştirme Görevleri

Bu belge, projenin yol haritasını ve tamamlanma durumunu takip eder.

## Milestone 0: Proje Anayasası ve Fırlatma Rampası
- [x] Proje vizyonu, mimarisi ve yol haritası tanımlandı.
- [x] Kapsamlı dökümantasyon temeli (`SPECIFICATION`, `ARCHITECTURE`, `TASKS`) oluşturuldu.
- [x] Proje iskeleti (Cargo Workspace) oluşturuldu.
- [x] Temel CI/CD pipeline'ı (Docker build) kuruldu.

## Milestone 1: Minimum Değerli Ürün (MVP)
### Sprint 1: Çekirdek Proxy ve Cache
- [x] `sentiric-service`: Temel HTTP/S MitM proxy motorunu geliştir.
- [x] `sentiric-service`: Sertifika üretme (`rcgen`) ve yönetme mantığını ekle.
- [x] `sentiric-service`: Diske akıtarak yazan (streaming) hibrit cache mekanizmasını geliştir.
- [x] `sentiric-cli`: Servisi başlatan temel çalıştırıcıyı tamamla.
- [x] `tests`: Temel HTTP ve HTTPS önbellekleme senaryoları için entegrasyon testleri ekle.

### Sprint 2: Yönetim Arayüzü ve Gözlemlenebilirlik
- [x] `sentiric-service`: Yönetim API'si için `Warp` sunucusunu entegre et.
- [ ] `sentiric-service`: Temel REST endpoint'lerini (`/api/stats`, `/api/entries`) geliştir.
- [x] `sentiric-service`: WebSocket (`/api/events`) yayın mekanizmasını kur.
- [x] `web`: Mevcut Preact projesini `web/` dizinine taşı ve yeni API'ye bağla.
- [x] `sentiric-service`: Warp sunucusunun web arayüzünü sunmasını sağla.

## Milestone 2: Zahmetsiz Deneyim
- [ ] `sentiric-companion`: Tauri projesini oluştur ve `service`'i entegre et.
- [ ] `sentiric-companion`: Tek tıkla sertifika kurulumunu (Windows, macOS, Linux) geliştir.
- [ ] `sentiric-companion`: Tek tıkla sistem proxy ayarlarını (Windows, macOS, Linux) geliştir.
- [ ] `sentiric-companion`: Sistem tepsisi menüsünü ekle.

## Milestone 3: Ağ Geneli "Sıfır Yapılandırma"
- [ ] `sentiric-service`: Akıllı DNS sunucusunu (transparent proxy için) geliştir.
- [ ] `web`: "Başka Cihaz Bağla" arayüzünü DNS odaklı olarak yeniden tasarla.
- [ ] `sentiric-service`: Proxy Auto-Config (`/proxy.pac`) endpoint'ini ekle.

## Milestone 4: Ürünleşme
- [ ] `packaging`: MSI, DMG ve DEB kurulum paketleri için script'ler oluştur.
- [ ] `web`: Gösterge Paneline "Kazanç" metrikleri ekle.
- [ ] `web`: Ağ Akışı İnceleyici özelliğini geliştir.
- [ ] `sentiric-service`: Gelişmiş kural motorunu (`rules.toml`) geliştir.