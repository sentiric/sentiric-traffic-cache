import { stats, isConnected } from '../store';

function formatBytes(bytes: number) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

const StatCard = ({ title, value, color }: { title: string; value: string | number; color?: string }) => (
  <div class="stat-card">
    <div class="stat-title">{title}</div>
    <div class="stat-value" style={{ color }}>{value}</div>
  </div>
);

export function Dashboard() {
  const s = stats.value;
  const hitRate = s.totalRequests > 0 ? ((s.hits / s.totalRequests) * 100).toFixed(1) + '%' : '0.0%';
  
  return (
    <div>
      <h1>Gösterge Paneli</h1>
      <div class="stats-grid">
        <StatCard title="Bağlantı" value={isConnected.value ? 'Aktif' : 'Kesildi'} color={isConnected.value ? '#4ade80' : '#f87171'} />
        <StatCard title="Hit Oranı" value={hitRate} />
        <StatCard title="Toplam İstek" value={s.totalRequests} />
        <StatCard title="Cache Boyutu" value={formatBytes(s.totalDiskSizeBytes)} />
        <StatCard title="Cache Girdileri" value={s.diskItems} />
      </div>
      {/* Canlı Loglar ve diğer bileşenler sonraki adımlarda eklenecek */}
    </div>
  );
}