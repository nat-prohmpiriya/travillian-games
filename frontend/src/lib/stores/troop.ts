import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";

// Troop types matching backend
export type TroopType =
    | 'infantry'
    | 'spearman'
    | 'war_elephant'
    | 'buffalo_wagon'
    | 'kris_warrior'
    | 'sea_diver'
    | 'war_prahu'
    | 'merchant_ship'
    | 'crossbowman'
    | 'mountain_warrior'
    | 'highland_pony'
    | 'trap_maker'
    | 'swamp_dragon'
    | 'locust_swarm'
    | 'battle_duck'
    | 'portuguese_musketeer'
    // Chief units (can reduce loyalty)
    | 'royal_advisor'
    | 'harbor_master'
    | 'elder_chief';

export type TribeType = 'phasuttha' | 'nava' | 'kiri' | 'special';

export interface TroopDefinition {
    troop_type: TroopType;
    tribe: TribeType;
    name: string;
    description: string | null;
    attack: number;
    defense_infantry: number;
    defense_cavalry: number;
    speed: number;
    carry_capacity: number;
    crop_consumption: number;
    training_time_seconds: number;
    wood_cost: number;
    clay_cost: number;
    iron_cost: number;
    crop_cost: number;
    required_building: string;
    required_building_level: number;
    loyalty_reduction: number;
}

export interface Troop {
    troop_type: TroopType;
    count: number;
    in_village: number;
    on_mission: number;
}

export interface TroopQueueItem {
    id: string;
    troop_type: TroopType;
    count: number;
    each_duration_seconds: number;
    started_at: string;
    ends_at: string;
}

export interface TroopCost {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    time_seconds: number;
}

export interface TrainResponse {
    queue_entry: TroopQueueItem;
    cost: TroopCost;
}

interface TroopState {
    definitions: TroopDefinition[];
    troops: Troop[];
    queue: TroopQueueItem[];
    loading: boolean;
    error: string | null;
}

function createTroopStore() {
    const { subscribe, set, update } = writable<TroopState>({
        definitions: [],
        troops: [],
        queue: [],
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Load all troop definitions
        loadDefinitions: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const definitions = await api.get<TroopDefinition[]>('/api/troops/definitions');
                update(state => ({
                    ...state,
                    definitions,
                    loading: false,
                }));
                return definitions;
            } catch (error: any) {
                const message = error.message || 'Failed to load troop definitions';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load troops for a village
        loadTroops: async (villageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const troops = await api.get<Troop[]>(`/api/villages/${villageId}/troops`);
                update(state => ({
                    ...state,
                    troops,
                    loading: false,
                }));
                return troops;
            } catch (error: any) {
                const message = error.message || 'Failed to load troops';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load training queue for a village
        loadQueue: async (villageId: string) => {
            try {
                const queue = await api.get<TroopQueueItem[]>(`/api/villages/${villageId}/troops/queue`);
                update(state => ({ ...state, queue }));
                return queue;
            } catch (error: any) {
                console.error('Failed to load training queue:', error);
                return [];
            }
        },

        // Train troops
        train: async (villageId: string, troopType: TroopType, count: number) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const response = await api.post<TrainResponse>(
                    `/api/villages/${villageId}/troops/train`,
                    { troop_type: troopType, count }
                );

                update(state => ({
                    ...state,
                    queue: [...state.queue, response.queue_entry],
                    loading: false,
                }));

                toast.success('Training Started', {
                    description: `Training ${count} ${formatTroopName(troopType)}`
                });

                return response;
            } catch (error: any) {
                const message = error.message || 'Failed to start training';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Training Failed', { description: message });
                throw error;
            }
        },

        // Cancel training
        cancelTraining: async (villageId: string, queueId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                await api.delete(`/api/villages/${villageId}/troops/queue/${queueId}`);

                update(state => ({
                    ...state,
                    queue: state.queue.filter(q => q.id !== queueId),
                    loading: false,
                }));

                toast.success('Training Cancelled');

                return true;
            } catch (error: any) {
                const message = error.message || 'Failed to cancel training';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Cancel Failed', { description: message });
                throw error;
            }
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                definitions: [],
                troops: [],
                queue: [],
                loading: false,
                error: null,
            });
        },
    };
}

export const troopStore = createTroopStore();

// Helper functions
export function formatTroopName(type: TroopType): string {
    return type
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
}

export function getTroopIcon(type: TroopType): string {
    const icons: Record<TroopType, string> = {
        infantry: '⚔️',
        spearman: '🗡️',
        war_elephant: '🐘',
        buffalo_wagon: '🛒',
        kris_warrior: '🗡️',
        sea_diver: '🤿',
        war_prahu: '⛵',
        merchant_ship: '🚢',
        crossbowman: '🏹',
        mountain_warrior: '⛰️',
        highland_pony: '🐎',
        trap_maker: '🪤',
        swamp_dragon: '🐊',
        locust_swarm: '🦗',
        battle_duck: '🦆',
        portuguese_musketeer: '🔫',
        // Chief units
        royal_advisor: '👑',
        harbor_master: '⚓',
        elder_chief: '🧙',
    };
    return icons[type] || '👤';
}

export function getTribeColor(tribe: TribeType): string {
    const colors: Record<TribeType, string> = {
        phasuttha: 'text-amber-600',
        nava: 'text-blue-600',
        kiri: 'text-green-600',
        special: 'text-purple-600',
    };
    return colors[tribe] || 'text-gray-600';
}

export function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) {
        return `${hours}h ${minutes}m`;
    }
    if (minutes > 0) {
        return `${minutes}m ${secs}s`;
    }
    return `${secs}s`;
}

export function getDefinitionByType(definitions: TroopDefinition[], type: TroopType): TroopDefinition | undefined {
    return definitions.find(d => d.troop_type === type);
}

// Chief troops can reduce village loyalty during Conquer missions
export function isChiefTroop(type: TroopType): boolean {
    return type === 'royal_advisor' || type === 'harbor_master' || type === 'elder_chief';
}
