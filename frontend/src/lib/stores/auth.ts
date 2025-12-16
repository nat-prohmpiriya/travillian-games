import { writable } from "svelte/store";
import { type User, onAuthStateChanged, GoogleAuthProvider, signInWithPopup, signOut } from "firebase/auth";
import { auth, isFirebaseConfigured } from "../firebase/config";

interface AuthState {
    user: User | null;
    loading: boolean;
    isAuthenticated: boolean;
    token: string | null;
    firebaseConfigured: boolean;
}

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        user: null,
        loading: true,
        isAuthenticated: false,
        token: null,
        firebaseConfigured: false,
    });

    return {
        subscribe,
        loginWithGoogle: async () => {
            if (!auth) {
                throw new Error('Firebase is not configured. Please set up environment variables.');
            }
            const provider = new GoogleAuthProvider();
            await signInWithPopup(auth, provider);
        },
        logout: async () => {
            if (!auth) {
                throw new Error('Firebase is not configured.');
            }
            await signOut(auth);
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
