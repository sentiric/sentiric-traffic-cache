// Bu dosya, backend API'mızın tüm endpoint'lerini ve veri yapılarını tanımlar.

// `sentiric-core/src/lib.rs` içindeki Stats yapısıyla eşleşir
export interface CacheStats {
  hits: number;
  misses: number;
  totalRequests: number;
  diskItems: number;
  totalDiskSizeBytes: number;
}

// `sentiric-service/src/management.rs` içindeki WsEvent enum'ıyla eşleşir
export type WsEvent =
  | { type: 'statsUpdated'; stats: CacheStats };

const API_BASE = '/api';

export async function fetchStats(): Promise<CacheStats> {
  const response = await fetch(`${API_BASE}/stats`);
  if (!response.ok) {
    throw new Error('Failed to fetch stats');
  }
  return response.json();
}

// WebSocket bağlantısını ve olay dinlemeyi yöneten kısım
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

  socket.onopen = () => {
    console.log("WebSocket connected.");
    callbacks.onOpen?.();
  };

  socket.onmessage = (event) => {
    try {
      const parsedEvent = JSON.parse(event.data) as WsEvent;
      if (parsedEvent.type === 'statsUpdated') {
        callbacks.onStatsUpdated?.(parsedEvent.stats);
      }
    } catch (e) {
      console.error("Failed to parse WebSocket event:", e);
    }
  };

  socket.onclose = () => {
    console.log("WebSocket disconnected. Reconnecting in 3 seconds...");
    callbacks.onClose?.();
    socket = null;
    setTimeout(() => connect(callbacks), 3000);
  };

  socket.onerror = (error) => {
    console.error("WebSocket error:", error);
    socket?.close();
  };
}

export function subscribeToEvents(callbacks: EventStreamCallbacks) {
  if (!socket) {
    connect(callbacks);
  }
  // Zaten bağlıysa, sadece callback'leri güncelle (bu örnekte basit tutulmuştur)
}