# 📖 Geliştirme Kılavuzu (Development Guide)
# Sürüm: 1.0

Bu belge, bu projeye katkıda bulunacak **herkes** için (insan veya yapay zeka) bir yol göstericidir. Projenin tutarlılığını, kalitesini ve vizyonunu korumak için bir "anayasadır". Her görevden önce bu kılavuza başvurulmalıdır.

---

## 1. 🎯 Temel Felsefe ve Kurallar

*   **Ana Vizyon:** "Görünmez Ağ Asistanı". Kullanıcı için kurulumu ve kullanımı zahmetsiz, arka planda akıllıca çalışan evrensel bir ağ katmanı oluşturmak.
*   **Öncelik Sırası:** 1. Kullanıcı Deneyimi (Basitlik) 2. Güvenilirlik (Testler) 3. Performans.
*   **Dil Kuralları:**
    *   **Dokümantasyon (`.md`):** Türkçe
    *   **Kod, Commit Mesajları, CI/CD:** İngilizce

---

## 2. 🏛️ Teknik Mimari ve Standartlar

*   **Mimari Model:** "Merkezi Çekirek + Çoklu Arayüz" (Cargo Workspace).
    *   `crates/core`: Saf iş mantığı, veri yapıları. **Asla I/O veya framework bağımlılığı içermez.**
    *   `crates/service`: Framework'ler (Hyper, Warp, Tokio), I/O işlemleri ve asıl servislerin (Proxy, DNS, API) implementasyonu.
    *   `crates/cli` / `crates/companion`: `service` katmanını başlatan, platforma özel çalıştırıcılar.
*   **Temel Prensip:** Her yeni özellik, bu modüler yapıya uygun olarak doğru krat (crate) içine eklenmelidir. Kod temizliğine ve tek sorumluluk prensibine her zaman uyulmalıdır.

---

## 3. 🛠️ Geliştirme ve Test Süreci (DevOps Felsefesi)

*   **Nihai Gerçeklik Kaynağı:** **CI Pipeline'ıdır.** Yerel testler sadece bir kolaylıktır. Bir özelliğin "tamamlandığı" an, `ci.yml`'deki tüm adımların (Backend, Frontend, Entegrasyon) yeşil olduğu andır.
*   **Test Önceliği:** Her yeni özellik veya hata düzeltmesi, bunu doğrulayan bir **otomatik test** ile birlikte gelmelidir. `tests/test-runner.sh` script'i, yeni E2E (uçtan uca) test senaryolarıyla sürekli zenginleştirilmelidir.
*   **Versiyonlama ve Yayınlama:** Sürümler, `git tag vx.y.z` ile manuel ve bilinçli olarak oluşturulur. Her `tag`, `release.yml` workflow'unu tetikler ve indirilebilir bir ürün (binary'ler ve Docker imajı) oluşturur.

---

## 4. 📚 Harici Bağımlılıklar ve Referans Kodlar

Bu bölüm, projenin kritik ve sık değişen bağımlılıkları için bir referans noktasıdır. **Bu bölümü güncel tutmak, projenin sağlığı için en yüksek önceliktir.**

### `trust-dns-server`
*   **Kullanılan Versiyon:** `~0.23`
*   **Anahtar Kavram:** Projemizin amacı "gelen her sorguyu yakalayıp özel bir cevap vermek" olduğu için, karmaşık `Authority` trait'i yerine, çok daha basit olan **`RequestHandler`** trait'i doğrudan implemente edilir.
*   **Referans Kod (`dns.rs`'ten - v0.23 için DOĞRU kullanım):**
    ```rust
    // RequestHandler'ın doğru kullanımı:
    #[async_trait]
    impl RequestHandler for DnsHandler {
        async fn handle_request<H: ResponseHandler>(
            &self,
            request: &Request,
            mut response_handle: H,
        ) -> ResponseInfo {
            // ... message oluşturma ve send_response çağırma
        }
    }
    ```

### `rust-embed`
*   **Kullanılan Versiyon:** Yok (Kullanımdan Kaldırıldı).
*   **Anahtar Kavram:** Derleme zamanı karmaşıklığını ve CI sorunlarını önlemek için standart bir dosya sunma yaklaşımı benimsenmiştir.
*   **Referans Kod:**
    *   **Dockerfile:** Frontend, `npm run build` ile `web/dist` klasörüne derlenir ve bu klasör son imaja kopyalanır.
    *   **management.rs:** `warp::fs::dir` kullanılarak bu klasör doğrudan diskten sunulur.

---