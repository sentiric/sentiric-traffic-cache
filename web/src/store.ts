import { signal } from "@preact/signals";
import * as api from './api';
import type { CacheStats } from './api';

export const isConnected = signal(false);
export const stats = signal<CacheStats>({
  hits: 0, misses: 0, totalRequests: 0, diskItems: 0, totalDiskSizeBytes: 0,
});

function initializeStore() {
  api.fetchStats().then(data => { stats.value = data; }).catch(console.error);
  api.subscribeToEvents({
    onOpen: () => { isConnected.value = true; },
    onClose: () => { isConnected.value = false; },
    onStatsUpdated: (newStats) => { stats.value = newStats; },
  });
}

initializeStore();