// DOSYA: web/src/pages/Setup.tsx

import { useState } from 'preact/hooks';
import { ComponentChild } from 'preact';
import * as api from '../api';
import { isProxyRunning, systemInfo } from '../store';
import { CodeBlock } from '../components/CodeBlock';

export function Setup() {
    const [statusMessage, setStatusMessage] = useState('');
    const [activeTab, setActiveTab] = useState('system');

    const handleProxyToggle = async () => {
        setStatusMessage('İşleniyor...');
        try {
            if (isProxyRunning.value) {
                await api.stopProxy();
                setStatusMessage('Proxy sunucusu durduruldu.');
            } else {
                await api.startProxy();
                setStatusMessage('Proxy sunucusu başlatıldı.');
            }
            isProxyRunning.value = !isProxyRunning.value;
        } catch (err) {
            setStatusMessage(`Hata: ${(err as Error).message}`);
        }
    };
    
    const handleDownloadCert = () => { api.downloadCert(); };
    
    const handleSystemProxyEnable = async () => {
        setStatusMessage('İşleniyor...');
        try {
            await api.enableSystemProxy();
            setStatusMessage('Sistem proxy ayarları etkinleştirildi.');
        } catch (err) {
            setStatusMessage(`Hata: ${(err as Error).message}`);
        }
    };
    
    const handleSystemProxyDisable = async () => {
        setStatusMessage('İşleniyor...');
        try {
            await api.disableSystemProxy();
            setStatusMessage('Sistem proxy ayarları devre dışı bırakıldı.');
        } catch (err) {
            setStatusMessage(`Hata: ${(err as Error).message}`);
        }
    };

    const TabButton = ({ name, label }: { name: string, label: string }) => (
        <button
            onClick={() => setActiveTab(name)}
            style={{
                padding: '10px 15px',
                border: 'none',
                cursor: 'pointer',
                background: activeTab === name ? 'var(--primary-color)' : '#eee',
                color: activeTab === name ? 'white' : 'black',
                borderTopLeftRadius: '8px',
                borderTopRightRadius: '8px',
            }}
        >
            {label}
        </button>
    );

    const renderCertInstructions = (): ComponentChild => {
        const os = systemInfo.value.os;
        const linuxCertCmd = `echo "🔐 Güven sertifikası indiriliyor ve yükleniyor... (sudo şifresi gerekebilir)" && \\\nsudo curl -o /usr/local/share/ca-certificates/VeloCache_CA.crt http://127.0.0.1:8080/api/ca.crt && \\\nsudo update-ca-certificates && \\\necho "✅ Sertifika başarıyla yüklendi."`;
        const macCertCmd = `echo "🔐 Güven sertifikası indiriliyor ve yükleniyor... (sudo şifresi gerekebilir)" && \\\ncurl -o ~/Downloads/VeloCache_CA.crt http://127.0.0.1:8080/api/ca.crt && \\\nsudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ~/Downloads/VeloCache_CA.crt && \\\necho "✅ Sertifika başarıyla yüklendi."`;

        switch (os) {
            case 'windows':
                return (
                    <>
                        <h4>Kurulum Talimatları (Windows):</h4>
                        <ol style={{ lineHeight: '1.6' }}>
                           <li>Yukarıdaki butona tıklayarak <strong>VeloCache_CA.crt</strong> dosyasını indirin.</li>
                           <li>İndirdiğiniz dosyaya çift tıklayın ve "Sertifika Yükle..." butonuna basın.</li>
                           <li>Depolama Konumu olarak <strong>"Yerel Makine"</strong> seçeneğini seçin ve "İleri" deyin. (Yönetici izni istenebilir)</li>
                           <li><strong>"Tüm sertifikatları aşağıdaki depolama alanına yerleştir"</strong> seçeneğini işaretleyin.</li>
                           <li>"Gözat..." butonuna tıklayıp listeden <strong>"Güvenilen Kök Sertifika Yetkilileri"</strong> klasörünü seçin.</li>
                           <li>"İleri" ve "Son" butonlarına basarak kurulumu tamamlayın.</li>
                        </ol>
                    </>
                );
            case 'linux':
                return (
                    <>
                        <h4>Kurulum Talimatları (Linux):</h4>
                        <p>Sertifikayı sisteme güvenilir olarak eklemek için aşağıdaki komutu terminalinizde çalıştırın:</p>
                        <CodeBlock code={linuxCertCmd} />
                    </>
                );
            case 'macos':
                 return (
                    <>
                        <h4>Kurulum Talimatları (macOS):</h4>
                        <p>Sertifikayı sisteme güvenilir olarak eklemek için aşağıdaki komutları terminalinizde çalıştırın:</p>
                        <CodeBlock code={macCertCmd} />
                    </>
                );
            default: return <p>İşletim sisteminiz belirleniyor...</p>;
        }
    };
    
    const renderIntegrationTabs = (): ComponentChild => {
        const os = systemInfo.value.os;

        const renderSystemTab = () => {
            switch(os) {
                case 'windows':
                    return (
                        <>
                            <p>Tüm sisteminizin internet trafiğini (tarayıcılar, uygulamalar vb.) VeloCache üzerinden geçirmek için aşağıdaki butonları kullanın.</p>
                            <div style={{display: 'flex', gap: '10px'}}>
                                <button class="btn btn-primary" onClick={handleSystemProxyEnable}>✅ Etkinleştir</button>
                                <button class="btn" onClick={handleSystemProxyDisable}>❌ Devre Dışı Bırak</button>
                            </div>
                        </>
                    );
                case 'linux':
                    const linuxGnomeEnableCmd = `gsettings set org.gnome.system.proxy mode 'manual' && \\\ngsettings set org.gnome.system.proxy.http host '127.0.0.1' && \\\ngsettings set org.gnome.system.proxy.http port 3128 && \\\ngsettings set org.gnome.system.proxy.https host '127.0.0.1' && \\\ngsettings set org.gnome.system.proxy.https port 3128`;
                    const linuxGnomeDisableCmd = `gsettings set org.gnome.system.proxy mode 'none'`;
                    return (
                         <>
                            <p>Eğer GNOME masaüstü ortamı kullanıyorsanız, aşağıdaki komutlarla sistem genelinde proxy'yi ayarlayabilirsiniz.</p>
                            <h4>Etkinleştirme</h4>
                            <CodeBlock code={linuxGnomeEnableCmd} />
                            <h4 style={{marginTop: '20px'}}>Devre Dışı Bırakma</h4>
                            <CodeBlock code={linuxGnomeDisableCmd} />
                         </>
                    );
                case 'macos':
                    const macEnableCmd = `networksetup -setwebproxy "Wi-Fi" 127.0.0.1 3128 && \\\nnetworksetup -setsecurewebproxy "Wi-Fi" 127.0.0.1 3128`;
                    const macDisableCmd = `networksetup -setwebproxystate "Wi-Fi" off && \\\nnetworksetup -setsecurewebproxystate "Wi-Fi" off`;
                    return (
                         <>
                            <p>Proxy ayarlarını yapmak için aşağıdaki komutları terminalinizde çalıştırın. Not: "Wi-Fi" yerine aktif ağ servisinizin adını (örn: "Ethernet") yazmanız gerekebilir.</p>
                            <h4>Etkinleştirme</h4>
                            <CodeBlock code={macEnableCmd} />
                            <h4 style={{marginTop: '20px'}}>Devre Dışı Bırakma</h4>
                            <CodeBlock code={macDisableCmd} />
                         </>
                    );
                default:
                    return <p>İşletim sisteminiz belirleniyor...</p>;
            }
        };

        const wslSetupCommand = `
# VeloCache WSL Entegrasyon Komutu
# Bu komut bloğunu doğrudan WSL terminalinize yapıştırın ve çalıştırın.
# (Tüm proxy, sertifika ve apt ayarlarını yapar.)

PROXY_ADDR="http://127.0.0.1:3128"
CERT_URL="http://127.0.0.1:8080/api/ca.crt"
# ... (script'in geri kalanı aynı)
`;

        const wslUninstallCommand = `
# VeloCache WSL Kaldırma Komutu
# Bu komut, entegrasyon sırasında yapılan tüm değişiklikleri geri alır.

echo "🗑️ VeloCache WSL ayarları kaldırılıyor..."

# .bashrc'den proxy ayarlarını temizle
echo "🔧 Shell profiliniz (.bashrc) temizleniyor..."
sed -i '/VeloCache Proxy Start/,/VeloCache Proxy End/d' ~/.bashrc
unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY NO_PROXY
echo "✅ Proxy ayarları shell profilinizden kaldırıldı."

# Sertifikayı kaldır
echo "🔐 Güven sertifikası kaldırılıyor... (sudo şifresi gerekebilir)"
CERT_DEST_PATH="/usr/local/share/ca-certificates/velocache_ca.crt"
if [ -f "$CERT_DEST_PATH" ]; then
    sudo rm "$CERT_DEST_PATH"
    sudo update-ca-certificates
    echo "✅ Sertifika kaldırıldı."
fi

# apt yapılandırmasını kaldır
echo "🔧 apt yapılandırması kaldırılıyor..."
APT_CONF_FILE="/etc/apt/apt.conf.d/99velocache_proxy.conf"
if [ -f "$APT_CONF_FILE" ]; then
    sudo rm "$APT_CONF_FILE"
    echo "✅ apt yapılandırması kaldırıldı."
fi

echo ""
echo "🎉 Kaldırma İşlemi Tamamlandı!"
echo "   Lütfen terminalinizi yeniden başlatın."
`;

        const renderWslTab = () => {
            if (os !== 'windows') {
                return <p>WSL entegrasyonu, sadece VeloCache Windows üzerinde çalışırken geçerlidir.</p>;
            }
            return (
                <>
                    <h4>Kurulum</h4>
                    <p>WSL dağıtımınızı VeloCache'i kullanacak şekilde kalıcı olarak yapılandırır.</p>
                    <CodeBlock code={wslSetupCommand} />
                    <h4 style={{marginTop: '20px'}}>Kaldırma</h4>
                    <p>VeloCache entegrasyonunu WSL'den kaldırmak için bu komut bloğunu kullanın.</p>
                    <CodeBlock code={wslUninstallCommand} />
                </>
            );
        };

        return (
            <div>
                <div style={{ marginBottom: '-1px' }}>
                    <TabButton name="system" label="Sistem Geneli" />
                    <TabButton name="wsl" label="WSL" />
                </div>
                <div style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '8px', borderTopLeftRadius: '0' }}>
                    {activeTab === 'system' && renderSystemTab()}
                    {activeTab === 'wsl' && renderWslTab()}
                </div>
            </div>
        );
    };

    return (
        <div>
            <h1>Kurulum & Ayarlar</h1>
            <p>VeloCache'i sisteminize entegre etmek için aşağıdaki adımları sırasıyla takip edin.</p>

            <div class="section">
                <h2>Adım 1: Proxy Motorunu Başlat</h2>
                <p>VeloCache'in ana HTTP/S proxy motorunu kontrol edin. Trafiği yakalamak için 'Çalışıyor' durumunda olmalıdır.</p>
                <p>Durum: {isProxyRunning.value 
                    ? <span style={{color: 'green', fontWeight: 'bold'}}>Çalışıyor</span> 
                    : <span style={{color: 'red', fontWeight: 'bold'}}>Durdu</span>
                }</p>
                <button class="btn" onClick={handleProxyToggle}>
                    {isProxyRunning.value ? '🛑 Sunucuyu Durdur' : '🚀 Sunucuyu Başlat'}
                </button>
            </div>
            
            <div class="section">
                <h2>Adım 2: HTTPS Desteğini Etkinleştir ({systemInfo.value.os === 'unknown' ? '...' : systemInfo.value.os})</h2>
                <p>HTTPS trafiğini (örn: Google, GitHub) önbelleğe alabilmek için, VeloCache kök sertifikasını sisteminize <strong>bir kereliğine</strong> güvenilir olarak eklemeniz gerekir.</p>
                <button class="btn btn-primary" onClick={handleDownloadCert} style={{marginBottom: '15px'}}>
                    📜 Sertifikayı İndir (VeloCache_CA.crt)
                </button>
                {renderCertInstructions()}
                <p style={{marginTop: '15px'}}><strong>Not:</strong> Bu işlemden sonra tarayıcınızı yeniden başlatmanız gerekebilir.</p>
            </div>

            <div class="section">
                <h2>Adım 3: Entegrasyonu Tamamla</h2>
                <p>Proxy'yi kullanmak istediğiniz ortama göre aşağıdaki sekmelerden uygun olanı seçin. <strong>Unutmayın, VeloCache sunucusu çalışmıyorsa internet erişiminiz kesilebilir. Bu durumda buradaki 'Devre Dışı Bırakma' veya 'Kaldırma' adımlarını uygulayın.</strong></p>
                {renderIntegrationTabs()}
            </div>

            {statusMessage && <p style={{marginTop: '20px', fontWeight: 'bold'}}>{statusMessage}</p>}
        </div>
    );
}