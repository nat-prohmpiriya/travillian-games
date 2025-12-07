import { writable } from "svelte/store";
import { type User, onAuthStateChanged, GoogleAuthProvider, signInWithPopup, signOut } from "firebase/auth";
import { auth } from "../firebase/config";

interface AuthState {
    user: User | null;
    loading: boolean;
    isAuthenticated: boolean;
    token: string | null;
}

function createAuthStore() {
    const { subscribe, set, update } = writable<AuthState>({
        user: null,
        loading: true,
        isAuthenticated: false,
        token: null,
    });

    return {
        subscribe,
        loginWithGoogle: async () => {
            const provider = new GoogleAuthProvider();
            await signInWithPopup(auth, provider);
        },
        logout: async () => {
            await signOut(auth);
        },
        init: () => {
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
