import { writable } from "svelte/store";
import { api } from "../api/client";

export interface MapVillageInfo {
    id: string;
    name: string;
    player_name: string | null;
    population: number;
    is_own: boolean;
}

export interface MapTile {
    x: number;
    y: number;
    village: MapVillageInfo | null;
}

interface MapState {
    tiles: MapTile[];
    centerX: number;
    centerY: number;
    range: number;
    loading: boolean;
    error: string | null;
}

function createMapStore() {
    const { subscribe, set, update } = writable<MapState>({
        tiles: [],
        centerX: 0,
        centerY: 0,
        range: 7,
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load map tiles from API
        loadTiles: async (x: number, y: number, range: number = 7) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const tiles = await api.get<MapTile[]>(`/api/map?x=${x}&y=${y}&range=${range}`);

                update(state => ({
                    ...state,
                    tiles,
                    centerX: x,
                    centerY: y,
                    range,
                    loading: false,
                }));

                return tiles;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to load map',
                }));
                throw error;
            }
        },

        // Set center coordinates
        setCenter: (x: number, y: number) => {
            update(state => ({ ...state, centerX: x, centerY: y }));
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                tiles: [],
                centerX: 0,
                centerY: 0,
                range: 7,
                loading: false,
                error: null,
            });
        },
    };
}

export const mapStore = createMapStore();

// Helper function to get village at coordinates from tiles array
export function getVillageAt(tiles: MapTile[], x: number, y: number): MapVillageInfo | null {
    const tile = tiles.find(t => t.x === x && t.y === y);
    return tile?.village || null;
}

// Helper to determine owner type from API response
export function getOwnerType(village: MapVillageInfo | null): 'self' | 'ally' | 'enemy' | 'neutral' | 'npc' | null {
    if (!village) return null;

    if (village.is_own) return 'self';

    // For now, treat all non-own villages as neutral
    // In the future, this would check alliance status
    if (village.player_name === 'Natarian') return 'npc';

    return 'neutral';
}
