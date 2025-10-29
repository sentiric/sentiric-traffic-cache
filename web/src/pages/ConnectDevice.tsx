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

    const pacUrl = networkInfo ? `http://${networkInfo.ipAddress}:8080/proxy.pac` : '';

    return (
        <div>
            <h1>Başka Bir Cihaz Bağlayın</h1>

            {error && (
                <div class="setup-notice" style={{ backgroundColor: '#ffdddd', borderColor: '#ffb3b3' }}>
                    <h2 style={{color: '#dc3545'}}>IP Adresi Alınamadı</h2>
                    <p>Yerel ağ IP adresiniz alınırken bir hata oluştu. Lütfen ağ bağlantınızı kontrol edin.</p>
                    <pre style={{ background: '#f0f0f0', padding: '10px', borderRadius: '8px' }}>{error}</pre>
                </div>
            )}

            {!networkInfo && !error && <p>Yerel IP adresiniz alınıyor, lütfen bekleyin...</p>}

            {networkInfo && (
                <div>
                    <div class="section">
                        <h2 style={{ marginTop: 0 }}>Yöntem 1: Akıllı DNS (Tavsiye Edilen)</h2>
                        <p>
                            Bu yöntem, cihazınızdaki <strong>tüm uygulamaların</strong> internet trafiğini otomatik olarak Sentiric üzerinden geçirir.
                            Cihazınızın Wi-Fi ayarlarındaki DNS sunucusu adresini aşağıdaki IP adresi ile değiştirmeniz yeterlidir.
                        </p>
                        <h3>Kullanılacak DNS Sunucu Adresi:</h3>
                        <pre class="ip-box">{networkInfo.ipAddress}</pre>
                    </div>

                    <div class="section">
                        <h2 style={{ marginTop: 0 }}>Yöntem 2: Proxy Auto-Config (PAC)</h2>
                        <p>
                            Bu yöntem, genellikle sadece <strong>web tarayıcı trafiğini</strong> yönlendirir.
                            Cihazınızın Wi-Fi ayarlarındaki "HTTP Proxy" bölümünü "Otomatik" olarak ayarlayın ve aşağıdaki URL'yi girin.
                        </p>
                        <h3>Kullanılacak PAC URL'i:</h3>
                        <pre class="ip-box">{pacUrl}</pre>
                    </div>
                </div>
            )}
        </div>
    );
}