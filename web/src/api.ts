// DOSYA: web/src/api.ts

export interface CacheStats { hits: number; misses: number; totalRequests: number; inMemoryItems: number; diskItems: number; totalDiskSizeBytes: number; dataServedFromCacheBytes: number; }
export interface CacheEntry { headers: Record<string, string>; createdAt: string; url: string; size: number; hash: string; }
export interface RequestBegin { id: number; timestamp: string; method: string; uri: string; }
export interface RequestEnd { id: number; timestamp: string; statusCode: number; size: number; isFromCache: boolean; durationMs: number; }

// YENİ: Sistem bilgisi için tip tanımı
export interface SystemInfo { os: 'windows' | 'macos' | 'linux' | 'unknown'; }

export type WsEvent =
    | { type: 'log'; payload: string }
    | { type: 'requestBegin'; event: RequestBegin }
    | { type: 'requestEnd'; event: RequestEnd }
    | { type: 'statsUpdated'; stats: CacheStats }
    | { type: 'dataChanged' };

const API_BASE = '/api';

export async function fetchStats(): Promise<CacheStats> { const response = await fetch(`${API_BASE}/stats`); if (!response.ok) throw new Error('Failed to fetch stats'); return response.json(); }
export async function fetchEntries(): Promise<CacheEntry[]> { const response = await fetch(`${API_BASE}/entries`); if (!response.ok) throw new Error('Failed to fetch entries'); return response.json(); }
export async function clearCache(): Promise<Response> { const response = await fetch(`${API_BASE}/clear`, { method: 'POST' }); if (!response.ok) throw new Error('Failed to clear cache'); return response; }
export async function deleteEntry(hash: string): Promise<Response> { const response = await fetch(`${API_BASE}/entries/${hash}`, { method: 'DELETE' }); if (!response.ok) throw new Error('Failed to delete entry'); return response; }
export function downloadCert() { window.location.href = `${API_BASE}/ca.crt`; }
export async function fetchProxyStatus(): Promise<{ running: boolean }> { const response = await fetch(`${API_BASE}/proxy/status`); if (!response.ok) throw new Error('Failed to fetch proxy status'); return response.json(); }
export async function startProxy(): Promise<Response> { const response = await fetch(`${API_BASE}/proxy/start`, { method: 'POST' }); if (!response.ok) throw new Error(await response.text()); return response; }
export async function stopProxy(): Promise<Response> { const response = await fetch(`${API_BASE}/proxy/stop`, { method: 'POST' }); if (!response.ok) throw new Error(await response.text()); return response; }
export async function enableSystemProxy(): Promise<Response> { const response = await fetch(`${API_BASE}/system_proxy/enable`, { method: 'POST' }); if (!response.ok) throw new Error(await response.text()); return response; }
export async function disableSystemProxy(): Promise<Response> { const response = await fetch(`${API_BASE}/system_proxy/disable`, { method: 'POST' }); if (!response.ok) throw new Error(await response.text()); return response; }

// YENİ: Sistem bilgisi için API çağrısı
export async function fetchSystemInfo(): Promise<SystemInfo> {
    const response = await fetch(`${API_BASE}/system_info`);
    if (!response.ok) throw new Error('Failed to fetch system info');
    return response.json();
}

interface EventStreamCallbacks {
    onLog?: (message: string) => void;
    onRequestBegin?: (event: RequestBegin) => void;
    onRequestEnd?: (event: RequestEnd) => void;
    onStatsUpdated?: (stats: CacheStats) => void;
    onDataChanged?: () => void;
}

let socket: WebSocket | null = null;
const subscribers = new Set<EventStreamCallbacks>();

function connect() {
    const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${wsProtocol}//${window.location.host}${API_BASE}/events`;
    socket = new WebSocket(wsUrl);

    socket.onopen = () => console.log("Global event stream connected.");

    socket.onmessage = (event) => {
        try {
            const parsedEvent = JSON.parse(event.data) as WsEvent;
            for (const sub of subscribers) {
                switch (parsedEvent.type) {
                    case 'log': sub.onLog?.(parsedEvent.payload); break;
                    case 'requestBegin': sub.onRequestBegin?.(parsedEvent.event); break;
                    case 'requestEnd': sub.onRequestEnd?.(parsedEvent.event); break;
                    case 'statsUpdated': sub.onStatsUpdated?.(parsedEvent.stats); break;
                    case 'dataChanged': sub.onDataChanged?.(); break;
                }
            }
        } catch (e) {
            console.error("Failed to parse WebSocket event:", event.data, e);
        }
    };

    socket.onclose = () => {
        console.log("Global event stream disconnected. Reconnecting in 3 seconds...");
        socket = null;
        setTimeout(connect, 3000);
    };

    socket.onerror = (error) => {
        console.error("Event stream WebSocket error:", error);
        socket?.close();
    };
}

connect();

export function subscribeToEvents(callbacks: EventStreamCallbacks): () => void {
    subscribers.add(callbacks);
    return () => {
        subscribers.delete(callbacks);
    };
}