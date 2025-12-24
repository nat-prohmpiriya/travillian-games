<script lang="ts">
    import * as Dialog from '$lib/components/ui/dialog';
    import { Button } from '$lib/components/ui/button';
    import BattleReportCard from '$lib/components/game/BattleReportCard.svelte';
    import ScoutReportCard from '$lib/components/game/ScoutReportCard.svelte';
    import { armyStore, type BattleReport, type ScoutReport } from '$lib/stores/army';
    import { authStore } from '$lib/stores/auth';

    interface Props {
        open: boolean;
    }

    let { open = $bindable(false) }: Props = $props();

    type TabType = 'battle' | 'scout';
    let activeTab = $state<TabType>('battle');

    // Store state
    let armyState = $state(armyStore);
    let battleReports = $derived($armyState.reports);
    let scoutReports = $derived($armyState.scoutReports);
    let loading = $derived($armyState.loading);

    let authState = $state(authStore);
    let backendUser = $derived($authState.backendUser);

    // Unread counts
    const unreadBattleCount = $derived(
        battleReports.filter(r => !r.is_read).length
    );
    const unreadScoutCount = $derived(
        scoutReports.filter(r => !r.is_read).length
    );

    // Load reports when modal opens
    $effect(() => {
        if (open) {
            loadReports();
        }
    });

    async function loadReports() {
        try {
            await Promise.all([
                armyStore.loadReports(),
                armyStore.loadScoutReports(),
            ]);
        } catch (error) {
            console.error('Failed to load reports:', error);
        }
    }

    // Check if current user is attacker
    function isAttacker(report: BattleReport | ScoutReport): boolean {
        return backendUser?.id === report.attacker_player_id;
    }

    // Handle mark as read
    async function handleMarkBattleRead(reportId: string) {
        await armyStore.markRead(reportId);
    }

    async function handleMarkScoutRead(reportId: string) {
        await armyStore.markScoutReportRead(reportId);
    }

    // Handle view details (placeholder for now)
    function handleViewDetails(reportId: string) {
        console.log('View details:', reportId);
        // TODO: Could open a detailed modal or navigate to report page
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Content class="sm:max-w-2xl max-h-[85vh] flex flex-col">
        <Dialog.Header>
            <div class="flex items-center gap-3">
                <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
                    📜
                </div>
                <div>
                    <Dialog.Title class="text-xl">Reports</Dialog.Title>
                    <Dialog.Description>
                        View your battle and scout reports
                    </Dialog.Description>
                </div>
            </div>
        </Dialog.Header>

        <!-- Tabs -->
        <div class="flex gap-2 mt-4">
            <Button
                variant={activeTab === 'battle' ? 'default' : 'outline'}
                onclick={() => activeTab = 'battle'}
                class="flex-1 gap-2"
            >
                <span>⚔️</span>
                <span>Battle Reports</span>
                {#if unreadBattleCount > 0}
                    <span class="px-1.5 py-0.5 text-xs bg-destructive text-destructive-foreground rounded-full">
                        {unreadBattleCount}
                    </span>
                {/if}
            </Button>
            <Button
                variant={activeTab === 'scout' ? 'default' : 'outline'}
                onclick={() => activeTab = 'scout'}
                class="flex-1 gap-2"
            >
                <span>👁️</span>
                <span>Scout Reports</span>
                {#if unreadScoutCount > 0}
                    <span class="px-1.5 py-0.5 text-xs bg-destructive text-destructive-foreground rounded-full">
                        {unreadScoutCount}
                    </span>
                {/if}
            </Button>
        </div>

        <!-- Report List -->
        <div class="flex-1 overflow-y-auto mt-4 space-y-3 min-h-[300px]">
            {#if loading}
                <div class="flex items-center justify-center py-12">
                    <span class="animate-spin text-2xl mr-2">⏳</span>
                    <span>Loading reports...</span>
                </div>
            {:else if activeTab === 'battle'}
                {#if battleReports.length === 0}
                    <div class="text-center py-12 text-muted-foreground">
                        <p class="text-4xl mb-2">⚔️</p>
                        <p>No battle reports yet</p>
                        <p class="text-sm">Reports will appear here after battles</p>
                    </div>
                {:else}
                    {#each battleReports as report (report.id)}
                        <BattleReportCard
                            {report}
                            isAttacker={isAttacker(report)}
                            onMarkRead={() => handleMarkBattleRead(report.id)}
                            onViewDetails={() => handleViewDetails(report.id)}
                        />
                    {/each}
                {/if}
            {:else}
                {#if scoutReports.length === 0}
                    <div class="text-center py-12 text-muted-foreground">
                        <p class="text-4xl mb-2">👁️</p>
                        <p>No scout reports yet</p>
                        <p class="text-sm">Reports will appear here after scouting missions</p>
                    </div>
                {:else}
                    {#each scoutReports as report (report.id)}
                        <ScoutReportCard
                            {report}
                            isAttacker={isAttacker(report)}
                            onMarkRead={() => handleMarkScoutRead(report.id)}
                            onViewDetails={() => handleViewDetails(report.id)}
                        />
                    {/each}
                {/if}
            {/if}
        </div>

        <!-- Footer -->
        <Dialog.Footer class="mt-4">
            <Button variant="outline" onclick={() => open = false}>
                Close
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
