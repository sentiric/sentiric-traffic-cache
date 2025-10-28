// DOSYA: web/src/store.ts

import { signal } from "@preact/signals";
import * as api from './api';
// DEĞİŞİKLİK: SystemInfo tipini de import et
import type { CacheStats, CacheEntry, RequestBegin, RequestEnd, SystemInfo } from './api';

export const isProxyRunning = signal(false);
export const stats = signal<CacheStats>({ hits: 0, misses: 0, totalRequests: 0, inMemoryItems: 0, diskItems: 0, totalDiskSizeBytes: 0, dataServedFromCacheBytes: 0 });
export const logs = signal<string[]>([]);
export const networkRequests = signal<Map<number, { begin: RequestBegin; end?: RequestEnd }>>(new Map());
export const cacheEntries = signal<CacheEntry[]>([]);
// YENİ: Sistem bilgisi için signal
export const systemInfo = signal<SystemInfo>({ os: 'unknown' });

function debounce(func: () => void, delay: number) {
    let timeout: any;
    return () => {
        clearTimeout(timeout);
        timeout = setTimeout(func, delay);
    };
}

const fetchCacheEntries = () => {
    api.fetchEntries().then(data => {
        data.sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime());
        cacheEntries.value = data;
    }).catch(console.error);
};

const debouncedFetchCacheEntries = debounce(fetchCacheEntries, 250);

function initializeStore() {
    api.fetchProxyStatus().then(data => (isProxyRunning.value = data.running)).catch(console.error);
    api.fetchStats().then(data => (stats.value = { ...data })).catch(console.error);
    fetchCacheEntries();
    
    // YENİ: Başlangıçta sistem bilgisini çek
    api.fetchSystemInfo().then(data => (systemInfo.value = data)).catch(console.error);

    api.subscribeToEvents({
      onLog: (newMessage) => {
        logs.value = [newMessage, ...logs.value.slice(0, 99)];
      },
      
      onRequestBegin: (event) => {
        const newMap = new Map(networkRequests.value);
        newMap.set(event.id, { begin: event });
        if (newMap.size > 200) {
          const oldestKey = Array.from(newMap.keys()).sort((a, b) => a - b)[0];
          newMap.delete(oldestKey);
        }
        networkRequests.value = newMap;
      },

      onRequestEnd: (event) => {
        const newMap = new Map(networkRequests.value);
        const entry = newMap.get(event.id);
        if (entry) {
          newMap.set(event.id, { ...entry, end: event });
          networkRequests.value = newMap;
        }
      },

      onStatsUpdated: (newStats) => {
        stats.value = { ...newStats };
      },
      
      onDataChanged: debouncedFetchCacheEntries,
    });

    setInterval(() => {
        api.fetchProxyStatus().then(data => (isProxyRunning.value = data.running)).catch(console.error);
    }, 5000);
}

initializeStore();