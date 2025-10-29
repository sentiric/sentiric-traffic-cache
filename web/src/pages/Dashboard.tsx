import { signal } from '@preact/signals';
import { useEffect } from 'preact/hooks';
import { stats, isConnected } from '../store';
import * as api from '../api';
import type { CacheEntry } from '../api';

const entries = signal<CacheEntry[]>([]);

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

async function refreshEntries() {
  try {
    entries.value = await api.fetchEntries();
  } catch (error) {
    console.error("Failed to refresh entries:", error);
  }
}

async function handleClearCache() {
  if (confirm('Tüm önbelleği temizlemek istediğinizden emin misiniz? Bu işlem geri alınamaz.')) {
    try {
      await api.clearCache();
      await refreshEntries();
      alert('Önbellek başarıyla temizlendi.');
    } catch (error) {
      console.error("Failed to clear cache:", error);
      alert('Önbellek temizlenirken bir hata oluştu.');
    }
  }
}

export function Dashboard() {
  const s = stats.value;
  const hitRate = s.totalRequests > 0 ? ((s.hits / s.totalRequests) * 100).toFixed(1) + '%' : '0.0%';
  
  return (
    <div>
      <h1>Gösterge Paneli</h1>
      <div class="stats-grid">
        <StatCard title="Bağlantı" value={isConnected.value ? 'Aktif' : 'Kesildi'} color={isConnected.value ? '#4ade80' : '#f87171'} />
        <StatCard title="Tasarruf Edilen Veri" value={formatBytes(s.bytesSaved)} color="#34d399" /> {/* <-- YENİ KART */}
        <StatCard title="Hit Oranı" value={hitRate} />
        <StatCard title="Toplam İstek" value={s.totalRequests} />
        <StatCard title="Cache Boyutu" value={formatBytes(s.totalDiskSizeBytes)} />
        <StatCard title="Cache Girdileri" value={s.diskItems} />
      </div>

      <div class="section">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
          <h2>Önbellek Yönetimi</h2>
          <button class="btn" onClick={handleClearCache}>Önbelleği Temizle</button>
        </div>
        <table>
          <thead>
            <tr>
              <th>URL</th>
              <th>Boyut</th>
            </tr>
          </thead>
          <tbody>
            {entries.value.length === 0 ? (
              <tr>
                <td colSpan={2} style={{ textAlign: 'center', padding: '20px' }}>Önbellekte hiç girdi bulunamadı.</td>
              </tr>
            ) : (
              entries.value.map(entry => (
                <tr key={entry.key}>
                  <td class="url-cell" title={entry.key}>{entry.key}</td>
                  <td>{formatBytes(entry.sizeBytes)}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}