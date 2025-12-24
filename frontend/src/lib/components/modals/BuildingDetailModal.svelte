<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Separator } from '$lib/components/ui/separator';
  import type { BuildingType } from '$lib/components/game/BuildingSlot.svelte';

  interface Building {
    id: string;
    type: BuildingType;
    level: number;
    isUpgrading: boolean;
    upgradeEndsAt?: Date;
  }

  interface UpgradeCost {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
    time: number; // seconds
  }

  interface VillageResources {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
  }

  interface Props {
    open: boolean;
    building: Building | null;
    villageResources?: VillageResources;
    onUpgrade?: () => void;
    onDemolish?: () => void;
    loading?: boolean;
    error?: string;
  }

  let { open = $bindable(false), building, villageResources, onUpgrade, onDemolish, loading = false, error = '' }: Props = $props();

  // Use village resources if provided, otherwise fallback to mock data
  const playerResources = $derived(villageResources || {
    wood: 1250,
    clay: 980,
    iron: 750,
    crop: 1100
  });

  // Building metadata
  const buildingInfo: Record<BuildingType, {
    name: string;
    icon: string;
    description: string;
    category: 'infrastructure' | 'military' | 'resource' | 'special';
  }> = {
    empty: { name: 'Empty Slot', icon: '➕', description: 'Build a new building here.', category: 'infrastructure' },
    main_building: { name: 'Main Building', icon: '🏛️', description: 'The heart of your village. Higher levels reduce construction time.', category: 'infrastructure' },
    warehouse: { name: 'Warehouse', icon: '📦', description: 'Stores wood, clay, and iron. Increase capacity by upgrading.', category: 'infrastructure' },
    granary: { name: 'Granary', icon: '🌾', description: 'Stores crop. Increase capacity by upgrading.', category: 'infrastructure' },
    barracks: { name: 'Barracks', icon: '⚔️', description: 'Train infantry units. Higher levels unlock more units and reduce training time.', category: 'military' },
    stable: { name: 'Stable', icon: '🐎', description: 'Train cavalry units. Higher levels unlock war elephants.', category: 'military' },
    workshop: { name: 'Workshop', icon: '🔧', description: 'Build siege weapons like rams and catapults.', category: 'military' },
    academy: { name: 'Academy', icon: '📚', description: 'Research new technologies and unit upgrades.', category: 'military' },
    smithy: { name: 'Smithy', icon: '🔨', description: 'Improve attack and defense of your units.', category: 'military' },
    market: { name: 'Market', icon: '🏪', description: 'Trade resources with other players and send merchants.', category: 'infrastructure' },
    embassy: { name: 'Embassy', icon: '🏰', description: 'Join or create alliances. Required for diplomacy.', category: 'special' },
    palace: { name: 'Palace', icon: '👑', description: 'Train settlers and administrators. Capital building.', category: 'special' },
    residence: { name: 'Residence', icon: '🏠', description: 'Train settlers for founding new villages.', category: 'special' },
    wall: { name: 'Wall', icon: '🧱', description: 'Increases defense bonus for your village.', category: 'military' },
    rally_point: { name: 'Rally Point', icon: '🚩', description: 'Manage troop movements and attacks.', category: 'military' },
    cranny: { name: 'Cranny', icon: '🕳️', description: 'Hide resources from enemy raids.', category: 'infrastructure' },
    hero_mansion: { name: 'Hero Mansion', icon: '🦸', description: 'House and manage your hero.', category: 'special' },
    tavern: { name: 'Tavern', icon: '🍺', description: 'Recruit special units and adventurers.', category: 'special' },
    town_hall: { name: 'Town Hall', icon: '🏛️', description: 'Host celebrations and increase culture points.', category: 'special' },
    treasury: { name: 'Treasury', icon: '💰', description: 'Store artifacts and increase their effect range.', category: 'special' },
    trade_office: { name: 'Trade Office', icon: '📊', description: 'Manage trade routes and merchant operations.', category: 'infrastructure' },
    woodcutter: { name: 'Woodcutter', icon: '🪵', description: 'Produces wood. Higher levels increase production.', category: 'resource' },
    clay_pit: { name: 'Clay Pit', icon: '🧱', description: 'Produces clay. Higher levels increase production.', category: 'resource' },
    iron_mine: { name: 'Iron Mine', icon: '⛏️', description: 'Produces iron. Higher levels increase production.', category: 'resource' },
    crop_field: { name: 'Crop Field', icon: '🌾', description: 'Produces crop. Higher levels increase production.', category: 'resource' }
  };

  // Mock upgrade costs based on level
  function getUpgradeCost(type: BuildingType, currentLevel: number): UpgradeCost {
    const baseCosts: Record<string, UpgradeCost> = {
      infrastructure: { wood: 100, clay: 80, iron: 50, crop: 30, time: 600 },
      military: { wood: 120, clay: 100, iron: 80, crop: 40, time: 900 },
      resource: { wood: 80, clay: 60, iron: 40, crop: 20, time: 450 },
      special: { wood: 200, clay: 150, iron: 100, crop: 50, time: 1200 }
    };

    const category = buildingInfo[type]?.category || 'infrastructure';
    const base = baseCosts[category];
    const multiplier = Math.pow(1.4, currentLevel);

    return {
      wood: Math.round(base.wood * multiplier),
      clay: Math.round(base.clay * multiplier),
      iron: Math.round(base.iron * multiplier),
      crop: Math.round(base.crop * multiplier),
      time: Math.round(base.time * multiplier)
    };
  }

  // Mock production for resource buildings
  function getProduction(type: BuildingType, level: number): number | null {
    if (!['woodcutter', 'clay_pit', 'iron_mine', 'crop_field'].includes(type)) {
      return null;
    }
    const baseProduction = 5;
    return Math.round(baseProduction * Math.pow(1.25, level));
  }

  function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  function formatTimeRemaining(endTime: Date): string {
    const diff = endTime.getTime() - Date.now();
    if (diff <= 0) return 'Done!';

    const hours = Math.floor(diff / 3600000);
    const minutes = Math.floor((diff % 3600000) / 60000);
    const seconds = Math.floor((diff % 60000) / 1000);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  }

  const info = $derived(building ? buildingInfo[building.type] : null);
  const upgradeCost = $derived(building ? getUpgradeCost(building.type, building.level) : null);
  const production = $derived(building ? getProduction(building.type, building.level) : null);
  const nextProduction = $derived(building ? getProduction(building.type, building.level + 1) : null);

  const canAfford = $derived(upgradeCost ? (
    playerResources.wood >= upgradeCost.wood &&
    playerResources.clay >= upgradeCost.clay &&
    playerResources.iron >= upgradeCost.iron &&
    playerResources.crop >= upgradeCost.crop
  ) : false);
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-lg">
    {#if building && info}
      <Dialog.Header>
        <div class="flex items-center gap-3">
          <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
            {info.icon}
          </div>
          <div>
            <Dialog.Title class="text-xl">{info.name}</Dialog.Title>
            <Dialog.Description>
              Level {building.level}
              {#if building.isUpgrading}
                <span class="text-amber-500 ml-2">• Upgrading...</span>
              {/if}
            </Dialog.Description>
          </div>
        </div>
      </Dialog.Header>

      <div class="space-y-4 py-4">
        <!-- Description -->
        <p class="text-sm text-muted-foreground">{info.description}</p>

        <!-- Current Production (for resource buildings) -->
        {#if production !== null}
          <Card class="p-4 bg-emerald-50 dark:bg-emerald-950/20">
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium">Current Production</span>
              <span class="text-lg font-bold text-emerald-600">+{production}/h</span>
            </div>
            {#if nextProduction !== null}
              <div class="flex items-center justify-between mt-2 text-sm text-muted-foreground">
                <span>After upgrade</span>
                <span class="text-emerald-600">+{nextProduction}/h (+{nextProduction - production})</span>
              </div>
            {/if}
          </Card>
        {/if}

        <!-- Upgrading Progress -->
        {#if building.isUpgrading && building.upgradeEndsAt}
          <Card class="p-4 bg-amber-50 dark:bg-amber-950/20">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm font-medium">Upgrading to Level {building.level + 1}</span>
              <span class="font-mono text-amber-600">{formatTimeRemaining(building.upgradeEndsAt)}</span>
            </div>
            <div class="w-full bg-amber-200 dark:bg-amber-900 rounded-full h-2">
              <div class="bg-amber-500 h-2 rounded-full animate-pulse" style="width: 45%"></div>
            </div>
            <div class="flex justify-end mt-2">
              <Button variant="outline" size="sm" class="text-amber-600">
                <span class="mr-1">💎</span> Finish Now
              </Button>
            </div>
          </Card>
        {:else if upgradeCost && building.level < 20}
          <!-- Upgrade Cost -->
          <Separator />

          <div>
            <h4 class="font-medium mb-3">Upgrade to Level {building.level + 1}</h4>

            <div class="grid grid-cols-2 gap-3 mb-4">
              <div class="flex items-center gap-2 p-2 rounded-lg bg-muted {playerResources.wood < upgradeCost.wood ? 'text-destructive' : ''}">
                <span>🪵</span>
                <span class="font-medium">{upgradeCost.wood}</span>
              </div>
              <div class="flex items-center gap-2 p-2 rounded-lg bg-muted {playerResources.clay < upgradeCost.clay ? 'text-destructive' : ''}">
                <span>🧱</span>
                <span class="font-medium">{upgradeCost.clay}</span>
              </div>
              <div class="flex items-center gap-2 p-2 rounded-lg bg-muted {playerResources.iron < upgradeCost.iron ? 'text-destructive' : ''}">
                <span>⛏️</span>
                <span class="font-medium">{upgradeCost.iron}</span>
              </div>
              <div class="flex items-center gap-2 p-2 rounded-lg bg-muted {playerResources.crop < upgradeCost.crop ? 'text-destructive' : ''}">
                <span>🌾</span>
                <span class="font-medium">{upgradeCost.crop}</span>
              </div>
            </div>

            <div class="flex items-center justify-between text-sm mb-4">
              <span class="text-muted-foreground">Build time</span>
              <span class="font-medium">⏱️ {formatTime(upgradeCost.time)}</span>
            </div>

            {#if error}
              <p class="text-sm text-destructive mb-2">{error}</p>
            {/if}

            <Button
              class="w-full"
              disabled={!canAfford || loading}
              onclick={onUpgrade}
            >
              {#if loading}
                <span class="animate-spin mr-2">⏳</span>
                Upgrading...
              {:else if canAfford}
                Upgrade to Level {building.level + 1}
              {:else}
                Not enough resources
              {/if}
            </Button>
          </div>
        {:else if building.level >= 20}
          <Card class="p-4 bg-primary/10">
            <div class="text-center">
              <span class="text-2xl">🏆</span>
              <p class="font-medium mt-2">Maximum Level Reached!</p>
            </div>
          </Card>
        {/if}

        <!-- Building Actions -->
        {#if info.category === 'military' && building.type === 'barracks'}
          <Separator />
          <Button variant="outline" class="w-full gap-2">
            <span>⚔️</span>
            Train Troops
          </Button>
        {/if}

        {#if info.category === 'military' && building.type === 'rally_point'}
          <Separator />
          <div class="space-y-2">
            <Button variant="outline" class="w-full gap-2">
              <span>⚔️</span>
              Send Attack
            </Button>
            <Button variant="outline" class="w-full gap-2">
              <span>🛡️</span>
              Send Reinforcement
            </Button>
          </div>
        {/if}

        {#if building.type === 'market'}
          <Separator />
          <div class="space-y-2">
            <Button variant="outline" class="w-full gap-2">
              <span>📤</span>
              Send Resources
            </Button>
            <Button variant="outline" class="w-full gap-2">
              <span>🔄</span>
              Trade
            </Button>
          </div>
        {/if}
      </div>

      <Dialog.Footer class="flex-col sm:flex-row gap-2">
        <Button variant="ghost" class="text-destructive" onclick={onDemolish} disabled={loading}>
          {#if loading}
            Demolishing...
          {:else}
            Demolish
          {/if}
        </Button>
        <Button variant="outline" onclick={() => open = false} disabled={loading}>
          Close
        </Button>
      </Dialog.Footer>
    {/if}
  </Dialog.Content>
</Dialog.Root>
