import { invoke } from '@tauri-apps/api/tauri';

const Section = ({ title, description, children }: { title: string, description: string, children: any }) => (
    <div class="section">
        <div style={{ borderBottom: '1px solid #eee', paddingBottom: '15px', marginBottom: '15px' }}>
            <h2 style={{ margin: 0 }}>{title}</h2>
            <p style={{ margin: '5px 0 0', color: '#6c757d' }}>{description}</p>
        </div>
        <div>{children}</div>
    </div>
);

export function Settings() {

    const handleInstallCert = async () => {
        try {
            await invoke('install_ca_certificate');
            alert('Sertifika yükleme işlemi (simülasyon) başarıyla tetiklendi. Detaylar için konsolu kontrol edin.');
        } catch (error) {
            alert(`Bir hata oluştu: ${error}`);
        }
    };

    const handleEnableProxy = async () => {
        try {
            await invoke('enable_system_proxy');
            alert('Sistem proxy etkinleştirme işlemi (simülasyon) başarıyla tetiklendi.');
        } catch (error) {
            alert(`Bir hata oluştu: ${error}`);
        }
    };

    const handleDisableProxy = async () => {
        try {
            await invoke('disable_system_proxy');
            alert('Sistem proxy devre dışı bırakma işlemi (simülasyon) başarıyla tetiklendi.');
        } catch (error) {
            alert(`Bir hata oluştu: ${error}`);
        }
    };

    return (
        <div>
            <h1>Ayarlar</h1>
            <Section
                title="Sertifika Kurulumu"
                description="HTTPS trafiğini analiz edebilmek için Sentiric CA sertifikasını sisteminize güvenilir olarak ekleyin."
            >
                <button class="btn btn-primary" onClick={handleInstallCert}>Sertifikayı Yükle</button>
            </Section>

            <Section
                title="Sistem Proxy Ayarları"
                description="Tüm ağ trafiğini otomatik olarak Sentiric üzerinden geçirmek için sistem proxy'sini yapılandırın."
            >
                <div style={{ display: 'flex', gap: '10px' }}>
                    <button class="btn btn-primary" onClick={handleEnableProxy}>Proxy'yi Aktif Et</button>
                    <button class="btn" onClick={handleDisableProxy}>Proxy'yi Kapat</button>
                </div>
            </Section>
        </div>
    );
}