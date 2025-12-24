<script lang="ts">
    import { Card } from '$lib/components/ui/card';
    import { Button } from '$lib/components/ui/button';
    import {
        type BattleReport,
        getMissionLabel,
        getMissionIcon,
        getWinnerLabel,
        getWinnerColor,
        getTotalTroops,
        getTotalResources,
    } from '$lib/stores/army';
    import { formatTroopName, type TroopType } from '$lib/stores/troop';

    interface Props {
        report: BattleReport;
        isAttacker: boolean;
        onMarkRead?: () => void;
        onViewDetails?: () => void;
    }

    let { report, isAttacker, onMarkRead, onViewDetails }: Props = $props();

    // Calculate losses
    const attackerTotalTroops = $derived(getTotalTroops(report.attacker_troops));
    const defenderTotalTroops = $derived(getTotalTroops(report.defender_troops));
    const attackerTotalLosses = $derived(getTotalTroops(report.attacker_losses));
    const defenderTotalLosses = $derived(getTotalTroops(report.defender_losses));
    const resourcesStolen = $derived(getTotalResources(report.resources_stolen));

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
</script>

<Card class="p-4 {!report.is_read ? 'border-l-4 border-l-primary bg-primary/5' : ''}">
    <div class="flex items-start justify-between">
        <!-- Left: Mission and Result -->
        <div class="flex items-start gap-3">
            <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center text-2xl">
                {getMissionIcon(report.mission)}
            </div>
            <div>
                <div class="flex items-center gap-2">
                    <span class="font-medium">{getMissionLabel(report.mission)}</span>
                    <span class="text-sm {getWinnerColor(report.winner, isAttacker)} font-bold">
                        {getWinnerLabel(report.winner, isAttacker)}
                    </span>
                    {#if !report.is_read}
                        <span class="px-1.5 py-0.5 text-xs bg-primary text-primary-foreground rounded">NEW</span>
                    {/if}
                </div>
                <p class="text-sm text-muted-foreground">
                    {isAttacker ? 'You attacked' : 'You were attacked'} - {formatDate(report.occurred_at)}
                </p>
            </div>
        </div>

        <!-- Right: Quick Stats -->
        <div class="text-right text-sm">
            {#if resourcesStolen > 0}
                <p class="font-medium text-amber-600">
                    📦 +{resourcesStolen} stolen
                </p>
            {/if}
        </div>
    </div>

    <!-- Battle Summary -->
    <div class="mt-4 grid grid-cols-2 gap-4 text-sm">
        <!-- Attacker Side -->
        <div class="space-y-1">
            <p class="font-medium {isAttacker ? 'text-primary' : 'text-muted-foreground'}">
                {isAttacker ? '👤 Your Army' : '👤 Attacker'}
            </p>
            <p class="text-muted-foreground">
                ⚔️ {attackerTotalTroops} troops
            </p>
            <p class="text-red-500">
                💀 -{attackerTotalLosses} lost
            </p>
        </div>

        <!-- Defender Side -->
        <div class="space-y-1 text-right">
            <p class="font-medium {!isAttacker ? 'text-primary' : 'text-muted-foreground'}">
                {!isAttacker ? '🛡️ Your Defense' : '🛡️ Defender'}
            </p>
            <p class="text-muted-foreground">
                ⚔️ {defenderTotalTroops} troops
            </p>
            <p class="text-red-500">
                💀 -{defenderTotalLosses} lost
            </p>
        </div>
    </div>

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
