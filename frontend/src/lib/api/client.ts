import { auth } from "../firebase/config";

const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

type RequestMethod = 'GET' | 'POST' | 'PUT' | 'DELETE';

interface RequestOptions {
    headers?: Record<string, string>;
    body?: any;
    auth?: boolean; // Default true
}

async function request<T>(method: RequestMethod, endpoint: string, options: RequestOptions = {}): Promise<T> {
    const { headers = {}, body, auth: useAuth = true } = options;

    const config: RequestInit = {
        method,
        headers: {
            'Content-Type': 'application/json',
            ...headers,
        },
    };

    if (useAuth && auth?.currentUser) {
        const token = await auth.currentUser.getIdToken();
        (config.headers as Record<string, string>)['Authorization'] = `Bearer ${token}`;
    }

    if (body) {
        config.body = JSON.stringify(body);
    }

    const response = await fetch(`${BASE_URL}${endpoint}`, config);

    if (!response.ok) {
        let errorMessage = 'An error occurred';
        try {
            const errorData = await response.json();
            errorMessage = errorData.message || errorMessage;
        } catch (e) {
            // Ignore JSON parse error
        }
        throw new Error(errorMessage);
    }

    // Handle 204 No Content
    if (response.status === 204) {
        return {} as T;
    }

    return response.json();
}

export const api = {
    get: <T>(endpoint: string, options?: RequestOptions) => request<T>('GET', endpoint, options),
    post: <T>(endpoint: string, body: any, options?: RequestOptions) => request<T>('POST', endpoint, { ...options, body }),
    put: <T>(endpoint: string, body: any, options?: RequestOptions) => request<T>('PUT', endpoint, { ...options, body }),
    delete: <T>(endpoint: string, options?: RequestOptions) => request<T>('DELETE', endpoint, options),
};
