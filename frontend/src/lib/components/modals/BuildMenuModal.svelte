<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import type { BuildingType } from '$lib/components/game/BuildingSlot.svelte';

  interface BuildOption {
    type: BuildingType;
    name: string;
    icon: string;
    description: string;
    category: 'infrastructure' | 'military' | 'special';
    cost: {
      wood: number;
      clay: number;
      iron: number;
      crop: number;
      time: number;
    };
    requirements: string[];
    available: boolean;
  }

  interface VillageResources {
    wood: number;
    clay: number;
    iron: number;
    crop: number;
  }

  interface Props {
    open: boolean;
    slot: number;
    isResourceField?: boolean;
    villageResources?: VillageResources;
    onBuild?: (type: BuildingType) => void;
    loading?: boolean;
    error?: string;
  }

  let { open = $bindable(false), slot, isResourceField = false, villageResources, onBuild, loading = false, error = '' }: Props = $props();

  let selectedCategory = $state<'all' | 'infrastructure' | 'military' | 'special'>('all');

  // Use village resources if provided, otherwise fallback to mock data
  const playerResources = $derived(villageResources || {
    wood: 1250,
    clay: 980,
    iron: 750,
    crop: 1100
  });

  // Available buildings for village center
  const villageBuildOptions: BuildOption[] = [
    {
      type: 'warehouse',
      name: 'Warehouse',
      icon: '📦',
      description: 'Stores wood, clay, and iron.',
      category: 'infrastructure',
      cost: { wood: 130, clay: 160, iron: 90, crop: 40, time: 600 },
      requirements: [],
      available: true
    },
    {
      type: 'granary',
      name: 'Granary',
      icon: '🌾',
      description: 'Stores crop.',
      category: 'infrastructure',
      cost: { wood: 80, clay: 100, iron: 70, crop: 20, time: 500 },
      requirements: [],
      available: true
    },
    {
      type: 'cranny',
      name: 'Cranny',
      icon: '🕳️',
      description: 'Hide resources from raids.',
      category: 'infrastructure',
      cost: { wood: 40, clay: 50, iron: 30, crop: 10, time: 300 },
      requirements: [],
      available: true
    },
    {
      type: 'market',
      name: 'Market',
      icon: '🏪',
      description: 'Trade resources with others.',
      category: 'infrastructure',
      cost: { wood: 80, clay: 70, iron: 120, crop: 70, time: 900 },
      requirements: ['Main Building Lv.3'],
      available: true
    },
    {
      type: 'barracks',
      name: 'Barracks',
      icon: '⚔️',
      description: 'Train infantry units.',
      category: 'military',
      cost: { wood: 210, clay: 140, iron: 260, crop: 120, time: 1200 },
      requirements: ['Main Building Lv.3', 'Rally Point Lv.1'],
      available: true
    },
    {
      type: 'stable',
      name: 'Stable',
      icon: '🐎',
      description: 'Train cavalry and war elephants.',
      category: 'military',
      cost: { wood: 260, clay: 140, iron: 220, crop: 100, time: 1500 },
      requirements: ['Barracks Lv.3', 'Academy Lv.5'],
      available: false
    },
    {
      type: 'workshop',
      name: 'Workshop',
      icon: '🔧',
      description: 'Build siege weapons.',
      category: 'military',
      cost: { wood: 460, clay: 510, iron: 600, crop: 320, time: 2400 },
      requirements: ['Academy Lv.10', 'Main Building Lv.5'],
      available: false
    },
    {
      type: 'academy',
      name: 'Academy',
      icon: '📚',
      description: 'Research technologies.',
      category: 'military',
      cost: { wood: 220, clay: 160, iron: 90, crop: 40, time: 1200 },
      requirements: ['Main Building Lv.3', 'Barracks Lv.3'],
      available: false
    },
    {
      type: 'smithy',
      name: 'Smithy',
      icon: '🔨',
      description: 'Improve unit stats.',
      category: 'military',
      cost: { wood: 180, clay: 200, iron: 180, crop: 60, time: 1000 },
      requirements: ['Main Building Lv.3', 'Academy Lv.1'],
      available: false
    },
    {
      type: 'wall',
      name: 'Wall',
      icon: '🧱',
      description: 'Increases village defense.',
      category: 'military',
      cost: { wood: 120, clay: 200, iron: 0, crop: 80, time: 800 },
      requirements: [],
      available: true
    },
    {
      type: 'embassy',
      name: 'Embassy',
      icon: '🏰',
      description: 'Join or create alliances.',
      category: 'special',
      cost: { wood: 180, clay: 130, iron: 150, crop: 80, time: 1200 },
      requirements: ['Main Building Lv.1'],
      available: true
    },
    {
      type: 'residence',
      name: 'Residence',
      icon: '🏠',
      description: 'Train settlers.',
      category: 'special',
      cost: { wood: 580, clay: 460, iron: 350, crop: 180, time: 2000 },
      requirements: ['Main Building Lv.5'],
      available: false
    },
    {
      type: 'hero_mansion',
      name: 'Hero Mansion',
      icon: '🦸',
      description: 'House your hero.',
      category: 'special',
      cost: { wood: 700, clay: 670, iron: 700, crop: 240, time: 3000 },
      requirements: ['Main Building Lv.3', 'Rally Point Lv.1'],
      available: false
    },
    {
      type: 'tavern',
      name: 'Tavern',
      icon: '🍺',
      description: 'Recruit adventurers.',
      category: 'special',
      cost: { wood: 1400, clay: 1330, iron: 1200, crop: 400, time: 4000 },
      requirements: ['Main Building Lv.10', 'Academy Lv.5'],
      available: false
    }
  ];

  function canAfford(cost: BuildOption['cost']): boolean {
    return (
      playerResources.wood >= cost.wood &&
      playerResources.clay >= cost.clay &&
      playerResources.iron >= cost.iron &&
      playerResources.crop >= cost.crop
    );
  }

  function formatTime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  function handleBuild(option: BuildOption) {
    if (!option.available || !canAfford(option.cost) || loading) return;
    onBuild?.(option.type);
    // Don't close modal here - let parent handle it after API call succeeds
  }

  const filteredOptions = $derived(
    selectedCategory === 'all'
      ? villageBuildOptions
      : villageBuildOptions.filter(o => o.category === selectedCategory)
  );

  const categories = [
    { value: 'all' as const, label: 'All', icon: '🏗️' },
    { value: 'infrastructure' as const, label: 'Infrastructure', icon: '📦' },
    { value: 'military' as const, label: 'Military', icon: '⚔️' },
    { value: 'special' as const, label: 'Special', icon: '✨' }
  ];
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-2xl max-h-[85vh] overflow-hidden flex flex-col">
    <Dialog.Header>
      <Dialog.Title class="text-xl">Build New Structure</Dialog.Title>
      <Dialog.Description>
        Choose a building for slot #{slot}
      </Dialog.Description>
    </Dialog.Header>

    <!-- Category Filter -->
    <div class="flex gap-2 py-2 overflow-x-auto">
      {#each categories as cat}
        <Button
          variant={selectedCategory === cat.value ? 'default' : 'outline'}
          size="sm"
          onclick={() => selectedCategory = cat.value}
          class="gap-1 shrink-0"
        >
          <span>{cat.icon}</span>
          <span class="hidden sm:inline">{cat.label}</span>
        </Button>
      {/each}
    </div>

    <!-- Building List -->
    <div class="flex-1 overflow-y-auto space-y-2 pr-2">
      {#each filteredOptions as option}
        <Card
          class="p-3 transition-all {option.available ? 'cursor-pointer hover:shadow-md' : 'opacity-60'}"
          onclick={() => handleBuild(option)}
        >
          <div class="flex items-start gap-3">
            <!-- Icon -->
            <div class="w-12 h-12 rounded-lg bg-muted flex items-center justify-center text-2xl shrink-0">
              {option.icon}
            </div>

            <!-- Info -->
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <h4 class="font-medium">{option.name}</h4>
                {#if !option.available}
                  <span class="text-xs bg-muted px-2 py-0.5 rounded-full text-muted-foreground">
                    Locked
                  </span>
                {/if}
              </div>
              <p class="text-sm text-muted-foreground line-clamp-1">{option.description}</p>

              <!-- Requirements -->
              {#if option.requirements.length > 0 && !option.available}
                <p class="text-xs text-amber-600 mt-1">
                  Requires: {option.requirements.join(', ')}
                </p>
              {/if}
            </div>

            <!-- Cost -->
            <div class="shrink-0 text-right">
              <div class="grid grid-cols-2 gap-x-3 gap-y-1 text-xs">
                <span class="{playerResources.wood < option.cost.wood ? 'text-destructive' : ''}">
                  🪵 {option.cost.wood}
                </span>
                <span class="{playerResources.clay < option.cost.clay ? 'text-destructive' : ''}">
                  🧱 {option.cost.clay}
                </span>
                <span class="{playerResources.iron < option.cost.iron ? 'text-destructive' : ''}">
                  ⛏️ {option.cost.iron}
                </span>
                <span class="{playerResources.crop < option.cost.crop ? 'text-destructive' : ''}">
                  🌾 {option.cost.crop}
                </span>
              </div>
              <p class="text-xs text-muted-foreground mt-1">
                ⏱️ {formatTime(option.cost.time)}
              </p>
            </div>
          </div>
        </Card>
      {/each}
    </div>

    <Dialog.Footer class="flex-col sm:flex-row gap-2">
      {#if error}
        <p class="text-sm text-destructive flex-1">{error}</p>
      {/if}
      {#if loading}
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
          <span class="animate-spin">⏳</span>
          Building...
        </div>
      {/if}
      <Button variant="outline" onclick={() => open = false} disabled={loading}>
        Cancel
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
