<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Card } from '$lib/components/ui/card';
  import MapTile, { type TileData, type TerrainType, type OwnerType } from '$lib/components/game/MapTile.svelte';
  import TileDetailModal from '$lib/components/modals/TileDetailModal.svelte';

  // Map viewport settings
  const VIEWPORT_SIZE = 15; // 15x15 grid
  const MAP_SIZE = 200; // Total map size (±100 from center)

  let centerX = $state(100);
  let centerY = $state(100);
  let selectedTile = $state<TileData | null>(null);
  let showTileModal = $state(false);
  let searchInput = $state('');

  // Current village (mock)
  const currentVillage = {
    x: 100,
    y: 100,
    name: 'Capital'
  };

  // Generate mock map data
  function generateMockTiles(cx: number, cy: number): TileData[][] {
    const halfSize = Math.floor(VIEWPORT_SIZE / 2);
    const tiles: TileData[][] = [];

    for (let dy = -halfSize; dy <= halfSize; dy++) {
      const row: TileData[] = [];
      for (let dx = -halfSize; dx <= halfSize; dx++) {
        const x = cx + dx;
        const y = cy + dy;
        row.push(generateTile(x, y));
      }
      tiles.push(row);
    }

    return tiles;
  }

  function generateTile(x: number, y: number): TileData {
    // Use coordinates as seed for consistent terrain
    const seed = Math.abs(x * 1000 + y);
    const rand = () => {
      const n = Math.sin(seed) * 10000;
      return n - Math.floor(n);
    };

    // Determine terrain type
    let terrain: TerrainType = 'grass';
    const terrainRoll = rand();
    if (terrainRoll < 0.05) terrain = 'water';
    else if (terrainRoll < 0.15) terrain = 'forest';
    else if (terrainRoll < 0.22) terrain = 'mountain';
    else if (terrainRoll < 0.25) terrain = 'desert';

    const tile: TileData = { x, y, terrain };

    // Add villages at specific locations (mock)
    if (x === 100 && y === 100) {
      tile.village = {
        id: '1',
        name: 'Capital',
        playerName: 'You',
        population: 127,
        ownerType: 'self'
      };
    } else if (x === 102 && y === 99) {
      tile.village = {
        id: '2',
        name: 'Outpost',
        playerName: 'You',
        population: 45,
        ownerType: 'self'
      };
    } else if (x === 98 && y === 101) {
      tile.village = {
        id: '3',
        name: 'Dragon Keep',
        playerName: 'DragonLord',
        population: 234,
        ownerType: 'ally',
        allianceTag: 'DRG'
      };
    } else if (x === 103 && y === 103) {
      tile.village = {
        id: '4',
        name: 'Dark Castle',
        playerName: 'Raider123',
        population: 189,
        ownerType: 'enemy',
        allianceTag: 'WAR'
      };
    } else if (x === 96 && y === 98) {
      tile.village = {
        id: '5',
        name: 'Peaceful Town',
        playerName: 'Farmer',
        population: 78,
        ownerType: 'neutral'
      };
    } else if (x === 105 && y === 100) {
      tile.village = {
        id: '6',
        name: 'Natarian Fort',
        playerName: 'Natarian',
        population: 500,
        ownerType: 'npc'
      };
    }

    // Add oases at specific locations
    if (!tile.village && terrain === 'grass') {
      if ((x === 97 && y === 102) || (x === 104 && y === 97)) {
        tile.terrain = 'oasis';
        tile.oasis = {
          type: x === 97 ? 'crop' : 'wood',
          bonus: 25,
          occupied: x === 97
        };
      }
    }

    return tile;
  }

  function handleTileClick(tile: TileData) {
    selectedTile = tile;
    showTileModal = true;
  }

  function moveMap(dx: number, dy: number) {
    centerX = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, centerX + dx));
    centerY = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, centerY + dy));
  }

  function goToCoordinates() {
    const match = searchInput.match(/(-?\d+)[,|](-?\d+)/);
    if (match) {
      const x = parseInt(match[1]);
      const y = parseInt(match[2]);
      centerX = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, x));
      centerY = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, y));
      searchInput = '';
    }
  }

  function centerOnVillage() {
    centerX = currentVillage.x;
    centerY = currentVillage.y;
  }

  function handleSendArmy(mission: string) {
    console.log('Send army:', mission, 'to', selectedTile);
    showTileModal = false;
    // TODO: Navigate to send army page
  }

  function handleKeydown(event: KeyboardEvent) {
    const step = event.shiftKey ? 5 : 1;
    switch (event.key) {
      case 'ArrowUp':
        moveMap(0, -step);
        event.preventDefault();
        break;
      case 'ArrowDown':
        moveMap(0, step);
        event.preventDefault();
        break;
      case 'ArrowLeft':
        moveMap(-step, 0);
        event.preventDefault();
        break;
      case 'ArrowRight':
        moveMap(step, 0);
        event.preventDefault();
        break;
    }
  }

  const tiles = $derived(generateMockTiles(centerX, centerY));
</script>

<svelte:head>
  <title>Map - Tusk & Horn</title>
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<div class="container mx-auto px-4 py-6 max-w-6xl">
  <!-- Header -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-4">
    <div>
      <h1 class="text-2xl font-bold">World Map</h1>
      <p class="text-muted-foreground text-sm">Center: ({centerX}|{centerY})</p>
    </div>

    <!-- Search & Controls -->
    <div class="flex items-center gap-2">
      <form onsubmit={(e) => { e.preventDefault(); goToCoordinates(); }} class="flex gap-2">
        <Input
          type="text"
          placeholder="x|y (e.g. 100|100)"
          bind:value={searchInput}
          class="w-32"
        />
        <Button type="submit" variant="outline" size="icon">
          🔍
        </Button>
      </form>
      <Button variant="outline" size="icon" onclick={centerOnVillage} title="Center on village">
        🏠
      </Button>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-4 gap-4">
    <!-- Map Grid -->
    <div class="lg:col-span-3">
      <Card class="p-2 overflow-hidden">
        <!-- Navigation Buttons -->
        <div class="flex justify-center mb-2">
          <Button variant="ghost" size="sm" onclick={() => moveMap(0, -5)}>
            ▲
          </Button>
        </div>

        <div class="flex items-center gap-2">
          <Button variant="ghost" size="sm" onclick={() => moveMap(-5, 0)}>
            ◀
          </Button>

          <!-- Map Grid -->
          <div
            class="flex-1 grid gap-px bg-border"
            style="grid-template-columns: repeat({VIEWPORT_SIZE}, minmax(0, 1fr));"
          >
            {#each tiles as row}
              {#each row as tile}
                <MapTile
                  {tile}
                  isSelected={selectedTile?.x === tile.x && selectedTile?.y === tile.y}
                  onclick={() => handleTileClick(tile)}
                />
              {/each}
            {/each}
          </div>

          <Button variant="ghost" size="sm" onclick={() => moveMap(5, 0)}>
            ▶
          </Button>
        </div>

        <div class="flex justify-center mt-2">
          <Button variant="ghost" size="sm" onclick={() => moveMap(0, 5)}>
            ▼
          </Button>
        </div>

        <!-- Keyboard hint -->
        <p class="text-xs text-muted-foreground text-center mt-2">
          Use arrow keys to navigate (hold Shift for faster)
        </p>
      </Card>
    </div>

    <!-- Sidebar -->
    <div class="space-y-4">
      <!-- Legend -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3">Legend</h3>
        <div class="space-y-2 text-sm">
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-2 ring-primary bg-primary/20"></div>
            <span>Your Village</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-2 ring-blue-500 bg-blue-500/20"></div>
            <span>Alliance</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-2 ring-emerald-500 bg-emerald-500/20"></div>
            <span>NAP</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-2 ring-red-500 bg-red-500/20"></div>
            <span>Enemy</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-1 ring-gray-400 bg-gray-400/20"></div>
            <span>Neutral</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded ring-1 ring-amber-500 bg-amber-500/20"></div>
            <span>Natarian (NPC)</span>
          </div>
        </div>
      </Card>

      <!-- Terrain Legend -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3">Terrain</h3>
        <div class="space-y-2 text-sm">
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-green-100 dark:bg-green-900/30"></div>
            <span>Grassland</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-green-200 dark:bg-green-800/40"></div>
            <span>🌲 Forest</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-stone-300 dark:bg-stone-700/50"></div>
            <span>⛰️ Mountain</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-blue-200 dark:bg-blue-800/40"></div>
            <span>🌊 Water</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-emerald-200 dark:bg-emerald-700/40"></div>
            <span>🏝️ Oasis</span>
          </div>
        </div>
      </Card>

      <!-- Quick Jump -->
      <Card class="p-4">
        <h3 class="font-semibold mb-3">Your Villages</h3>
        <div class="space-y-2">
          <Button
            variant="outline"
            class="w-full justify-start gap-2"
            size="sm"
            onclick={() => { centerX = 100; centerY = 100; }}
          >
            <span>👑</span>
            <span class="truncate">Capital (100|100)</span>
          </Button>
          <Button
            variant="outline"
            class="w-full justify-start gap-2"
            size="sm"
            onclick={() => { centerX = 102; centerY = 99; }}
          >
            <span>🏠</span>
            <span class="truncate">Outpost (102|99)</span>
          </Button>
        </div>
      </Card>
    </div>
  </div>
</div>

<!-- Tile Detail Modal -->
<TileDetailModal
  bind:open={showTileModal}
  tile={selectedTile}
  currentVillageCoords={currentVillage}
  onSendArmy={handleSendArmy}
/>
