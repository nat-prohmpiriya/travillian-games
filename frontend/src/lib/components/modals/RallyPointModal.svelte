<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import { Input } from '$lib/components/ui/input';
    import { Label } from '$lib/components/ui/label';
    import { Card } from '$lib/components/ui/card';
    import { Separator } from '$lib/components/ui/separator';
    import TroopSelector from '$lib/components/game/TroopSelector.svelte';
    import {
        troopStore,
        type Troop,
        type TroopDefinition,
        type TroopType,
    } from '$lib/stores/troop';
    import {
        armyStore,
        type MissionType,
        type SendArmyRequest,
        type TroopCounts,
        getMissionLabel,
        getMissionIcon,
    } from '$lib/stores/army';

    interface Props {
        open: boolean;
        villageId: string;
        villageX: number;
        villageY: number;
        targetX?: number;
        targetY?: number;
    }

    let { open = $bindable(false), villageId, villageX, villageY, targetX, targetY }: Props = $props();

    // Form state
    let toX = $state(targetX || 0);
    let toY = $state(targetY || 0);
    let mission = $state<MissionType>('raid');
    let selectedTroops = $state<TroopCounts>({});
    let loading = $state(false);
    let error = $state<string | null>(null);

    // Data from stores
    let troops = $state<Troop[]>([]);
    let definitions = $state<TroopDefinition[]>([]);

    // Reset when target changes
    $effect(() => {
        if (targetX !== undefined) toX = targetX;
        if (targetY !== undefined) toY = targetY;
    });

    // Load data when modal opens
    async function loadData() {
        if (!villageId) return;

        loading = true;
        error = null;
        try {
            await Promise.all([
                troopStore.loadDefinitions(),
                troopStore.loadTroops(villageId),
            ]);

            troopStore.subscribe((state) => {
                definitions = state.definitions;
                troops = state.troops;
            });
        } catch (e: any) {
            error = e.message || 'Failed to load data';
        } finally {
            loading = false;
        }
    }

    $effect(() => {
        if (open) {
            loadData();
            selectedTroops = {};
        }
    });

    // Calculate total troops
    const totalTroops = $derived(
        Object.values(selectedTroops).reduce((sum, count) => sum + (count || 0), 0)
    );

    // Get definition for a troop type
    function getDefinition(type: TroopType): TroopDefinition | undefined {
        return definitions.find(d => d.troop_type === type);
    }

    // Calculate slowest speed
    const slowestSpeed = $derived(() => {
        let minSpeed = Infinity;
        for (const [type, count] of Object.entries(selectedTroops)) {
            if (count > 0) {
                const def = getDefinition(type as TroopType);
                if (def && def.speed < minSpeed) {
                    minSpeed = def.speed;
                }
            }
        }
        return minSpeed === Infinity ? 6 : minSpeed;
    });

    // Calculate distance
    const distance = $derived(() => {
        const dx = toX - villageX;
        const dy = toY - villageY;
        return Math.sqrt(dx * dx + dy * dy);
    });

    // Calculate travel time
    const travelTimeSeconds = $derived(() => {
        const dist = distance();
        const speed = slowestSpeed();
        if (speed <= 0) return 0;
        const hours = dist / speed;
        return Math.max(60, Math.floor(hours * 3600));
    });

    // Format travel time
    function formatTravelTime(seconds: number): string {
        if (seconds < 60) return `${seconds}s`;
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        const secs = seconds % 60;

        if (hours > 0) {
            return `${hours}h ${minutes}m ${secs}s`;
        }
        return `${minutes}m ${secs}s`;
    }

    // Calculate total carry capacity
    const totalCarryCapacity = $derived(
        Object.entries(selectedTroops).reduce((sum, [type, count]) => {
            const def = getDefinition(type as TroopType);
            return sum + (def ? def.carry_capacity * count : 0);
        }, 0)
    );

    // Validate form
    const isValid = $derived(
        totalTroops > 0 &&
        (toX !== villageX || toY !== villageY) &&
        !loading
    );

    // Handle send army
    async function handleSend() {
        if (!isValid) return;

        loading = true;
        error = null;
        try {
            const request: SendArmyRequest = {
                to_x: toX,
                to_y: toY,
                mission,
                troops: selectedTroops,
            };

            await armyStore.sendArmy(villageId, request);
            open = false;
            selectedTroops = {};
        } catch (e: any) {
            error = e.message || 'Failed to send army';
        } finally {
            loading = false;
        }
    }

    // Mission options (Raid, Attack, Scout, and Support)
    const missionOptions: { type: MissionType; enabled: boolean }[] = [
        { type: 'raid', enabled: true },
        { type: 'attack', enabled: true },
        { type: 'scout', enabled: true },
        { type: 'support', enabled: true },
    ];
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-2xl max-h-[90vh] overflow-y-auto">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    🚩
                </div>
                <div>
                    <Dialog.Title class="text-xl">Rally Point</Dialog.Title>
                    <Dialog.Description>
                        Send troops from ({villageX}, {villageY})
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <div class="space-y-6 py-4">
            {#if loading && troops.length === 0}
                <div class="flex items-center justify-center py-8">
                    <span class="animate-spin text-2xl mr-2">⏳</span>
                    <span>Loading...</span>
                </div>
            {:else}
                <!-- Target Coordinates -->
                <div>
                    <Label class="text-sm font-medium mb-2 block">Target Coordinates</Label>
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-2">
                            <span class="text-muted-foreground">X:</span>
                            <Input
                                type="number"
                                bind:value={toX}
                                class="w-24"
                            />
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-muted-foreground">Y:</span>
                            <Input
                                type="number"
                                bind:value={toY}
                                class="w-24"
                            />
                        </div>
                        {#if distance() > 0}
                            <span class="text-sm text-muted-foreground">
                                📍 Distance: {distance().toFixed(1)} fields
                            </span>
                        {/if}
                    </div>
                </div>

                <Separator />

                <!-- Mission Type -->
                <div>
                    <Label class="text-sm font-medium mb-2 block">Mission Type</Label>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
                        {#each missionOptions as opt}
                            <Button
                                variant={mission === opt.type ? 'default' : 'outline'}
                                class="gap-2 {!opt.enabled ? 'opacity-50' : ''}"
                                disabled={!opt.enabled}
                                onclick={() => opt.enabled && (mission = opt.type)}
                            >
                                <span>{getMissionIcon(opt.type)}</span>
                                <span>{getMissionLabel(opt.type)}</span>
                            </Button>
                        {/each}
                    </div>
                    <p class="text-xs text-muted-foreground mt-2">
                        {#if mission === 'raid'}
                            Steal resources. Troops can flee if outnumbered.
                        {:else if mission === 'attack'}
                            Full attack. Kill enemy troops and steal resources.
                        {:else if mission === 'scout'}
                            Reconnaissance mission to gather information.
                        {:else if mission === 'support'}
                            Send troops to defend an allied village.
                        {/if}
                    </p>
                </div>

                <Separator />

                <!-- Troop Selection -->
                <TroopSelector
                    {troops}
                    {definitions}
                    {selectedTroops}
                    onSelectionChange={(selected) => selectedTroops = selected}
                />

                <!-- Travel Time and Summary -->
                {#if totalTroops > 0}
                    <Separator />
                    <Card class="p-4 bg-muted/50">
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                            <div>
                                <span class="text-muted-foreground block">Troops</span>
                                <span class="font-bold text-lg">{totalTroops}</span>
                            </div>
                            <div>
                                <span class="text-muted-foreground block">Travel Time</span>
                                <span class="font-bold text-lg">{formatTravelTime(travelTimeSeconds())}</span>
                            </div>
                            <div>
                                <span class="text-muted-foreground block">Carry Capacity</span>
                                <span class="font-bold text-lg">📦 {totalCarryCapacity}</span>
                            </div>
                            <div>
                                <span class="text-muted-foreground block">Arrival</span>
                                <span class="font-bold text-sm">
                                    {new Date(Date.now() + travelTimeSeconds() * 1000).toLocaleTimeString()}
                                </span>
                            </div>
                        </div>
                    </Card>
                {/if}

                <!-- Error -->
                {#if error}
                    <Card class="p-3 bg-destructive/10 text-destructive">
                        <p class="text-sm">{error}</p>
                    </Card>
                {/if}
            {/if}
        </div>

        <Dialog.Footer class="flex-col sm:flex-row gap-2">
            <Button variant="outline" onclick={() => open = false} disabled={loading}>
                Cancel
            </Button>
            <Button
                onclick={handleSend}
                disabled={!isValid}
                class="gap-2"
            >
                {#if loading}
                    <span class="animate-spin">⏳</span>
                    Sending...
                {:else}
                    <span>{getMissionIcon(mission)}</span>
                    Send {getMissionLabel(mission)}
                {/if}
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
