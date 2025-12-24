<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import {
        type ScoutReport,
        type CarriedResources,
        getTotalResources,
        getTotalTroops,
    } from '$lib/stores/army';
    import { formatTroopName, type TroopType } from '$lib/stores/troop';

    interface Props {
        report: ScoutReport;
        isAttacker: boolean;
        onMarkRead?: () => void;
        onViewDetails?: () => void;
    }

    let { report, isAttacker, onMarkRead, onViewDetails }: Props = $props();

    // Calculate totals
    const scoutedTroopsTotal = $derived(
        report.scouted_troops ? getTotalTroops(report.scouted_troops) : 0
    );
    const scoutedResourcesTotal = $derived(
        report.scouted_resources ? getTotalResources(report.scouted_resources) : 0
    );

    // Format date
    function formatDate(dateStr: string): string {
        const date = new Date(dateStr);
        const now = new Date();
        const diff = now.getTime() - date.getTime();
        const hours = Math.floor(diff / (1000 * 60 * 60));
        const days = Math.floor(hours / 24);

        if (days > 0) {
            return `${days}d ago`;
        }
        if (hours > 0) {
            return `${hours}h ago`;
        }
        return 'Just now';
    }

    // Format resources
    function formatResources(resources: CarriedResources): string {
        const parts = [];
        if (resources.wood > 0) parts.push(`🪵 ${resources.wood}`);
        if (resources.clay > 0) parts.push(`🧱 ${resources.clay}`);
        if (resources.iron > 0) parts.push(`⛏️ ${resources.iron}`);
        if (resources.crop > 0) parts.push(`🌾 ${resources.crop}`);
        return parts.join(' | ');
    }
</script>

<Card class="p-4 {!report.is_read ? 'border-l-4 border-l-primary bg-primary/5' : ''}">
    <div class="flex items-start justify-between">
        <!-- Left: Mission and Result -->
        <div class="flex items-start gap-3">
            <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center text-2xl">
                👁️
            </div>
            <div>
                <div class="flex items-center gap-2">
                    <span class="font-medium">Scout</span>
                    <span class="text-sm font-bold {report.success ? 'text-green-600' : 'text-red-600'}">
                        {report.success ? 'Success' : 'Failed'}
                    </span>
                    {#if !report.is_read}
                        <span class="px-1.5 py-0.5 text-xs bg-primary text-primary-foreground rounded">NEW</span>
                    {/if}
                </div>
                <p class="text-sm text-muted-foreground">
                    {isAttacker ? 'Your scouts reported' : 'Enemy scouts detected'} - {formatDate(report.occurred_at)}
                </p>
            </div>
        </div>

        <!-- Right: Quick Stats -->
        <div class="text-right text-sm">
            {#if isAttacker && report.success}
                <p class="text-green-600 font-medium">
                    Intel gathered
                </p>
            {:else if !isAttacker}
                <p class="text-amber-600 font-medium">
                    You were scouted
                </p>
            {/if}
        </div>
    </div>

    <!-- Scout Summary -->
    <div class="mt-4 grid grid-cols-2 gap-4 text-sm">
        <!-- Scout Combat -->
        <div class="space-y-1">
            <p class="font-medium {isAttacker ? 'text-primary' : 'text-muted-foreground'}">
                {isAttacker ? '👤 Your Scouts' : '👤 Enemy Scouts'}
            </p>
            <p class="text-muted-foreground">
                🕵️ {report.attacker_scouts} sent
            </p>
            {#if report.attacker_scouts_lost > 0}
                <p class="text-red-500">
                    💀 -{report.attacker_scouts_lost} lost
                </p>
            {:else}
                <p class="text-green-500">
                    No losses
                </p>
            {/if}
        </div>

        <!-- Defender Side -->
        <div class="space-y-1 text-right">
            <p class="font-medium {!isAttacker ? 'text-primary' : 'text-muted-foreground'}">
                {!isAttacker ? '🛡️ Your Defense' : '🛡️ Defender'}
            </p>
            <p class="text-muted-foreground">
                🕵️ {report.defender_scouts} scouts
            </p>
            {#if report.defender_scouts_lost > 0}
                <p class="text-red-500">
                    💀 -{report.defender_scouts_lost} lost
                </p>
            {:else if report.defender_scouts > 0}
                <p class="text-green-500">
                    No losses
                </p>
            {:else}
                <p class="text-amber-500">
                    Undefended
                </p>
            {/if}
        </div>
    </div>

    <!-- Scouted Information (only for successful attacker) -->
    {#if isAttacker && report.success && (report.scouted_resources || report.scouted_troops)}
        <div class="mt-4 p-3 bg-muted/50 rounded-lg">
            <p class="text-sm font-medium mb-2">Intel Report:</p>

            {#if report.scouted_resources}
                <div class="text-sm mb-2">
                    <span class="text-muted-foreground">Resources: </span>
                    <span>{formatResources(report.scouted_resources)}</span>
                </div>
            {/if}

            {#if report.scouted_troops && scoutedTroopsTotal > 0}
                <div class="text-sm">
                    <span class="text-muted-foreground">Troops: </span>
                    <span class="text-amber-600">⚔️ {scoutedTroopsTotal} total</span>
                    <div class="mt-1 text-xs text-muted-foreground">
                        {#each Object.entries(report.scouted_troops) as [type, count]}
                            {#if count && count > 0}
                                <span class="mr-2">{formatTroopName(type as TroopType)}: {count}</span>
                            {/if}
                        {/each}
                    </div>
                </div>
            {:else if report.scouted_troops}
                <div class="text-sm">
                    <span class="text-muted-foreground">Troops: </span>
                    <span class="text-green-600">No troops in village</span>
                </div>
            {/if}
        </div>
    {:else if isAttacker && !report.success}
        <div class="mt-4 p-3 bg-red-50 dark:bg-red-950/20 rounded-lg">
            <p class="text-sm text-red-600">
                Scout mission failed. No intel gathered.
            </p>
        </div>
    {/if}

    <!-- Actions -->
    <div class="mt-4 flex items-center justify-between">
        <Button variant="ghost" size="sm" onclick={onViewDetails}>
            View Details
        </Button>
        {#if !report.is_read}
            <Button variant="outline" size="sm" onclick={onMarkRead}>
                Mark as Read
            </Button>
        {/if}
    </div>
</Card>
