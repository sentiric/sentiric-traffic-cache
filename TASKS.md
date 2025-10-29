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

## Milestone 2: Zahmetsiz Deneyim
- [x] `sentiric-companion`: Tauri projesini oluştur ve `service`'i entegre et.
- [x] `sentiric-companion`: Tek tıkla sertifika kurulumu için altyapı (Tauri komutları ve UI) eklendi.
- [x] `sentiric-companion`: Tek tıkla sistem proxy ayarları için altyapı (Tauri komutları ve UI) eklendi.
- [x] `sentiric-companion`: Sistem tepsisi menüsünü ekle.

## Milestone 3: Ağ Geneli "Sıfır Yapılandırma"
- [x] `sentiric-service`: Akıllı DNS sunucusunu (transparent proxy için) geliştir.
- [x] `web`: "Başka Cihaz Bağla" arayüzünü DNS odaklı olarak yeniden tasarla.
- [x] `sentiric-service`: Proxy Auto-Config (`/proxy.pac`) endpoint'ini ekle.

## Milestone 4: Ürünleşme
- [x] `packaging`: MSI, DMG ve DEB kurulum paketleri için script'ler oluştur.
- [x] `web`: Gösterge Paneline "Kazanç" metrikleri ekle.
- [x] `web`: Ağ Akışı İnceleyici özelliğini geliştir.
- [x] `sentiric-service`: Gelişmiş kural motorunu (`rules.toml`) geliştir.