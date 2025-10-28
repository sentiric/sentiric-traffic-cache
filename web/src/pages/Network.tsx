// File: web/src/pages/Network.tsx
import { isProxyRunning, networkRequests } from '../store';
import './Spinner.css';

function formatBytes(bytes: number, decimals = 2) { if (bytes === undefined || bytes === null || bytes === 0) return '0 Bytes'; const k = 1024; const dm = decimals < 0 ? 0 : decimals; const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']; const i = Math.floor(Math.log(bytes) / Math.log(k)); return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`; }
const Spinner = () => <div class="spinner"></div>;

export function Network() {
    const requestList = Array.from(networkRequests.value.values()).sort((a, b) => b.begin.id - a.begin.id);
    const handleRowClick = (id: number) => { console.log("Request details for ID:", id, networkRequests.value.get(id)); };

    return (
        <div>
            <h1>Ağ Akışı</h1>
            <div class="section">
                <div style={{ maxHeight: 'calc(100vh - 200px)', overflowY: 'auto' }}>
                    <table>
                        <thead>
                            <tr><th>Metot</th><th>URL</th><th>Durum</th><th>Boyut</th><th>Süre</th><th>Kaynak</th></tr>
                        </thead>
                        <tbody>
                            {!isProxyRunning.value && <tr><td colSpan={6} style={{ textAlign: 'center', padding: '20px' }}>Proxy sunucusu çalışmıyor. Lütfen <strong>Ayarlar</strong> sayfasından başlatın.</td></tr>}
                            {isProxyRunning.value && requestList.length === 0 && (
                                <tr><td colSpan={6} style={{ textAlign: 'center', padding: '20px' }}>
                                    <div style={{display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '10px'}}>
                                        <Spinner /><span>Trafik bekleniyor...</span>
                                    </div>
                                </td></tr>
                            )}
                            {isProxyRunning.value && requestList.map(({ begin, end }) => (
                                <tr key={begin.id} onClick={() => handleRowClick(begin.id)} style={{cursor: 'pointer'}}>
                                    <td><strong>{begin.method}</strong></td>
                                    <td class="url-cell" title={begin.uri}>{begin.uri}</td>
                                    <td style={{ color: end && end.statusCode >= 400 ? '#dc3545' : end && end.statusCode >= 300 ? '#fbbf24' : 'inherit', fontWeight: 'bold' }}>{end ? end.statusCode : '...'}</td>
                                    <td>{end ? formatBytes(end.size) : '...'}</td>
                                    <td>{end ? `${end.durationMs} ms` : '...'}</td>
                                    <td>{end ? (end.isFromCache ? <span style={{ color: '#005a9c', fontWeight: 'bold' }}>Cache</span> : 'Ağ') : '...'}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
}