<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import {
        armyStore,
        type Army,
        getTotalTroops,
    } from '$lib/stores/army';

    interface Props {
        villageId: string;
    }

    let { villageId }: Props = $props();

    // Store state
    let armyState = $state(armyStore);
    let stationedTroops = $derived($armyState.stationedTroops);
    let supportSent = $derived($armyState.supportSent);
    let loading = $derived($armyState.loading);

    // Load data when villageId changes
    $effect(() => {
        if (villageId) {
            loadData();
        }
    });

    async function loadData() {
        try {
            await Promise.all([
                armyStore.loadStationed(villageId),
                armyStore.loadSupportSent(),
            ]);
        } catch (error) {
            console.error('Failed to load stationed troops:', error);
        }
    }

    async function handleRecall(armyId: string) {
        try {
            await armyStore.recallSupport(armyId);
        } catch (error) {
            console.error('Failed to recall troops:', error);
        }
    }

    // Format village coordinates
    function formatLocation(army: Army): string {
        return `(${army.to_x}|${army.to_y})`;
    }

    const totalStationedCount = $derived(stationedTroops.length);
    const totalSupportSentCount = $derived(supportSent.length);
    const hasAny = $derived(totalStationedCount > 0 || totalSupportSentCount > 0);
</script>

<Card class="p-4">
    <h3 class="font-semibold mb-3 flex items-center gap-2">
        <span>🛡️</span>
        Support Troops
        {#if hasAny}
            <span class="px-1.5 py-0.5 text-xs bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded-full">
                {totalStationedCount + totalSupportSentCount}
            </span>
        {/if}
    </h3>

    {#if loading && !hasAny}
        <div class="text-center py-4">
            <span class="animate-spin text-lg">⏳</span>
        </div>
    {:else if !hasAny}
        <p class="text-sm text-muted-foreground text-center py-4">
            No support troops
        </p>
    {:else}
        <div class="space-y-4">
            <!-- Support received (stationed at this village) -->
            {#if totalStationedCount > 0}
                <div>
                    <p class="text-xs text-muted-foreground mb-2 flex items-center gap-1">
                        <span>📥</span>
                        Received ({totalStationedCount})
                    </p>
                    <div class="space-y-2">
                        {#each stationedTroops as army (army.id)}
                            <div class="p-2 bg-blue-50 dark:bg-blue-950 rounded-lg border-l-2 border-l-blue-500">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="text-lg">🛡️</span>
                                        <div>
                                            <p class="text-sm font-medium">
                                                Support
                                            </p>
                                            <p class="text-xs text-muted-foreground">
                                                From ({army.to_x}|{army.to_y})
                                            </p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm font-medium text-blue-600">
                                            🪖 {getTotalTroops(army.troops)}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}

            <!-- Support sent (to other villages) -->
            {#if totalSupportSentCount > 0}
                <div>
                    <p class="text-xs text-muted-foreground mb-2 flex items-center gap-1">
                        <span>📤</span>
                        Sent ({totalSupportSentCount})
                    </p>
                    <div class="space-y-2">
                        {#each supportSent as army (army.id)}
                            <div class="p-2 bg-muted rounded-lg">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="text-lg">🛡️</span>
                                        <div>
                                            <p class="text-sm font-medium">
                                                Support
                                            </p>
                                            <p class="text-xs text-muted-foreground">
                                                At {formatLocation(army)}
                                            </p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-2">
                                        <p class="text-sm text-muted-foreground">
                                            🪖 {getTotalTroops(army.troops)}
                                        </p>
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => handleRecall(army.id)}
                                            disabled={loading}
                                        >
                                            Recall
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    {/if}
</Card>
