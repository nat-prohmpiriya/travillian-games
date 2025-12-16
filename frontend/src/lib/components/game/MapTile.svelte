<script lang="ts">
  export type TerrainType = 'grass' | 'forest' | 'mountain' | 'water' | 'desert' | 'oasis';
  export type OwnerType = 'self' | 'ally' | 'nap' | 'enemy' | 'neutral' | 'npc';

  interface Village {
    id: string;
    name: string;
    playerName: string;
    population: number;
    ownerType: OwnerType;
    allianceTag?: string;
  }

  interface Oasis {
    type: 'wood' | 'clay' | 'iron' | 'crop' | 'mixed';
    bonus: number;
    occupied: boolean;
  }

  export interface TileData {
    x: number;
    y: number;
    terrain: TerrainType;
    village?: Village;
    oasis?: Oasis;
  }

  interface Props {
    tile: TileData;
    isSelected?: boolean;
    onclick?: () => void;
  }

  let { tile, isSelected = false, onclick }: Props = $props();

  const terrainStyles: Record<TerrainType, { bg: string; icon: string }> = {
    grass: { bg: 'bg-green-100 dark:bg-green-900/30', icon: '' },
    forest: { bg: 'bg-green-200 dark:bg-green-800/40', icon: '🌲' },
    mountain: { bg: 'bg-stone-300 dark:bg-stone-700/50', icon: '⛰️' },
    water: { bg: 'bg-blue-200 dark:bg-blue-800/40', icon: '🌊' },
    desert: { bg: 'bg-amber-100 dark:bg-amber-900/30', icon: '' },
    oasis: { bg: 'bg-emerald-200 dark:bg-emerald-700/40', icon: '🏝️' }
  };

  const ownerColors: Record<OwnerType, string> = {
    self: 'ring-2 ring-primary bg-primary/20',
    ally: 'ring-2 ring-blue-500 bg-blue-500/20',
    nap: 'ring-2 ring-emerald-500 bg-emerald-500/20',
    enemy: 'ring-2 ring-red-500 bg-red-500/20',
    neutral: 'ring-1 ring-gray-400 bg-gray-400/20',
    npc: 'ring-1 ring-amber-500 bg-amber-500/20'
  };

  const oasisIcons: Record<string, string> = {
    wood: '🪵',
    clay: '🧱',
    iron: '⛏️',
    crop: '🌾',
    mixed: '✨'
  };

  const style = $derived(terrainStyles[tile.terrain]);
  const hasVillage = $derived(!!tile.village);
  const hasOasis = $derived(!!tile.oasis);
</script>

<button
  type="button"
  {onclick}
  class="relative aspect-square w-full transition-all hover:scale-105 hover:z-10 focus:outline-none focus:ring-2 focus:ring-ring
         {style.bg}
         {isSelected ? 'ring-2 ring-primary scale-105 z-10' : ''}
         {hasVillage ? ownerColors[tile.village!.ownerType] : ''}"
  title="{tile.x}|{tile.y}"
>
  <!-- Terrain Icon (for special terrain) -->
  {#if style.icon && !hasVillage && !hasOasis}
    <span class="absolute inset-0 flex items-center justify-center text-xs opacity-50">
      {style.icon}
    </span>
  {/if}

  <!-- Oasis -->
  {#if hasOasis && !hasVillage}
    <div class="absolute inset-0 flex flex-col items-center justify-center">
      <span class="text-sm">{oasisIcons[tile.oasis!.type]}</span>
      <span class="text-[8px] font-bold text-emerald-700 dark:text-emerald-300">
        +{tile.oasis!.bonus}%
      </span>
    </div>
  {/if}

  <!-- Village -->
  {#if hasVillage}
    <div class="absolute inset-0 flex flex-col items-center justify-center p-0.5">
      <!-- Village Icon based on owner -->
      <span class="text-sm">
        {#if tile.village!.ownerType === 'self'}
          🏠
        {:else if tile.village!.ownerType === 'npc'}
          🏛️
        {:else}
          🏘️
        {/if}
      </span>

      <!-- Population indicator -->
      <span class="text-[8px] font-medium truncate w-full text-center">
        {tile.village!.population}
      </span>
    </div>

    <!-- Alliance Tag -->
    {#if tile.village!.allianceTag}
      <span class="absolute top-0 right-0 text-[6px] font-bold px-0.5 bg-black/50 text-white rounded-bl">
        {tile.village!.allianceTag}
      </span>
    {/if}
  {/if}

  <!-- Coordinates (shown on hover via CSS) -->
  <span class="absolute bottom-0 left-0 right-0 text-[6px] text-center opacity-0 hover:opacity-100 bg-black/50 text-white transition-opacity">
    {tile.x}|{tile.y}
  </span>
</button>

<style>
  button:hover span:last-child {
    opacity: 1;
  }
</style>
