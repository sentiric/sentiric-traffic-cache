# sentiric-traffic-cache: Teknik Şartname v1.0

## 1. Vizyon ve Temel Prensipler

-   **Vizyon:** Ağ trafiğini akıllıca yöneten, hızlandıran ve basitleştiren, kullanıcı için "görünmez" bir ağ asistanı olmak.
-   **Prensipler:** Modülerlik, Test Edilebilirlik, Kullanıcı Odaklılık, Şeffaflık.

## 2. Kullanıcı Personaları

-   **Geliştirici (DevOps/Backend/Frontend):** Temiz ve tekrarlanabilir ortamlar için sık sık `docker prune`, `apt-get update` gibi komutlar çalıştıran, zaman ve bant genişliği kaybını en aza indirmek isteyen kullanıcı.
-   **Son Kullanıcı (Ev/Ofis):** Teknik bilgisi az olan, ağındaki tüm cihazların (PC, mobil, TV) daha hızlı çalışmasını ve daha az veri tüketmesini isteyen kullanıcı.

## 3. Özellik Seti (Nihai Ürün)

### 3.1. Çekirdek Servisler
-   **[ ] Evrensel HTTP/S Önbellekleme:** HTTP/1.1 ve HTTP/2 trafiği için şeffaf MitM (Man-in-the-Middle) önbellekleme.
-   **[ ] Akıllı DNS Sunucusu:** Ağdaki cihazları yapılandırma gerektirmeden proxy'ye yönlendiren, kural tabanlı DNS çözümlemesi.
-   **[ ] Gelişmiş Kural Motoru:** `rules.toml` dosyası üzerinden host, URL deseni ve içerik tipine göre trafiği engelleme, atlama veya TTL değiştirme.

### 3.2. Yönetim ve Arayüz
-   **[ ] Gömülü Web Arayüzü:** Tüm yönetim ve gözlemleme işlemlerinin yapıldığı, reaktif, modern web paneli.
-   **[ ] Gerçek Zamanlı Gösterge Paneli:** Anlık istatistikler, Hit Oranı, Tasarruf Edilen Veri (Kazanç) ve Kurtarılan Zaman metrikleri.
-   **[ ] Ağ Akışı İnceleyici:** Sistemden geçen her isteğin detaylarını (başlıklar, zamanlama, içerik) inceleme aracı.

### 3.3. Companion App (Masaüstü Yardımcısı)
-   **[ ] Tek Tıkla Otomatik Kurulum:** Sertifika, sistem proxy'si ve açılışta başlama ayarlarını tek bir butonla, kullanıcı adına otomatik yapma.
-   **[ ] Sistem Tepsisi Entegrasyonu:** Uygulamanın arka planda çalışması ve temel kontrollere (Başlat/Durdur) hızlı erişim.
-   **[ ] Tutarlı Deneyim:** Ayrı bir UI olmadan, `core`'un sunduğu web arayüzünü bir webview içinde gösterme.

## 4. API Endpointleri (Özet)
-   `GET /api/stats`: Anlık istatistikleri döndürür.
-   `GET /api/entries`: Önbellek girdilerini listeler.
-   `POST /api/clear`: Tüm önbelleği temizler.
-   `POST /api/proxy/start | /stop`: Proxy servisini yönetir.
-   `POST /api/dns/start | /stop`: DNS servisini yönetir.
-   `GET /proxy.pac`: Otomatik proxy yapılandırma dosyası sunar.