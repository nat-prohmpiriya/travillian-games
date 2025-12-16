<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import { Separator } from '$lib/components/ui/separator';
  import type { TileData, TerrainType, OwnerType } from '$lib/components/game/MapTile.svelte';

  interface Props {
    open: boolean;
    tile: TileData | null;
    currentVillageCoords?: { x: number; y: number };
    onSendArmy?: (mission: string) => void;
  }

  let { open = $bindable(false), tile, currentVillageCoords, onSendArmy }: Props = $props();

  const terrainInfo: Record<TerrainType, { name: string; description: string; icon: string }> = {
    grass: { name: 'Grassland', description: 'Fertile plains suitable for settlement.', icon: '🌿' },
    forest: { name: 'Forest', description: 'Dense woodland. Provides wood bonus.', icon: '🌲' },
    mountain: { name: 'Mountain', description: 'Rocky terrain. Provides iron bonus.', icon: '⛰️' },
    water: { name: 'Water', description: 'Impassable water tile.', icon: '🌊' },
    desert: { name: 'Desert', description: 'Arid wasteland.', icon: '🏜️' },
    oasis: { name: 'Oasis', description: 'Resource-rich area that can be captured.', icon: '🏝️' }
  };

  const ownerLabels: Record<OwnerType, { label: string; color: string }> = {
    self: { label: 'Your Village', color: 'text-primary' },
    ally: { label: 'Alliance Member', color: 'text-blue-500' },
    nap: { label: 'Non-Aggression Pact', color: 'text-emerald-500' },
    enemy: { label: 'Enemy', color: 'text-red-500' },
    neutral: { label: 'Neutral Player', color: 'text-gray-500' },
    npc: { label: 'Natarian (NPC)', color: 'text-amber-500' }
  };

  function calculateDistance(x1: number, y1: number, x2: number, y2: number): number {
    return Math.sqrt(Math.pow(x2 - x1, 2) + Math.pow(y2 - y1, 2));
  }

  function formatDistance(distance: number): string {
    return distance.toFixed(1);
  }

  function calculateTravelTime(distance: number, speed: number = 10): string {
    // speed is fields per hour
    const hours = distance / speed;
    if (hours < 1) {
      return `${Math.round(hours * 60)}m`;
    }
    const h = Math.floor(hours);
    const m = Math.round((hours - h) * 60);
    return `${h}h ${m}m`;
  }

  const terrain = $derived(tile ? terrainInfo[tile.terrain] : null);
  const distance = $derived(
    tile && currentVillageCoords
      ? calculateDistance(currentVillageCoords.x, currentVillageCoords.y, tile.x, tile.y)
      : 0
  );
  const travelTime = $derived(calculateTravelTime(distance));
  const canSettle = $derived(tile && !tile.village && tile.terrain === 'grass');
  const canAttack = $derived(tile?.village && tile.village.ownerType !== 'self');
  const isOwnVillage = $derived(tile?.village?.ownerType === 'self');
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-md">
    {#if tile && terrain}
      <Dialog.Header>
        <div class="flex items-center gap-3">
          <div class="w-12 h-12 rounded-xl bg-muted flex items-center justify-center text-2xl">
            {terrain.icon}
          </div>
          <div>
            <Dialog.Title>
              {#if tile.village}
                {tile.village.name}
              {:else if tile.oasis}
                Oasis
              {:else}
                {terrain.name}
              {/if}
            </Dialog.Title>
            <Dialog.Description>
              Coordinates: ({tile.x}|{tile.y})
            </Dialog.Description>
          </div>
        </div>
      </Dialog.Header>

      <div class="space-y-4 py-4">
        <!-- Distance Info -->
        {#if currentVillageCoords && distance > 0}
          <div class="flex items-center justify-between text-sm">
            <span class="text-muted-foreground">Distance</span>
            <span class="font-medium">{formatDistance(distance)} fields</span>
          </div>
          <div class="flex items-center justify-between text-sm">
            <span class="text-muted-foreground">Travel time (cavalry)</span>
            <span class="font-medium">{travelTime}</span>
          </div>
          <Separator />
        {/if}

        <!-- Village Info -->
        {#if tile.village}
          <Card class="p-4">
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Owner</span>
                <span class="font-medium">{tile.village.playerName}</span>
              </div>
              {#if tile.village.allianceTag}
                <div class="flex items-center justify-between">
                  <span class="text-sm text-muted-foreground">Alliance</span>
                  <span class="font-medium">[{tile.village.allianceTag}]</span>
                </div>
              {/if}
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Population</span>
                <span class="font-medium">{tile.village.population}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Status</span>
                <span class="font-medium {ownerLabels[tile.village.ownerType].color}">
                  {ownerLabels[tile.village.ownerType].label}
                </span>
              </div>
            </div>
          </Card>
        {/if}

        <!-- Oasis Info -->
        {#if tile.oasis}
          <Card class="p-4 bg-emerald-50 dark:bg-emerald-950/20">
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Resource Bonus</span>
                <span class="font-medium text-emerald-600">+{tile.oasis.bonus}% {tile.oasis.type}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-muted-foreground">Status</span>
                <span class="font-medium {tile.oasis.occupied ? 'text-amber-600' : 'text-emerald-600'}">
                  {tile.oasis.occupied ? 'Occupied' : 'Unoccupied'}
                </span>
              </div>
            </div>
          </Card>
        {/if}

        <!-- Empty Land Info -->
        {#if !tile.village && !tile.oasis}
          <p class="text-sm text-muted-foreground">{terrain.description}</p>
          {#if canSettle}
            <Card class="p-4 bg-primary/5">
              <p class="text-sm text-center">
                This tile is available for settlement!
              </p>
            </Card>
          {/if}
        {/if}

        <Separator />

        <!-- Actions -->
        <div class="space-y-2">
          {#if isOwnVillage}
            <Button class="w-full gap-2" onclick={() => { open = false; }}>
              <span>🏠</span>
              Go to Village
            </Button>
          {:else if canAttack}
            <div class="grid grid-cols-2 gap-2">
              <Button variant="destructive" class="gap-2" onclick={() => onSendArmy?.('raid')}>
                <span>🗡️</span>
                Raid
              </Button>
              <Button variant="destructive" class="gap-2" onclick={() => onSendArmy?.('attack')}>
                <span>⚔️</span>
                Attack
              </Button>
            </div>
            <div class="grid grid-cols-2 gap-2">
              <Button variant="outline" class="gap-2" onclick={() => onSendArmy?.('scout')}>
                <span>🔍</span>
                Scout
              </Button>
              {#if tile.village?.ownerType === 'ally' || tile.village?.ownerType === 'nap'}
                <Button variant="outline" class="gap-2" onclick={() => onSendArmy?.('support')}>
                  <span>🛡️</span>
                  Reinforce
                </Button>
              {:else}
                <Button variant="outline" class="gap-2" onclick={() => onSendArmy?.('conquer')}>
                  <span>👑</span>
                  Conquer
                </Button>
              {/if}
            </div>
          {:else if canSettle}
            <Button class="w-full gap-2" onclick={() => onSendArmy?.('settle')}>
              <span>🏗️</span>
              Send Settlers
            </Button>
          {:else if tile.oasis && !tile.oasis.occupied}
            <Button class="w-full gap-2" onclick={() => onSendArmy?.('occupy')}>
              <span>🏝️</span>
              Occupy Oasis
            </Button>
          {/if}

          <Button variant="outline" class="w-full gap-2">
            <span>📍</span>
            Set as Target
          </Button>
        </div>
      </div>

      <Dialog.Footer>
        <Button variant="ghost" onclick={() => open = false}>
          Close
        </Button>
      </Dialog.Footer>
    {/if}
  </Dialog.Content>
</Dialog.Root>
