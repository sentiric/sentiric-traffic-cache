export interface CacheStats {
  hits: number;
  misses: number;
  totalRequests: number;
  diskItems: number;
  totalDiskSizeBytes: number;
  bytesSaved: number;
}

export interface CacheEntry {
  key: string;
  sizeBytes: number;
}

export interface FlowEntry {
  id: string;
  method: string;
  uri: string;
  statusCode: number;
  responseSizeBytes: number;
  isHit: boolean;
}

export type Action = 'Allow' | 'Block' | 'BypassCache';

// API'den gelen `url-pattern` ile eşleşmesi için.
export type RuleCondition = { domain: string } | { "url-pattern": string };

export interface Rule {
  name: string;
  condition: RuleCondition;
  action: Action;
}

export type WsEvent =
  | { type: 'statsUpdated'; stats: CacheStats }
  | { type: 'flowUpdated'; flow: FlowEntry };

// --- NİHAİ DÜZELTME: ADRESLERİ HER ZAMAN MUTLAK OLARAK TANIMLA ---
// Backend'imiz her zaman 8080 portunda çalışır. Bu değişmez bir kuraldır.
const API_BASE_URL = 'http://localhost:8080/api';
const WS_BASE_URL = 'ws://localhost:8080/api';

export async function fetchStats(): Promise<CacheStats> {
  const response = await fetch(`${API_BASE_URL}/stats`);
  if (!response.ok) throw new Error('Failed to fetch stats');
  return response.json();
}

export async function fetchEntries(): Promise<CacheEntry[]> {
  const response = await fetch(`${API_BASE_URL}/entries`);
  if (!response.ok) throw new Error('Failed to fetch entries');
  return response.json();
}

export async function clearCache(): Promise<Response> {
  const response = await fetch(`${API_BASE_URL}/clear`, { method: 'POST' });
  if (!response.ok) throw new Error('Failed to clear cache');
  return response;
}

export async function fetchRules(): Promise<Rule[]> {
  const response = await fetch(`${API_BASE_URL}/rules`);
  if (!response.ok) throw new Error('Failed to fetch rules');
  return response.json();
}

interface EventStreamCallbacks {
  onStatsUpdated?: (stats: CacheStats) => void;
  onFlowUpdated?: (flow: FlowEntry) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

let socket: WebSocket | null = null;

function connect(callbacks: EventStreamCallbacks) {
  const wsUrl = `${WS_BASE_URL}/events`;
  socket = new WebSocket(wsUrl);

  socket.onopen = callbacks.onOpen ?? null;
  socket.onclose = callbacks.onClose ?? null;
  
  socket.onmessage = (event) => {
    try {
      const parsedEvent = JSON.parse(event.data) as WsEvent;
      // Gelen JSON'daki anahtar isimlerini kontrol etmek için loglayalım
      // console.log("Received Event:", parsedEvent); 
      if (parsedEvent.type === 'statsUpdated') {
        callbacks.onStatsUpdated?.(parsedEvent.stats);
      } else if (parsedEvent.type === 'flowUpdated') {
        callbacks.onFlowUpdated?.(parsedEvent.flow);
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