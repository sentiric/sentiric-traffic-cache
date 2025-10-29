import { flows } from '../store';

function formatBytes(bytes: number) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

const StatusBadge = ({ code }: { code: number }) => {
    let color = '#6c757d'; // Gray
    if (code >= 200 && code < 300) color = '#28a745'; // Green
    if (code >= 400 && code < 500) color = '#ffc107'; // Yellow
    if (code >= 500) color = '#dc3545'; // Red
    
    return <span style={{ color, fontWeight: 'bold' }}>{code}</span>;
}

const HitBadge = ({ isHit }: { isHit: boolean }) => (
    <span style={{ 
        background: isHit ? '#d4edda' : '#f8d7da',
        color: isHit ? '#155724' : '#721c24',
        padding: '2px 6px',
        borderRadius: '4px',
        fontSize: '0.8rem',
        fontWeight: 'bold',
    }}>
        {isHit ? 'HIT' : 'MISS'}
    </span>
)

export function NetworkFlow() {
  return (
    <div>
        <h1>Ağ Akışı İnceleyici</h1>
        <div class="section">
            <p style={{marginTop: 0, color: '#6c757d'}}>Sisteminizden geçen HTTP/S isteklerini burada gerçek zamanlı olarak izleyebilirsiniz.</p>
            <div style={{ maxHeight: '75vh', overflowY: 'auto' }}>
                <table>
                    <thead style={{ position: 'sticky', top: 0, background: 'white' }}>
                        <tr>
                            <th>Durum</th>
                            <th>Metot</th>
                            <th>URL</th>
                            <th>Boyut</th>
                            <th>Önbellek</th>
                        </tr>
                    </thead>
                    <tbody>
                        {flows.value.length === 0 ? (
                        <tr>
                            <td colSpan={5} style={{ textAlign: 'center', padding: '40px' }}>Henüz bir trafik algılanmadı...</td>
                        </tr>
                        ) : (
                        flows.value.map(flow => (
                            <tr key={flow.id}>
                            <td><StatusBadge code={flow.statusCode} /></td>
                            <td>{flow.method}</td>
                            <td class="url-cell" title={flow.uri}>{flow.uri}</td>
                            <td>{formatBytes(flow.responseSizeBytes)}</td>
                            <td><HitBadge isHit={flow.isHit} /></td>
                            </tr>
                        ))
                        )}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
  );
}