// File: web/src/pages/Cache.tsx
// DÜZELTME: useContext kaldırıldı, doğrudan store'dan gelen sinyaller kullanılıyor.

import { useState, useMemo } from 'preact/hooks';
import { isProxyRunning, cacheEntries } from '../store';
import { deleteEntry, clearCache } from '../api';
import type { CacheEntry } from '../api'; // Tip tanımı için hala gerekli

function formatBytes(bytes: number, decimals = 2) { if (!bytes || bytes === 0) return '0 Bytes'; const k = 1024; const dm = decimals < 0 ? 0 : decimals; const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']; const i = Math.floor(Math.log(bytes) / Math.log(k)); return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`; }
function formatAge(dateString: string) { const now = new Date(); const then = new Date(dateString); const diffSeconds = Math.round((now.getTime() - then.getTime()) / 1000); if (isNaN(diffSeconds) || diffSeconds < 0) return 'az önce'; if (diffSeconds < 60) return `${diffSeconds} sn önce`; const diffMinutes = Math.round(diffSeconds / 60); if (diffMinutes < 60) return `${diffMinutes} dk önce`; const diffHours = Math.round(diffMinutes / 60); if (diffHours < 24) return `${diffHours} sa önce`; return `${Math.round(diffHours / 24)} gün önce`; }

export function Cache() {
    const [searchTerm, setSearchTerm] = useState('');
    
    const handleClearCache = async () => { if (confirm('Tüm önbelleği kalıcı olarak silmek istediğinizden emin misiniz?')) { try { await clearCache(); alert('Önbellek temizlendi.'); } catch (error) { alert(`Hata: ${(error as Error).message}`); } } };
    const handleDeleteEntry = async (hash: string) => { if (confirm(`'${hash.substring(0, 8)}...' karmalı girdiyi silmek istediğinizden emin misiniz?`)) { try { await deleteEntry(hash); } catch (error) { alert(`Hata: ${(error as Error).message}`); } } };
    
    const filteredEntries = useMemo(() => {
        if (!searchTerm) return cacheEntries.value;
        return cacheEntries.value.filter((entry: CacheEntry) => 
            entry.url.toLowerCase().includes(searchTerm.toLowerCase())
        );
    }, [searchTerm, cacheEntries.value]);

    return (
        <div>
            <h1>Önbellek Yönetimi</h1>
            <div class="controls" style={{ justifyContent: 'space-between', marginBottom: '20px', display: 'flex', alignItems: 'center' }}>
                <button class="btn" onClick={handleClearCache}>🗑️ Tüm Cache'i Temizle</button>
                <input type="text" placeholder="URL'de ara..." value={searchTerm} onInput={(e) => setSearchTerm((e.target as HTMLInputElement).value)} style={{ padding: '8px 12px', borderRadius: '8px', border: '1px solid #ccc', minWidth: '300px' }} />
            </div>
            <div class="section">
                <h2>Cache Girdileri ({filteredEntries.length})</h2>
                <div style={{ maxHeight: 'calc(100vh - 300px)', overflowY: 'auto' }}>
                  <table>
                    <thead><tr><th>URL</th><th>İçerik Tipi</th><th>Boyut</th><th>Yaş</th><th>İşlem</th></tr></thead>
                    <tbody>
                        {!isProxyRunning.value && cacheEntries.value.length === 0 && <tr><td colSpan={5} style={{ textAlign: 'center', padding: '20px' }}>Proxy sunucusu çalışmıyor. Lütfen <strong>Ayarlar</strong> sayfasından başlatın.</td></tr>}
                        {isProxyRunning.value && cacheEntries.value.length === 0 && (
                            <tr><td colSpan={5} style={{ textAlign: 'center', padding: '20px' }}>
                                {searchTerm ? 'Arama kriteriyle eşleşen girdi bulunamadı.' : 'Önbellek boş. Trafik bekleniyor...'}
                            </td></tr>
                        )}
                        {isProxyRunning.value && filteredEntries.map((entry: CacheEntry) => (
                        <tr key={entry.hash}>
                          <td class="url-cell" title={entry.url}>{entry.url}</td>
                          <td>{(entry.headers['content-type'] || 'N/A').split(';')[0]}</td>
                          <td>{formatBytes(entry.size)}</td>
                          <td>{formatAge(entry.createdAt)}</td>
                          <td><button class="btn btn-small" onClick={() => handleDeleteEntry(entry.hash)}>Sil</button></td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
            </div>
        </div>
    );
}