// Sadece backend'de VAR OLAN özellikleri tanımlıyoruz.
export interface CacheStats {
  hits: number;
  misses: number;
  totalRequests: number;
  diskItems: number;
  totalDiskSizeBytes: number;
}

export type WsEvent =
  | { type: 'statsUpdated'; stats: CacheStats };

const API_BASE = '/api';

export async function fetchStats(): Promise<CacheStats> {
  const response = await fetch(`${API_BASE}/stats`);
  if (!response.ok) throw new Error('Failed to fetch stats');
  return response.json();
}

interface EventStreamCallbacks {
  onStatsUpdated?: (stats: CacheStats) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

let socket: WebSocket | null = null;

function connect(callbacks: EventStreamCallbacks) {
  const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = `${wsProtocol}//${window.location.host}${API_BASE}/events`;
  socket = new WebSocket(wsUrl);

  socket.onopen = callbacks.onOpen ?? null;
  // DÜZELTME: Eğer callbacks.onClose tanımsız ise, null ata.
  socket.onclose = callbacks.onClose ?? null;
  
  socket.onmessage = (event) => {
    try {
      const parsedEvent = JSON.parse(event.data) as WsEvent;
      if (parsedEvent.type === 'statsUpdated') {
        callbacks.onStatsUpdated?.(parsedEvent.stats);
      }
    } catch (e) { console.error("Failed to parse event:", e); }
  };

  socket.onerror = (error) => {
    console.error("WebSocket error:", error);
    socket?.close();
  };
}

export function subscribeToEvents(callbacks: EventStreamCallbacks) {
  // Yeniden bağlanma mantığını basitleştiriyoruz.
  const reconnectingCallbacks = {
    ...callbacks,
    onClose: () => {
      callbacks.onClose?.();
      socket = null; // Soketi temizle
      setTimeout(() => connect(reconnectingCallbacks), 3000); // 3 saniye sonra aynı callback'lerle tekrar bağlan
    }
  };

  if (!socket) {
    connect(reconnectingCallbacks);
  }
}