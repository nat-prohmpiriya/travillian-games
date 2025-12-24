import { writable, get } from "svelte/store";
import { api } from "../api/client";

// Building types matching backend enum + frontend-only types
export type BuildingType =
    | 'empty'
    | 'main_building'
    | 'warehouse'
    | 'granary'
    | 'barracks'
    | 'stable'
    | 'workshop'
    | 'academy'
    | 'smithy'
    | 'rally_point'
    | 'market'
    | 'embassy'
    | 'town_hall'
    | 'residence'
    | 'palace'
    | 'treasury'
    | 'trade_office'
    | 'wall'
    | 'cranny'
    | 'hero_mansion'
    | 'tavern'
    | 'woodcutter'
    | 'clay_pit'
    | 'iron_mine'
    | 'crop_field';

export interface Building {
    id: string;
    building_type: BuildingType;
    slot: number;
    level: number;
    is_upgrading: boolean;
    upgrade_ends_at: string | null;
}

export interface BuildingCost {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    time_seconds: number;
}

export interface ProductionRates {
    wood_per_hour: number;
    clay_per_hour: number;
    iron_per_hour: number;
    crop_per_hour: number;
    crop_consumption: number;
    net_crop_per_hour: number;
}

export interface Village {
    id: string;
    name: string;
    x: number;
    y: number;
    is_capital: boolean;
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    warehouse_capacity: number;
    granary_capacity: number;
    population: number;
    culture_points: number;
    loyalty: number;
    created_at: string;
    production?: ProductionRates;
}

interface BuildResponse {
    building: Building;
    cost: BuildingCost;
}

interface UpgradeResponse {
    building: Building;
    cost: BuildingCost;
}

interface VillageState {
    villages: Village[];
    currentVillage: Village | null;
    buildings: Building[];
    buildQueue: Building[];
    loading: boolean;
    error: string | null;
}

function createVillageStore() {
    const { subscribe, set, update } = writable<VillageState>({
        villages: [],
        currentVillage: null,
        buildings: [],
        buildQueue: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load all villages for current user
        loadVillages: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const villages = await api.get<Village[]>('/api/villages');
                update(state => ({
                    ...state,
                    villages,
                    loading: false,
                }));
                return villages;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to load villages',
                }));
                throw error;
            }
        },

        // Load a specific village with production rates
        loadVillage: async (villageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const [village, buildings, buildQueue] = await Promise.all([
                    api.get<Village>(`/api/villages/${villageId}`),
                    api.get<Building[]>(`/api/villages/${villageId}/buildings`),
                    api.get<Building[]>(`/api/villages/${villageId}/buildings/queue`),
                ]);

                update(state => ({
                    ...state,
                    currentVillage: village,
                    buildings,
                    buildQueue,
                    loading: false,
                }));

                return { village, buildings, buildQueue };
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to load village',
                }));
                throw error;
            }
        },

        // Set current village
        setCurrentVillage: (village: Village) => {
            update(state => ({ ...state, currentVillage: village }));
        },

        // Build new building
        build: async (villageId: string, slot: number, buildingType: BuildingType) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.post<BuildResponse>(
                    `/api/villages/${villageId}/buildings/${slot}`,
                    { building_type: buildingType }
                );

                // Update buildings and build queue
                update(state => ({
                    ...state,
                    buildings: [...state.buildings, response.building],
                    buildQueue: [...state.buildQueue, response.building],
                    loading: false,
                }));

                // Reload village to get updated resources
                const village = await api.get<Village>(`/api/villages/${villageId}`);
                update(state => ({ ...state, currentVillage: village }));

                return response;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to build',
                }));
                throw error;
            }
        },

        // Upgrade existing building
        upgrade: async (villageId: string, slot: number) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.post<UpgradeResponse>(
                    `/api/villages/${villageId}/buildings/${slot}/upgrade`,
                    {}
                );

                // Update buildings list
                update(state => ({
                    ...state,
                    buildings: state.buildings.map(b =>
                        b.slot === slot ? response.building : b
                    ),
                    buildQueue: [...state.buildQueue, response.building],
                    loading: false,
                }));

                // Reload village to get updated resources
                const village = await api.get<Village>(`/api/villages/${villageId}`);
                update(state => ({ ...state, currentVillage: village }));

                return response;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to upgrade',
                }));
                throw error;
            }
        },

        // Demolish building
        demolish: async (villageId: string, slot: number) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.delete(`/api/villages/${villageId}/buildings/${slot}`);

                // Remove from buildings list
                update(state => ({
                    ...state,
                    buildings: state.buildings.filter(b => b.slot !== slot),
                    buildQueue: state.buildQueue.filter(b => b.slot !== slot),
                    loading: false,
                }));

                return true;
            } catch (error: any) {
                update(state => ({
                    ...state,
                    loading: false,
                    error: error.message || 'Failed to demolish',
                }));
                throw error;
            }
        },

        // Refresh build queue
        refreshBuildQueue: async (villageId: string) => {
            try {
                const buildQueue = await api.get<Building[]>(`/api/villages/${villageId}/buildings/queue`);
                update(state => ({ ...state, buildQueue }));
                return buildQueue;
            } catch {
                return [];
            }
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                villages: [],
                currentVillage: null,
                buildings: [],
                buildQueue: [],
                loading: false,
                error: null,
            });
        },
    };
}

export const villageStore = createVillageStore();

// Helper function to get building by slot
export function getBuildingBySlot(buildings: Building[], slot: number): Building | null {
    return buildings.find(b => b.slot === slot) || null;
}

// Helper function to check if slot is empty
export function isSlotEmpty(buildings: Building[], slot: number): boolean {
    return !buildings.some(b => b.slot === slot);
}

// Helper to format building type for display
export function formatBuildingType(type: BuildingType): string {
    return type
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

// Helper to check if building is resource field
export function isResourceField(type: BuildingType): boolean {
    return ['woodcutter', 'clay_pit', 'iron_mine', 'crop_field'].includes(type);
}
