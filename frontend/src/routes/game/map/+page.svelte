<script lang="ts">
  import { onMount } from 'svelte';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Card } from '$lib/components/ui/card';
  import MapTile, { type TileData, type TerrainType, type OwnerType } from '$lib/components/game/MapTile.svelte';
  import TileDetailModal from '$lib/components/modals/TileDetailModal.svelte';
  import { mapStore, getOwnerType, type MapTile as ApiMapTile } from '$lib/stores/map';
  import { villageStore } from '$lib/stores/village';

  // Map viewport settings
  const VIEWPORT_SIZE = 15; // 15x15 grid
  const MAP_SIZE = 400; // Total map size (±200 from center)
  const API_RANGE = 7; // API range parameter

  let centerX = $state(0);
  let centerY = $state(0);
  let selectedTile = $state<TileData | null>(null);
  let showTileModal = $state(false);
  let searchInput = $state('');
  let isLoading = $state(true);

  // Subscribe to stores
  let mapState = $state(mapStore);
  let villageState = $state(villageStore);
  let apiTiles = $derived($mapState.tiles);
  let villages = $derived($villageState.villages);
  let currentVillage = $derived($villageState.currentVillage);

  // Generate deterministic terrain based on coordinates (since backend doesn't provide terrain)
  function generateTerrain(x: number, y: number): TerrainType {
    const seed = Math.abs(x * 1000 + y);
    const rand = Math.sin(seed) * 10000;
    const roll = rand - Math.floor(rand);

    if (roll < 0.05) return 'water';
    if (roll < 0.15) return 'forest';
    if (roll < 0.22) return 'mountain';
    if (roll < 0.25) return 'desert';
    return 'grass';
  }

  // Check if coordinates have an oasis (deterministic based on coords)
  function isOasis(x: number, y: number): { type: 'wood' | 'clay' | 'iron' | 'crop'; bonus: number } | null {
    const seed = Math.abs(x * 1234 + y * 5678);
    const rand = Math.sin(seed) * 10000;
    const roll = rand - Math.floor(rand);

    if (roll < 0.02) {
      const typeRoll = Math.abs(Math.sin(seed * 2) * 10000) % 4;
      const types: Array<'wood' | 'clay' | 'iron' | 'crop'> = ['wood', 'clay', 'iron', 'crop'];
      return {
        type: types[Math.floor(typeRoll)],
        bonus: 25
      };
    }
    return null;
  }

  // Convert API tile to display tile with terrain
  function apiTileToDisplayTile(apiTile: ApiMapTile): TileData {
    const tile: TileData = {
      x: apiTile.x,
      y: apiTile.y,
      terrain: generateTerrain(apiTile.x, apiTile.y)
    };

    // Add village from API if exists
    if (apiTile.village) {
      tile.village = {
        id: apiTile.village.id,
        name: apiTile.village.name,
        playerName: apiTile.village.player_name || 'Unknown',
        population: apiTile.village.population,
        ownerType: getOwnerType(apiTile.village) || 'neutral'
      };
      // Villages override terrain
      tile.terrain = 'grass';
    }

    // Add oasis if no village and terrain is grass
    if (!tile.village && tile.terrain === 'grass') {
      const oasis = isOasis(apiTile.x, apiTile.y);
      if (oasis) {
        tile.terrain = 'oasis';
        tile.oasis = {
          type: oasis.type,
          bonus: oasis.bonus,
          occupied: false
        };
      }
    }

    return tile;
  }

  // Generate display tiles from API tiles
  function generateDisplayTiles(cx: number, cy: number): TileData[][] {
    const halfSize = Math.floor(VIEWPORT_SIZE / 2);
    const tiles: TileData[][] = [];

    // Create a map of API tiles by coordinates for quick lookup
    const tileMap = new Map<string, ApiMapTile>();
    for (const tile of apiTiles) {
      tileMap.set(`${tile.x},${tile.y}`, tile);
    }

    for (let dy = -halfSize; dy <= halfSize; dy++) {
      const row: TileData[] = [];
      for (let dx = -halfSize; dx <= halfSize; dx++) {
        const x = cx + dx;
        const y = cy + dy;
        const key = `${x},${y}`;

        const apiTile = tileMap.get(key);
        if (apiTile) {
          row.push(apiTileToDisplayTile(apiTile));
        } else {
          // Generate local tile if not in API response
          row.push({
            x,
            y,
            terrain: generateTerrain(x, y)
          });
        }
      }
      tiles.push(row);
    }

    return tiles;
  }

  function handleTileClick(tile: TileData) {
    selectedTile = tile;
    showTileModal = true;
  }

  async function moveMap(dx: number, dy: number) {
    const newX = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, centerX + dx));
    const newY = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, centerY + dy));

    if (newX !== centerX || newY !== centerY) {
      centerX = newX;
      centerY = newY;
      await loadMapTiles();
    }
  }

  async function goToCoordinates() {
    const match = searchInput.match(/(-?\d+)[,|](-?\d+)/);
    if (match) {
      const x = parseInt(match[1]);
      const y = parseInt(match[2]);
      centerX = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, x));
      centerY = Math.max(-MAP_SIZE/2, Math.min(MAP_SIZE/2, y));
      searchInput = '';
      await loadMapTiles();
    }
  }

  async function centerOnVillage() {
    if (currentVillage) {
      centerX = currentVillage.x;
      centerY = currentVillage.y;
      await loadMapTiles();
    }
  }

  async function jumpToVillage(x: number, y: number) {
    centerX = x;
    centerY = y;
    await loadMapTiles();
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

  async function loadMapTiles() {
    try {
      await mapStore.loadTiles(centerX, centerY, API_RANGE);
    } catch (error) {
      console.error('Failed to load map tiles:', error);
    }
  }

  // Load initial data
  onMount(async () => {
    try {
      // Load villages first to get user's village coordinates
      const loadedVillages = await villageStore.loadVillages();

      if (loadedVillages.length > 0) {
        // Center map on first village
        centerX = loadedVillages[0].x;
        centerY = loadedVillages[0].y;
      }

      // Load map tiles
      await loadMapTiles();
    } catch (error) {
      console.error('Failed to load map:', error);
    } finally {
      isLoading = false;
    }
  });

  const tiles = $derived(generateDisplayTiles(centerX, centerY));
</script>

<svelte:head>
  <title>Map - Tusk & Horn</title>
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

{#if isLoading}
  <div class="flex items-center justify-center min-h-[60vh]">
    <div class="text-center">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary mx-auto mb-4"></div>
      <p class="text-muted-foreground">Loading map...</p>
    </div>
  </div>
{:else}
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

        <!-- Quick Jump - Your Villages -->
        <Card class="p-4">
          <h3 class="font-semibold mb-3">Your Villages</h3>
          <div class="space-y-2">
            {#if villages.length > 0}
              {#each villages as village}
                <Button
                  variant="outline"
                  class="w-full justify-start gap-2"
                  size="sm"
                  onclick={() => jumpToVillage(village.x, village.y)}
                >
                  <span>{village.is_capital ? '👑' : '🏠'}</span>
                  <span class="truncate">{village.name} ({village.x}|{village.y})</span>
                </Button>
              {/each}
            {:else}
              <p class="text-sm text-muted-foreground text-center py-2">No villages</p>
            {/if}
          </div>
        </Card>
      </div>
    </div>
  </div>
{/if}

<!-- Tile Detail Modal -->
<TileDetailModal
  bind:open={showTileModal}
  tile={selectedTile}
  currentVillageCoords={currentVillage ? { x: currentVillage.x, y: currentVillage.y } : { x: 0, y: 0 }}
  onSendArmy={handleSendArmy}
/>
