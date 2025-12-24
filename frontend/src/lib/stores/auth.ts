import { writable, get } from "svelte/store";
import { type User, onAuthStateChanged, GoogleAuthProvider, signInWithPopup, signOut } from "firebase/auth";
import { auth, isFirebaseConfigured } from "../firebase/config";
import { api } from "../api/client";

interface BackendUser {
    id: string;
    firebase_uid: string;
    email: string | null;
    display_name: string | null;
    photo_url: string | null;
    provider: string;
    created_at: string;
    updated_at: string;
}

interface SyncResponse {
    user: BackendUser;
    is_new: boolean;
}

interface Village {
    id: string;
    name: string;
    x: number;
    y: number;
    is_capital: boolean;
}

interface AuthState {
    user: User | null;
    backendUser: BackendUser | null;
    loading: boolean;
    syncing: boolean;
    isAuthenticated: boolean;
    token: string | null;
    firebaseConfigured: boolean;
    hasVillage: boolean;
    villages: Village[];
    syncError: string | null;
}

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        user: null,
        backendUser: null,
        loading: true,
        syncing: false,
        isAuthenticated: false,
        token: null,
        firebaseConfigured: false,
        hasVillage: false,
        villages: [],
        syncError: null,
    });

    const syncWithBackend = async (): Promise<{ isNew: boolean; hasVillage: boolean }> => {
        update(state => ({ ...state, syncing: true, syncError: null }));

        try {
            // Sync user with backend
            const syncResponse = await api.post<SyncResponse>('/api/auth/sync', {});

            // Check if user has villages
            const villages = await api.get<Village[]>('/api/villages');
            const hasVillage = villages.length > 0;

            update(state => ({
                ...state,
                backendUser: syncResponse.user,
                hasVillage,
                villages,
                syncing: false,
            }));

            return { isNew: syncResponse.is_new, hasVillage };
        } catch (error: any) {
            const errorMessage = error.message || 'Failed to sync with backend';
            update(state => ({ ...state, syncing: false, syncError: errorMessage }));
            throw error;
        }
    };

    const checkHasVillage = async (): Promise<boolean> => {
        try {
            const villages = await api.get<Village[]>('/api/villages');
            const hasVillage = villages.length > 0;
            update(state => ({ ...state, hasVillage, villages }));
            return hasVillage;
        } catch {
            return false;
        }
    };

    return {
        subscribe,
        syncWithBackend,
        checkHasVillage,
        loginWithGoogle: async () => {
            if (!auth) {
                throw new Error('Firebase is not configured. Please set up environment variables.');
            }
            const provider = new GoogleAuthProvider();
            const result = await signInWithPopup(auth, provider);

            // After successful login, sync with backend
            if (result.user) {
                return await syncWithBackend();
            }
            return { isNew: true, hasVillage: false };
        },
        logout: async () => {
            if (!auth) {
                throw new Error('Firebase is not configured.');
            }
            await signOut(auth);
            update(state => ({
                ...state,
                backendUser: null,
                hasVillage: false,
                villages: [],
            }));
        },
        init: () => {
            // If Firebase is not configured, just mark as not loading
            if (!auth || !isFirebaseConfigured) {
                update(state => ({
                    ...state,
                    loading: false,
                    firebaseConfigured: false,
                }));
                return;
            }

            update(state => ({
                ...state,
                firebaseConfigured: true,
            }));

            onAuthStateChanged(auth, async (user) => {
                let token = null;
                if (user) {
                    token = await user.getIdToken();

                    // Auto-sync with backend when auth state changes
                    try {
                        await syncWithBackend();
                    } catch (error) {
                        console.error('Failed to sync with backend:', error);
                    }
                }

                update(state => ({
                    ...state,
                    user,
                    token,
                    isAuthenticated: !!user,
                    loading: false
                }));
            });
        }
    };
}

export const authStore = createAuthStore();
