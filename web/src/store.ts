// Bu dosya, uygulamanın global "state"ini (durumunu) yönetir.
import { signal } from "@preact/signals";
import * as api from './api';
import type { CacheStats } from './api';

// Sinyaller: Bu değerler değiştiğinde, onları kullanan tüm bileşenler otomatik olarak güncellenir.
export const isConnected = signal(false);
export const stats = signal<CacheStats>({
  hits: 0,
  misses: 0,
  totalRequests: 0,
  diskItems: 0,
  totalDiskSizeBytes: 0,
});

function initializeStore() {
  // Sayfa ilk yüklendiğinde anlık istatistikleri çek
  api.fetchStats()
    .then(data => {
      stats.value = data;
    })
    .catch(console.error);

  // Canlı güncellemeler için WebSocket'e abone ol
  api.subscribeToEvents({
    onOpen: () => {
      isConnected.value = true;
    },
    onClose: () => {
      isConnected.value = false;
    },
    onStatsUpdated: (newStats) => {
      stats.value = newStats;
    },
  });
}

// Uygulama başlarken store'u başlat
initializeStore();