import { writable } from "svelte/store";
import { toast } from "svelte-sonner";
import { api } from "../api/client";
import type { TroopType } from "./troop";

// Mission types matching backend
export type MissionType = 'raid' | 'attack' | 'conquer' | 'support' | 'scout' | 'settle';

export interface CarriedResources {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
}

export type TroopCounts = Partial<Record<TroopType, number>>;

export interface Army {
    id: string;
    from_village_id: string;
    to_x: number;
    to_y: number;
    to_village_id: string | null;
    mission: MissionType;
    troops: TroopCounts;
    resources: CarriedResources;
    departed_at: string;
    arrives_at: string;
    returns_at: string | null;
    is_returning: boolean;
}

export interface BattleReport {
    id: string;
    attacker_player_id: string;
    defender_player_id: string | null;
    attacker_village_id: string;
    defender_village_id: string | null;
    mission: MissionType;
    attacker_troops: TroopCounts;
    defender_troops: TroopCounts;
    attacker_losses: TroopCounts;
    defender_losses: TroopCounts;
    resources_stolen: CarriedResources;
    winner: 'attacker' | 'defender' | 'draw';
    occurred_at: string;
    is_read: boolean;
}

export interface SendArmyRequest {
    to_x: number;
    to_y: number;
    mission: MissionType;
    troops: TroopCounts;
    resources?: CarriedResources;
}

interface ArmyState {
    outgoingArmies: Army[];
    incomingArmies: Army[];
    reports: BattleReport[];
    unreadCount: number;
    loading: boolean;
    error: string | null;
}

function createArmyStore() {
    const { subscribe, set, update } = writable<ArmyState>({
        outgoingArmies: [],
        incomingArmies: [],
        reports: [],
        unreadCount: 0,
        loading: false,
        error: null,
    });

    return {
        subscribe,

        // Send army from village
        sendArmy: async (villageId: string, request: SendArmyRequest) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const army = await api.post<Army>(
                    `/api/villages/${villageId}/armies`,
                    request
                );

                update(state => ({
                    ...state,
                    outgoingArmies: [...state.outgoingArmies, army],
                    loading: false,
                }));

                const missionLabel = getMissionLabel(request.mission);
                toast.success('Army Sent', {
                    description: `${missionLabel} to (${request.to_x}, ${request.to_y})`
                });

                return army;
            } catch (error: any) {
                const message = error.message || 'Failed to send army';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                toast.error('Send Failed', { description: message });
                throw error;
            }
        },

        // Load outgoing armies from village
        loadOutgoing: async (villageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const armies = await api.get<Army[]>(
                    `/api/villages/${villageId}/armies/outgoing`
                );
                update(state => ({
                    ...state,
                    outgoingArmies: armies,
                    loading: false,
                }));
                return armies;
            } catch (error: any) {
                const message = error.message || 'Failed to load outgoing armies';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load incoming armies to village
        loadIncoming: async (villageId: string) => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const armies = await api.get<Army[]>(
                    `/api/villages/${villageId}/armies/incoming`
                );
                update(state => ({
                    ...state,
                    incomingArmies: armies,
                    loading: false,
                }));
                return armies;
            } catch (error: any) {
                const message = error.message || 'Failed to load incoming armies';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load battle reports
        loadReports: async () => {
            update(state => ({ ...state, loading: true, error: null }));
            try {
                const reports = await api.get<BattleReport[]>('/api/reports');
                update(state => ({
                    ...state,
                    reports,
                    loading: false,
                }));
                return reports;
            } catch (error: any) {
                const message = error.message || 'Failed to load reports';
                update(state => ({
                    ...state,
                    loading: false,
                    error: message,
                }));
                throw error;
            }
        },

        // Load single report
        loadReport: async (reportId: string) => {
            try {
                return await api.get<BattleReport>(`/api/reports/${reportId}`);
            } catch (error: any) {
                console.error('Failed to load report:', error);
                throw error;
            }
        },

        // Mark report as read
        markRead: async (reportId: string) => {
            try {
                await api.post(`/api/reports/${reportId}/read`, {});
                update(state => ({
                    ...state,
                    reports: state.reports.map(r =>
                        r.id === reportId ? { ...r, is_read: true } : r
                    ),
                    unreadCount: Math.max(0, state.unreadCount - 1),
                }));
            } catch (error: any) {
                console.error('Failed to mark report as read:', error);
            }
        },

        // Load unread count
        loadUnreadCount: async () => {
            try {
                const response = await api.get<{ unread_count: number }>(
                    '/api/reports/unread-count'
                );
                update(state => ({
                    ...state,
                    unreadCount: response.unread_count,
                }));
                return response.unread_count;
            } catch (error: any) {
                console.error('Failed to load unread count:', error);
                return 0;
            }
        },

        // Clear error
        clearError: () => {
            update(state => ({ ...state, error: null }));
        },

        // Reset store
        reset: () => {
            set({
                outgoingArmies: [],
                incomingArmies: [],
                reports: [],
                unreadCount: 0,
                loading: false,
                error: null,
            });
        },
    };
}

export const armyStore = createArmyStore();

// Helper functions
export function getMissionLabel(mission: MissionType): string {
    const labels: Record<MissionType, string> = {
        raid: 'Raid',
        attack: 'Attack',
        conquer: 'Conquer',
        support: 'Reinforce',
        scout: 'Scout',
        settle: 'Settle',
    };
    return labels[mission] || mission;
}

export function getMissionIcon(mission: MissionType): string {
    const icons: Record<MissionType, string> = {
        raid: '💰',
        attack: '⚔️',
        conquer: '🏰',
        support: '🛡️',
        scout: '👁️',
        settle: '🏗️',
    };
    return icons[mission] || '📍';
}

export function getMissionColor(mission: MissionType): string {
    const colors: Record<MissionType, string> = {
        raid: 'text-yellow-600',
        attack: 'text-red-600',
        conquer: 'text-purple-600',
        support: 'text-blue-600',
        scout: 'text-gray-600',
        settle: 'text-green-600',
    };
    return colors[mission] || 'text-gray-600';
}

export function getWinnerLabel(winner: string, isAttacker: boolean): string {
    if (winner === 'draw') return 'Draw';
    const won = (winner === 'attacker') === isAttacker;
    return won ? 'Victory' : 'Defeat';
}

export function getWinnerColor(winner: string, isAttacker: boolean): string {
    if (winner === 'draw') return 'text-gray-600';
    const won = (winner === 'attacker') === isAttacker;
    return won ? 'text-green-600' : 'text-red-600';
}

export function formatArrivalTime(arrivedAt: string): string {
    const arrival = new Date(arrivedAt);
    const now = new Date();
    const diff = arrival.getTime() - now.getTime();

    if (diff <= 0) return 'Arrived';

    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((diff % (1000 * 60)) / 1000);

    if (hours > 0) {
        return `${hours}h ${minutes}m`;
    }
    if (minutes > 0) {
        return `${minutes}m ${seconds}s`;
    }
    return `${seconds}s`;
}

export function getTotalTroops(troops: TroopCounts): number {
    return Object.values(troops).reduce((sum, count) => sum + (count || 0), 0);
}

export function getTotalResources(resources: CarriedResources): number {
    return resources.wood + resources.clay + resources.iron + resources.crop;
}
