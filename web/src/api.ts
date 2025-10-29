export interface CacheStats {
  hits: number;
  misses: number;
  totalRequests: number;
  diskItems: number;
  totalDiskSizeBytes: number;
  bytesSaved: number; // <-- YENİ
}

export interface CacheEntry {
  key: string;
  sizeBytes: number;
}

export type WsEvent =
  | { type: 'statsUpdated'; stats: CacheStats };

const API_BASE = '/api';

export async function fetchStats(): Promise<CacheStats> {
  const response = await fetch(`${API_BASE}/stats`);
  if (!response.ok) throw new Error('Failed to fetch stats');
  return response.json();
}

export async function fetchEntries(): Promise<CacheEntry[]> {
  const response = await fetch(`${API_BASE}/entries`);
  if (!response.ok) throw new Error('Failed to fetch entries');
  return response.json();
}

export async function clearCache(): Promise<Response> {
  const response = await fetch(`${API_BASE}/clear`, { method: 'POST' });
  if (!response.ok) throw new Error('Failed to clear cache');
  return response;
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
  const reconnectingCallbacks = {
    ...callbacks,
    onClose: () => {
      callbacks.onClose?.();
      socket = null;
      setTimeout(() => connect(reconnectingCallbacks), 3000);
    }
  };

  if (!socket) {
    connect(reconnectingCallbacks);
  }
}