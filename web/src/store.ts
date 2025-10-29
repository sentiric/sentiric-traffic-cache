import { signal } from "@preact/signals";
import * as api from './api';
import type { CacheStats, FlowEntry } from './api';

export const isConnected = signal(false);
export const stats = signal<CacheStats>({
  hits: 0, misses: 0, totalRequests: 0, diskItems: 0, totalDiskSizeBytes: 0, bytesSaved: 0,
});
export const flows = signal<FlowEntry[]>([]); // YENİ SİNYAL

const MAX_FLOWS = 100; // Ekranda en fazla kaç akış tutulacağı

function initializeStore() {
  api.fetchStats().then(data => { stats.value = data; }).catch(console.error);
  api.subscribeToEvents({
    onOpen: () => { isConnected.value = true; },
    onClose: () => { isConnected.value = false; },
    onStatsUpdated: (newStats) => { stats.value = newStats; },
    onFlowUpdated: (newFlow) => {
      // Yeni akışı listenin başına ekle
      const updatedFlows = [newFlow, ...flows.value];
      // Listeyi belirli bir boyutta tut
      if (updatedFlows.length > MAX_FLOWS) {
        updatedFlows.pop();
      }
      flows.value = updatedFlows;
    },
  });
}

initializeStore();