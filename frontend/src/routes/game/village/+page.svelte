<script lang="ts">
  import { onMount } from 'svelte';
  import { Card } from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import BuildingSlot from '$lib/components/game/BuildingSlot.svelte';
  import BuildingDetailModal from '$lib/components/modals/BuildingDetailModal.svelte';
  import BuildMenuModal from '$lib/components/modals/BuildMenuModal.svelte';
  import TrainingModal from '$lib/components/modals/TrainingModal.svelte';
  import { villageStore, getBuildingBySlot, type Building, type BuildingType } from '$lib/stores/village';
  import { goto } from '$app/navigation';
  import ArmyMovementPanel from '$lib/components/game/ArmyMovementPanel.svelte';
  import StationedTroopsPanel from '$lib/components/game/StationedTroopsPanel.svelte';

  type ViewMode = 'village' | 'resources';

  // UI States
  let viewMode = $state<ViewMode>('village');
  let showBuildingDetail = $state(false);
  let showBuildMenu = $state(false);
  let showTrainingModal = $state(false);
  let selectedBuilding = $state<Building | null>(null);
  let selectedSlot = $state(0);
  let selectedIsResource = $state(false);
  let trainingBuildingType = $state<BuildingType>('barracks');
  let trainingBuildingLevel = $state(1);
  let actionLoading = $state(false);
  let actionError = $state('');

  // Subscribe to stores
  let villageState = $state(villageStore);
  let currentVillage = $derived($villageState.currentVillage);
  let buildings = $derived($villageState.buildings);
  let buildQueue = $derived($villageState.buildQueue);
  let loading = $derived($villageState.loading);
  let storeError = $derived($villageState.error);

  // Village center slots (22 slots for infrastructure buildings)
  const VILLAGE_SLOTS = Array.from({ length: 22 }, (_, i) => i + 1);

  // Resource field slots (18 slots: 4 wood, 4 clay, 4 iron, 6 crop)
  const RESOURCE_SLOTS = {
    wood: [101, 102, 103, 104],
    clay: [105, 106, 107, 108],
    iron: [109, 110, 111, 112],
    crop: [113, 114, 115, 116, 117, 118],
  };

  // Get building for a specific slot
  function getBuildingForSlot(slot: number): Building | null {
    return getBuildingBySlot(buildings, slot);
  }

  // Convert Building to format expected by BuildingSlot component
  function toBuildingSlotFormat(building: Building | null) {
    if (!building) return null;
    return {
      id: building.id,
      type: building.building_type,
      level: building.level,
      isUpgrading: building.is_upgrading,
      upgradeEndsAt: building.upgrade_ends_at ? new Date(building.upgrade_ends_at) : undefined,
    };
  }

  function formatTimeRemaining(endTime: Date): string {
    const diff = endTime.getTime() - Date.now();
    if (diff <= 0) return 'Done';

    const hours = Math.floor(diff / 3600000);
    const minutes = Math.floor((diff % 3600000) / 60000);
    const seconds = Math.floor((diff % 60000) / 1000);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  }

  function formatBuildingName(type: BuildingType): string {
    return type
      .split('_')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');
  }

  function handleBuildingClick(building: Building | null, slot: number, isResource: boolean) {
    selectedSlot = slot;
    selectedIsResource = isResource;
    actionError = '';

    if (building) {
      selectedBuilding = building;
      showBuildingDetail = true;
    } else {
      showBuildMenu = true;
    }
  }

  async function handleUpgrade() {
    if (!currentVillage || !selectedBuilding) return;

    actionLoading = true;
    actionError = '';

    try {
      await villageStore.upgrade(currentVillage.id, selectedBuilding.slot);
      showBuildingDetail = false;
    } catch (err: any) {
      actionError = err.message || 'Failed to upgrade building';
    } finally {
      actionLoading = false;
    }
  }

  async function handleDemolish() {
    if (!currentVillage || !selectedBuilding) return;

    actionLoading = true;
    actionError = '';

    try {
      await villageStore.demolish(currentVillage.id, selectedBuilding.slot);
      showBuildingDetail = false;
    } catch (err: any) {
      actionError = err.message || 'Failed to demolish building';
    } finally {
      actionLoading = false;
    }
  }

  async function handleBuild(type: BuildingType) {
    if (!currentVillage) return;

    actionLoading = true;
    actionError = '';

    try {
      await villageStore.build(currentVillage.id, selectedSlot, type);
      showBuildMenu = false;
    } catch (err: any) {
      actionError = err.message || 'Failed to build';
    } finally {
      actionLoading = false;
    }
  }

  function handleTrainTroops() {
    if (!selectedBuilding) return;

    trainingBuildingType = selectedBuilding.building_type;
    trainingBuildingLevel = selectedBuilding.level;
    showTrainingModal = true;
  }

  function openTrainingModalForBuilding(buildingType: BuildingType) {
    // Find the building of this type in the village
    const building = buildings.find(b => b.building_type === buildingType);
    if (building) {
      trainingBuildingType = buildingType;
      trainingBuildingLevel = building.level;
      showTrainingModal = true;
    }
  }

  // Load village data on mount
  onMount(() => {
    let refreshInterval: ReturnType<typeof setInterval>;

    const loadData = async () => {
      try {
        // Load villages list
        const villages = await villageStore.loadVillages();

        if (villages.length === 0) {
          // No villages, redirect to onboarding
          goto('/onboarding');
          return;
        }

        // Load first village details
        await villageStore.loadVillage(villages[0].id);

        // Set up interval to refresh build queue
        refreshInterval = setInterval(() => {
          if (currentVillage) {
            villageStore.refreshBuildQueue(currentVillage.id);
          }
        }, 10000); // Refresh every 10 seconds
      } catch (error) {
        console.error('Failed to load village:', error);
      }
    };

    loadData();

    return () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    };
  });
</script>

<svelte:head>
  <title>Village - Tusk & Horn</title>
</svelte:head>

{#if loading && !currentVillage}
  <div class="flex items-center justify-center min-h-[60vh]">
    <div class="text-center">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
      <p class="text-muted-foreground">Loading village...</p>
    </div>
  </div>
{:else if storeError && !currentVillage}
  <div class="flex items-center justify-center min-h-[60vh]">
    <div class="text-center">
      <p class="text-destructive mb-4">{storeError}</p>
      <Button onclick={() => location.reload()}>Retry</Button>
    </div>
  </div>
{:else if currentVillage}
  <div class="container mx-auto px-4 py-6 max-w-6xl">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
      <div>
        <h1 class="text-2xl font-bold">{currentVillage.name}</h1>
        <p class="text-muted-foreground text-sm">Coordinates: ({currentVillage.x}|{currentVillage.y})</p>
      </div>

      <!-- View Toggle -->
      <div class="flex items-center gap-2 bg-muted p-1 rounded-lg">
        <Button
          variant={viewMode === 'village' ? 'default' : 'ghost'}
          size="sm"
          onclick={() => viewMode = 'village'}
          class="gap-2"
        >
          <span>🏘️</span>
          <span>Village</span>
        </Button>
        <Button
          variant={viewMode === 'resources' ? 'default' : 'ghost'}
          size="sm"
          onclick={() => viewMode = 'resources'}
          class="gap-2"
        >
          <span>🌾</span>
          <span>Resources</span>
        </Button>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
      <!-- Building Grid -->
      <div class="lg:col-span-3">
        {#if viewMode === 'village'}
          <!-- Village Center Grid - 22 slots -->
          <Card class="p-4">
            <h2 class="text-lg font-semibold mb-4">Village Center</h2>
            <div class="grid grid-cols-4 sm:grid-cols-5 md:grid-cols-6 gap-2 sm:gap-3">
              {#each VILLAGE_SLOTS as slot}
                {@const building = getBuildingForSlot(slot)}
                <BuildingSlot
                  building={toBuildingSlotFormat(building)}
                  {slot}
                  onclick={() => handleBuildingClick(building, slot, false)}
                />
              {/each}
            </div>
          </Card>
        {:else}
          <!-- Resource Fields Grid - 18 slots -->
          <Card class="p-4">
            <h2 class="text-lg font-semibold mb-4">Resource Fields</h2>

            <div class="space-y-6">
              <!-- Wood -->
              <div>
                <div class="flex items-center gap-2 mb-3">
                  <span class="text-lg">🪵</span>
                  <span class="font-medium">Woodcutters</span>
                </div>
                <div class="grid grid-cols-4 gap-2 sm:gap-3">
                  {#each RESOURCE_SLOTS.wood as slot}
                    {@const building = getBuildingForSlot(slot)}
                    <BuildingSlot
                      building={toBuildingSlotFormat(building)}
                      {slot}
                      isResourceField={true}
                      onclick={() => handleBuildingClick(building, slot, true)}
                    />
                  {/each}
                </div>
              </div>

              <!-- Clay -->
              <div>
                <div class="flex items-center gap-2 mb-3">
                  <span class="text-lg">🧱</span>
                  <span class="font-medium">Clay Pits</span>
                </div>
                <div class="grid grid-cols-4 gap-2 sm:gap-3">
                  {#each RESOURCE_SLOTS.clay as slot}
                    {@const building = getBuildingForSlot(slot)}
                    <BuildingSlot
                      building={toBuildingSlotFormat(building)}
                      {slot}
                      isResourceField={true}
                      onclick={() => handleBuildingClick(building, slot, true)}
                    />
                  {/each}
                </div>
              </div>

              <!-- Iron -->
              <div>
                <div class="flex items-center gap-2 mb-3">
                  <span class="text-lg">⛏️</span>
                  <span class="font-medium">Iron Mines</span>
                </div>
                <div class="grid grid-cols-4 gap-2 sm:gap-3">
                  {#each RESOURCE_SLOTS.iron as slot}
                    {@const building = getBuildingForSlot(slot)}
                    <BuildingSlot
                      building={toBuildingSlotFormat(building)}
                      {slot}
                      isResourceField={true}
                      onclick={() => handleBuildingClick(building, slot, true)}
                    />
                  {/each}
                </div>
              </div>

              <!-- Crop -->
              <div>
                <div class="flex items-center gap-2 mb-3">
                  <span class="text-lg">🌾</span>
                  <span class="font-medium">Crop Fields</span>
                </div>
                <div class="grid grid-cols-3 sm:grid-cols-6 gap-2 sm:gap-3">
                  {#each RESOURCE_SLOTS.crop as slot}
                    {@const building = getBuildingForSlot(slot)}
                    <BuildingSlot
                      building={toBuildingSlotFormat(building)}
                      {slot}
                      isResourceField={true}
                      onclick={() => handleBuildingClick(building, slot, true)}
                    />
                  {/each}
                </div>
              </div>
            </div>
          </Card>
        {/if}
      </div>

      <!-- Sidebar -->
      <div class="space-y-4">
        <!-- Build Queue -->
        <Card class="p-4">
          <h3 class="font-semibold mb-3 flex items-center gap-2">
            <span>🔨</span>
            Build Queue
          </h3>
          {#if buildQueue.length > 0}
            <div class="space-y-3">
              {#each buildQueue as item, index}
                <div class="flex items-center justify-between p-2 bg-muted rounded-lg">
                  <div>
                    <p class="font-medium text-sm">{formatBuildingName(item.building_type)}</p>
                    <p class="text-xs text-muted-foreground">Level {item.level + 1}</p>
                  </div>
                  <div class="text-right">
                    {#if item.upgrade_ends_at}
                      <p class="text-sm font-mono text-amber-600">
                        {formatTimeRemaining(new Date(item.upgrade_ends_at))}
                      </p>
                    {/if}
                    {#if index === 0}
                      <span class="text-xs text-emerald-600">Building...</span>
                    {:else}
                      <span class="text-xs text-muted-foreground">Queued</span>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-muted-foreground text-center py-4">
              No buildings in queue
            </p>
          {/if}
        </Card>

        <!-- Army Movements -->
        <ArmyMovementPanel villageId={currentVillage.id} />

        <!-- Support Troops -->
        <StationedTroopsPanel villageId={currentVillage.id} />

        <!-- Quick Actions -->
        <Card class="p-4">
          <h3 class="font-semibold mb-3 flex items-center gap-2">
            <span>⚡</span>
            Quick Actions
          </h3>
          <div class="space-y-2">
            <Button
              variant="outline"
              class="w-full justify-start gap-2"
              size="sm"
              onclick={() => openTrainingModalForBuilding('barracks')}
              disabled={!buildings.some(b => b.building_type === 'barracks')}
            >
              <span>⚔️</span>
              Train Troops
            </Button>
            <Button variant="outline" class="w-full justify-start gap-2" size="sm">
              <span>🏪</span>
              Open Market
            </Button>
            <Button variant="outline" class="w-full justify-start gap-2" size="sm">
              <span>📜</span>
              Village Overview
            </Button>
          </div>
        </Card>

        <!-- Village Stats -->
        <Card class="p-4">
          <h3 class="font-semibold mb-3 flex items-center gap-2">
            <span>📊</span>
            Village Stats
          </h3>
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-muted-foreground">Population</span>
              <span class="font-medium">{currentVillage.population}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-muted-foreground">Culture Points</span>
              <span class="font-medium">{currentVillage.culture_points}/day</span>
            </div>
            <div class="flex justify-between">
              <span class="text-muted-foreground">Loyalty</span>
              <span class="font-medium text-emerald-600">{currentVillage.loyalty}%</span>
            </div>
            {#if currentVillage.production}
              <div class="pt-2 border-t mt-2">
                <p class="text-xs text-muted-foreground mb-1">Production/hour</p>
                <div class="grid grid-cols-2 gap-1 text-xs">
                  <span>🪵 {currentVillage.production.wood_per_hour}</span>
                  <span>🧱 {currentVillage.production.clay_per_hour}</span>
                  <span>⛏️ {currentVillage.production.iron_per_hour}</span>
                  <span>🌾 {currentVillage.production.net_crop_per_hour}</span>
                </div>
              </div>
            {/if}
          </div>
        </Card>
      </div>
    </div>
  </div>
{/if}

<!-- Modals -->
<BuildingDetailModal
  bind:open={showBuildingDetail}
  building={selectedBuilding ? {
    id: selectedBuilding.id,
    type: selectedBuilding.building_type,
    level: selectedBuilding.level,
    isUpgrading: selectedBuilding.is_upgrading,
    upgradeEndsAt: selectedBuilding.upgrade_ends_at ? new Date(selectedBuilding.upgrade_ends_at) : undefined,
  } : null}
  villageId={currentVillage?.id || ''}
  villageX={currentVillage?.x || 0}
  villageY={currentVillage?.y || 0}
  villageResources={currentVillage ? {
    wood: currentVillage.wood,
    clay: currentVillage.clay,
    iron: currentVillage.iron,
    crop: currentVillage.crop,
  } : undefined}
  onUpgrade={handleUpgrade}
  onDemolish={handleDemolish}
  onTrainTroops={handleTrainTroops}
  loading={actionLoading}
  error={actionError}
/>

<BuildMenuModal
  bind:open={showBuildMenu}
  slot={selectedSlot}
  isResourceField={selectedIsResource}
  villageResources={currentVillage ? {
    wood: currentVillage.wood,
    clay: currentVillage.clay,
    iron: currentVillage.iron,
    crop: currentVillage.crop,
  } : undefined}
  onBuild={handleBuild}
  loading={actionLoading}
  error={actionError}
/>

<TrainingModal
  bind:open={showTrainingModal}
  villageId={currentVillage?.id || ''}
  buildingType={trainingBuildingType}
  buildingLevel={trainingBuildingLevel}
  villageResources={currentVillage ? {
    wood: currentVillage.wood,
    clay: currentVillage.clay,
    iron: currentVillage.iron,
    crop: currentVillage.crop,
  } : { wood: 0, clay: 0, iron: 0, crop: 0 }}
/>
