<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import { Input } from '$lib/components/ui/input';
    import { Button } from '$lib/components/ui/button';
    import {
        type Troop,
        type TroopType,
        type TroopDefinition,
        formatTroopName,
        getTroopIcon,
    } from '$lib/stores/troop';
    import type { TroopCounts } from '$lib/stores/army';

    interface Props {
        troops: Troop[];
        definitions: TroopDefinition[];
        selectedTroops: TroopCounts;
        onSelectionChange: (selected: TroopCounts) => void;
    }

    let { troops, definitions, selectedTroops, onSelectionChange }: Props = $props();

    // Get definition for a troop type
    function getDefinition(type: TroopType): TroopDefinition | undefined {
        return definitions.find(d => d.troop_type === type);
    }

    // Update selection
    function updateSelection(type: TroopType, count: number) {
        const troop = troops.find(t => t.troop_type === type);
        const maxAvailable = troop?.in_village || 0;
        const validCount = Math.max(0, Math.min(count, maxAvailable));

        const newSelection = { ...selectedTroops };
        if (validCount > 0) {
            newSelection[type] = validCount;
        } else {
            delete newSelection[type];
        }
        onSelectionChange(newSelection);
    }

    // Set all troops of a type
    function setAll(type: TroopType) {
        const troop = troops.find(t => t.troop_type === type);
        if (troop) {
            updateSelection(type, troop.in_village);
        }
    }

    // Clear all selections
    function clearAll() {
        onSelectionChange({});
    }

    // Select all available troops
    function selectAllTroops() {
        const newSelection: TroopCounts = {};
        for (const troop of troops) {
            if (troop.in_village > 0) {
                newSelection[troop.troop_type] = troop.in_village;
            }
        }
        onSelectionChange(newSelection);
    }

    // Calculate total troops selected
    const totalSelected = $derived(
        Object.values(selectedTroops).reduce((sum, count) => sum + (count || 0), 0)
    );

    // Calculate total carry capacity
    const totalCarryCapacity = $derived(
        Object.entries(selectedTroops).reduce((sum, [type, count]) => {
            const def = getDefinition(type as TroopType);
            return sum + (def ? def.carry_capacity * count : 0);
        }, 0)
    );

    // Calculate slowest speed (for travel time)
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
        return minSpeed === Infinity ? 0 : minSpeed;
    });
</script>

<div class="space-y-4">
    <!-- Quick Actions -->
    <div class="flex items-center justify-between">
        <h4 class="font-medium text-sm">Select Troops</h4>
        <div class="flex gap-2">
            <Button variant="outline" size="sm" onclick={selectAllTroops}>
                Select All
            </Button>
            <Button variant="ghost" size="sm" onclick={clearAll}>
                Clear
            </Button>
        </div>
    </div>

    <!-- Troop List -->
    {#if troops.length === 0}
        <Card class="p-4">
            <p class="text-center text-muted-foreground">No troops available in this village.</p>
        </Card>
    {:else}
        <div class="space-y-2">
            {#each troops.filter(t => t.in_village > 0) as troop}
                {@const def = getDefinition(troop.troop_type)}
                {@const icon = getTroopIcon(troop.troop_type)}
                {@const selected = selectedTroops[troop.troop_type] || 0}

                <Card class="p-3">
                    <div class="flex items-center gap-3">
                        <!-- Icon and Name -->
                        <div class="w-10 h-10 rounded-lg bg-muted flex items-center justify-center text-xl">
                            {icon}
                        </div>
                        <div class="flex-1">
                            <p class="font-medium text-sm">{formatTroopName(troop.troop_type)}</p>
                            <p class="text-xs text-muted-foreground">
                                Available: {troop.in_village}
                                {#if def}
                                    | ⚔️ {def.attack} | 📦 {def.carry_capacity}
                                {/if}
                            </p>
                        </div>

                        <!-- Selection Input -->
                        <div class="flex items-center gap-2">
                            <Input
                                type="number"
                                min={0}
                                max={troop.in_village}
                                value={selected}
                                oninput={(e) => updateSelection(troop.troop_type, parseInt((e.target as HTMLInputElement).value) || 0)}
                                class="w-20 h-8 text-center"
                            />
                            <Button
                                variant="outline"
                                size="sm"
                                class="h-8 px-2"
                                onclick={() => setAll(troop.troop_type)}
                            >
                                All
                            </Button>
                        </div>
                    </div>
                </Card>
            {/each}
        </div>
    {/if}

    <!-- Summary -->
    {#if totalSelected > 0}
        <Card class="p-3 bg-muted/50">
            <div class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-4">
                    <span>
                        <strong>{totalSelected}</strong> troops selected
                    </span>
                    <span class="text-muted-foreground">
                        📦 {totalCarryCapacity} carry capacity
                    </span>
                </div>
                <span class="text-muted-foreground">
                    💨 Speed: {slowestSpeed()} fields/h
                </span>
            </div>
        </Card>
    {/if}
</div>
