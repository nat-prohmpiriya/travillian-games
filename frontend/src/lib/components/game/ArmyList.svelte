<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import {
        type Army,
        getMissionLabel,
        getMissionIcon,
        getMissionColor,
        formatArrivalTime,
        getTotalTroops,
        getTotalResources,
    } from '$lib/stores/army';
    import { onMount, onDestroy } from 'svelte';

    interface Props {
        armies: Army[];
        type: 'outgoing' | 'incoming';
    }

    let { armies, type }: Props = $props();

    // Update every second for countdown
    let now = $state(Date.now());
    let interval: ReturnType<typeof setInterval>;

    onMount(() => {
        interval = setInterval(() => {
            now = Date.now();
        }, 1000);
    });

    onDestroy(() => {
        if (interval) clearInterval(interval);
    });

    // Get remaining time
    function getRemainingTime(arrivesAt: string): string {
        const arrival = new Date(arrivesAt).getTime();
        const diff = arrival - now;

        if (diff <= 0) return 'Arrived';

        const hours = Math.floor(diff / (1000 * 60 * 60));
        const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
        const seconds = Math.floor((diff % (1000 * 60)) / 1000);

        if (hours > 0) {
            return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
        }
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    }

    // Sort by arrival time
    const sortedArmies = $derived(
        [...armies].sort((a, b) =>
            new Date(a.arrives_at).getTime() - new Date(b.arrives_at).getTime()
        )
    );
</script>

<div class="space-y-3">
    {#if sortedArmies.length === 0}
        <Card class="p-4">
            <p class="text-center text-muted-foreground">
                {type === 'outgoing' ? 'No armies on the move' : 'No incoming armies'}
            </p>
        </Card>
    {:else}
        {#each sortedArmies as army}
            {@const totalTroops = getTotalTroops(army.troops)}
            {@const totalRes = getTotalResources(army.resources)}

            <Card class="p-4">
                <div class="flex items-center justify-between">
                    <!-- Mission Info -->
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center text-xl">
                            {getMissionIcon(army.mission)}
                        </div>
                        <div>
                            <p class="font-medium {getMissionColor(army.mission)}">
                                {getMissionLabel(army.mission)}
                                {#if army.is_returning}
                                    <span class="text-muted-foreground ml-1">(Returning)</span>
                                {/if}
                            </p>
                            <p class="text-sm text-muted-foreground">
                                {#if type === 'outgoing'}
                                    To ({army.to_x}, {army.to_y})
                                {:else}
                                    From village
                                {/if}
                            </p>
                        </div>
                    </div>

                    <!-- Troops and Resources -->
                    <div class="text-right">
                        <p class="font-medium">
                            <span class="text-muted-foreground mr-1">⚔️</span>
                            {totalTroops} troops
                        </p>
                        {#if totalRes > 0}
                            <p class="text-sm text-muted-foreground">
                                📦 {totalRes} resources
                            </p>
                        {/if}
                    </div>
                </div>

                <!-- Progress Bar -->
                {@const departed = new Date(army.departed_at).getTime()}
                {@const arrives = new Date(army.arrives_at).getTime()}
                {@const total = arrives - departed}
                {@const elapsed = now - departed}
                {@const progress = Math.min(100, Math.max(0, (elapsed / total) * 100))}

                <div class="mt-3">
                    <div class="flex items-center justify-between text-xs text-muted-foreground mb-1">
                        <span>
                            Departed: {new Date(army.departed_at).toLocaleTimeString()}
                        </span>
                        <span class="font-mono text-sm font-medium text-foreground">
                            {getRemainingTime(army.arrives_at)}
                        </span>
                    </div>

                    <div class="w-full bg-muted rounded-full h-2">
                        <div
                            class="h-2 rounded-full transition-all duration-1000 {army.is_returning ? 'bg-blue-500' : 'bg-amber-500'}"
                            style="width: {progress}%"
                        ></div>
                    </div>
                </div>
            </Card>
        {/each}
    {/if}
</div>
