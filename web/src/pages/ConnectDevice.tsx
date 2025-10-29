
import { useState, useEffect } from 'preact/hooks';
import { invoke } from '@tauri-apps/api/tauri';

interface NetworkInfo {
  ipAddress: string;
}

export function ConnectDevice() {
    const [networkInfo, setNetworkInfo] = useState<NetworkInfo | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        invoke<NetworkInfo>('get_network_info')
            .then(info => setNetworkInfo(info))
            .catch(err => setError(err.toString()));
    }, []);

    return (
        <div>
            <h1>Başka Bir Cihaz Bağlayın</h1>
            <div class="section">
                <h2 style={{ marginTop: 0 }}>Akıllı DNS ile Otomatik Kurulum</h2>
                <p>
                    Ağınızdaki diğer cihazların (akıllı telefon, tablet, TV vb.) Sentiric'in önbelleğinden faydalanması için
                    cihazınızın Wi-Fi ayarlarındaki DNS sunucusu adresini aşağıda belirtilen IP adresi ile değiştirmeniz yeterlidir.
                </p>
                <p>
                    Bu işlem, cihazınızdaki tüm uygulamaların internet trafiğini otomatik olarak Sentiric üzerinden geçirecektir.
                    <strong>Hiçbir proxy ayarı yapmanıza gerek yoktur.</strong>
                </p>

                {error && (
                    <div class="setup-notice" style={{ backgroundColor: '#ffdddd', borderColor: '#ffb3b3' }}>
                        <h2 style={{color: '#dc3545'}}>IP Adresi Alınamadı</h2>
                        <p>Yerel ağ IP adresiniz alınırken bir hata oluştu. Lütfen ağ bağlantınızı kontrol edin.</p>
                        <pre style={{ background: '#f0f0f0', padding: '10px', borderRadius: '8px' }}>{error}</pre>
                    </div>
                )}

                {networkInfo ? (
                    <div>
                        <h3>Kullanılacak DNS Sunucu Adresi:</h3>
                        <pre style={{ fontSize: '1.5rem', background: '#e6f0ff', padding: '20px', borderRadius: 'var(--border-radius)', textAlign: 'center', letterSpacing: '2px' }}>
                            {networkInfo.ipAddress}
                        </pre>
                        <small>Not: DNS Portu varsayılan olarak 53'tür.</small>
                    </div>
                ) : !error && (
                    <p>Yerel IP adresiniz alınıyor, lütfen bekleyin...</p>
                )}
            </div>
        </div>
    );
}