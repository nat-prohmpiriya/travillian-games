import { writable, type Writable } from 'svelte/store';
import { auth } from '../firebase/config';

interface WebSocketState {
    connected: boolean;
    error: Event | null;
}

export const wsState: Writable<WebSocketState> = writable({
    connected: false,
    error: null
});

type WebSocketHandler = (data: any) => void;

class WebSocketClient {
    private ws: WebSocket | null = null;
    private url: string;
    private reconnectAttempts = 0;
    private maxReconnectAttempts = 5;
    private reconnectDelay = 1000;
    private handlers: Map<string, WebSocketHandler[]> = new Map();
    private shouldReconnect = true;

    constructor() {
        const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:8080';
        // Convert http(s) to ws(s)
        this.url = apiUrl.replace(/^http/, 'ws') + '/ws';
    }

    async connect() {
        if (this.ws?.readyState === WebSocket.OPEN) return;

        this.shouldReconnect = true;
        let token = '';

        if (auth?.currentUser) {
            token = await auth.currentUser.getIdToken();
        }

        const wsUrl = token ? `${this.url}?token=${token}` : this.url;

        try {
            this.ws = new WebSocket(wsUrl);

            this.ws.onopen = () => {
                console.log('WebSocket connected');
                wsState.set({ connected: true, error: null });
                this.reconnectAttempts = 0;
            };

            this.ws.onclose = () => {
                console.log('WebSocket disconnected');
                wsState.update(s => ({ ...s, connected: false }));
                if (this.shouldReconnect) {
                    this.handleReconnect();
                }
            };

            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                wsState.update(s => ({ ...s, error }));
            };

            this.ws.onmessage = (event) => {
                try {
                    const message = JSON.parse(event.data);
                    this.dispatch(message.type, message.data);
                } catch (e) {
                    console.error('Failed to parse WebSocket message:', event.data);
                }
            };

        } catch (err) {
            console.error('Failed to connect WebSocket:', err);
            this.handleReconnect();
        }
    }

    disconnect() {
        this.shouldReconnect = false;
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    send(type: string, data: any) {
        if (this.ws?.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify({ type, data }));
        } else {
            console.warn('WebSocket not connected, cannot send message:', type);
        }
    }

    subscribe(type: string, handler: WebSocketHandler) {
        if (!this.handlers.has(type)) {
            this.handlers.set(type, []);
        }
        this.handlers.get(type)?.push(handler);

        // Return unsubscribe function
        return () => {
            const currentHandlers = this.handlers.get(type) || [];
            this.handlers.set(type, currentHandlers.filter(h => h !== handler));
        };
    }

    private dispatch(type: string, data: any) {
        const handlers = this.handlers.get(type);
        if (handlers) {
            handlers.forEach(h => h(data));
        }
    }

    private handleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts);
            console.log(`Reconnecting in ${delay}ms... (Attempt ${this.reconnectAttempts + 1})`);

            setTimeout(() => {
                this.reconnectAttempts++;
                this.connect();
            }, delay);
        } else {
            console.error('Max reconnection attempts reached');
        }
    }
}

export const wsClient = new WebSocketClient();
