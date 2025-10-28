// File: web/src/pages/Dashboard.tsx
import { isProxyRunning, stats, logs } from '../store';

function formatBytes(bytes: number, decimals = 2) { if (!bytes || bytes === 0) return '0 Bytes'; const k = 1024; const dm = decimals < 0 ? 0 : decimals; const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']; const i = Math.floor(Math.log(bytes) / Math.log(k)); return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`; }
const StatCard = ({ title, value, color }: { title: string; value: string | number; color?: string }) => ( <div class="stat-card"> <div class="stat-title">{title}</div> <div class="stat-value" style={{ color: color, transition: 'color 0.5s ease' }}>{value}</div> </div> );

export function Dashboard() {
  const hitRateValue = stats.value.totalRequests > 0 ? (stats.value.hits / stats.value.totalRequests) * 100 : 0;
  const hitRateDisplay = hitRateValue.toFixed(1) + '%';

  const getHitRateColor = () => {
    if (!isProxyRunning.value || stats.value.totalRequests === 0) return undefined;
    if (hitRateValue > 80) return '#4ade80';
    if (hitRateValue > 50) return '#fbbf24';
    return '#f87171';
  };

  return (
    <div>
      <h1>Gösterge Paneli</h1>
      <div class="stats-grid">
        <StatCard title="Durum" value={isProxyRunning.value ? 'Aktif' : 'Durdu'} color={isProxyRunning.value ? '#4ade80' : '#f87171'} />
        <StatCard title="Hit Oranı" value={hitRateDisplay} color={getHitRateColor()} />
        <StatCard title="Toplam İstek" value={stats.value.totalRequests} />
        <StatCard title="Disk Cache Boyutu" value={formatBytes(stats.value.totalDiskSizeBytes)} />
        <StatCard title="Kazanç (Cache'den)" value={formatBytes(stats.value.dataServedFromCacheBytes)} />
      </div>
      <div class="section">
        <h2>Canlı Loglar</h2>
        <pre id="log-output">{[...logs.value].reverse().join('\n')}</pre>
      </div>
    </div>
  );
}