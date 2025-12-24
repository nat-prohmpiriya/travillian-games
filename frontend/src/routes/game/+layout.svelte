<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { t } from 'svelte-i18n';
  import { toast } from 'svelte-sonner';
  import { Button } from '$lib/components/ui/button';
  import ResourceBar from '$lib/components/game/ResourceBar.svelte';
  import LanguageSwitcher from '$lib/components/LanguageSwitcher.svelte';
  import ReportsModal from '$lib/components/modals/ReportsModal.svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { villageStore } from '$lib/stores/village';
  import { armyStore } from '$lib/stores/army';
  import { wsClient, wsState } from '$lib/api/ws';

  let { children }: { children: Snippet } = $props();

  // Subscribe to stores
  let villageState = $state(villageStore);
  let villages = $derived($villageState.villages);
  let currentVillage = $derived($villageState.currentVillage);
  let loading = $derived($villageState.loading);

  let wsConnState = $state(wsState);
  let wsConnected = $derived($wsConnState.connected);

  let selectedVillageId = $state('');

  // Reports modal state
  let reportsModalOpen = $state(false);
  let armyState = $state(armyStore);
  let unreadReportsCount = $derived($armyState.unreadCount);

  // Convert village data to ResourceBar format
  const resources = $derived(currentVillage ? {
    wood: Math.floor(currentVillage.wood),
    clay: Math.floor(currentVillage.clay),
    iron: Math.floor(currentVillage.iron),
    crop: Math.floor(currentVillage.crop),
    warehouseCapacity: currentVillage.warehouse_capacity,
    granaryCapacity: currentVillage.granary_capacity,
    woodProduction: currentVillage.production?.wood_per_hour || 0,
    clayProduction: currentVillage.production?.clay_per_hour || 0,
    ironProduction: currentVillage.production?.iron_per_hour || 0,
    cropProduction: currentVillage.production?.crop_per_hour || 0,
    cropConsumption: currentVillage.production?.crop_consumption || 0
  } : {
    wood: 0, clay: 0, iron: 0, crop: 0,
    warehouseCapacity: 800, granaryCapacity: 800,
    woodProduction: 0, clayProduction: 0, ironProduction: 0,
    cropProduction: 0, cropConsumption: 0
  });

  // TODO: Get gold from player stats API
  const gold = 0;

  const navItems = [
    { href: '/game/village', labelKey: 'nav.village', icon: '🏘️' },
    { href: '/game/map', labelKey: 'nav.map', icon: '🗺️' }
  ];

  function isActive(href: string): boolean {
    return page.url.pathname.startsWith(href);
  }

  async function handleVillageChange(villageId: string) {
    if (!villageId || villageId === currentVillage?.id) return;

    try {
      await villageStore.loadVillage(villageId);
      selectedVillageId = villageId;
    } catch (error: any) {
      toast.error('Failed to load village', {
        description: error.message
      });
    }
  }

  // Initialize game data and WebSocket connection
  onMount(() => {
    let resourceUpdateInterval: ReturnType<typeof setInterval>;
    let unsubscribeResourceUpdate: (() => void) | null = null;
    let unsubscribeBuildComplete: (() => void) | null = null;

    const init = async () => {
      try {
        // Load villages if not already loaded
        if (villages.length === 0) {
          const loadedVillages = await villageStore.loadVillages();
          if (loadedVillages.length > 0) {
            selectedVillageId = loadedVillages[0].id;
            await villageStore.loadVillage(loadedVillages[0].id);
          } else {
            // No villages, redirect to onboarding
            goto('/onboarding');
            return;
          }
        } else if (currentVillage) {
          selectedVillageId = currentVillage.id;
        }

        // Load unread reports count
        armyStore.loadUnreadCount();

        // Connect WebSocket
        await wsClient.connect();

        // Subscribe to resource updates
        unsubscribeResourceUpdate = wsClient.subscribe('resource_update', (data) => {
          if (data.village_id === currentVillage?.id) {
            // Reload village to get updated resources
            villageStore.loadVillage(data.village_id);
          }
        });

        // Subscribe to build complete notifications
        unsubscribeBuildComplete = wsClient.subscribe('build_complete', (data) => {
          toast.success('Building Complete!', {
            description: `${data.building_type} has been upgraded to level ${data.level}`
          });
          // Reload village and buildings
          if (currentVillage) {
            villageStore.loadVillage(currentVillage.id);
          }
        });

        // Fallback: Poll for resource updates every 30 seconds if WebSocket not connected
        resourceUpdateInterval = setInterval(() => {
          if (currentVillage && !wsConnected) {
            villageStore.loadVillage(currentVillage.id);
          }
        }, 30000);

      } catch (error) {
        console.error('Failed to initialize game:', error);
      }
    };

    init();

    return () => {
      if (resourceUpdateInterval) {
        clearInterval(resourceUpdateInterval);
      }
      if (unsubscribeResourceUpdate) {
        unsubscribeResourceUpdate();
      }
      if (unsubscribeBuildComplete) {
        unsubscribeBuildComplete();
      }
      wsClient.disconnect();
    };
  });
</script>

<svelte:head>
  <title>Game - Tusk & Horn</title>
</svelte:head>

<div class="min-h-screen bg-background flex flex-col">
  <!-- Top Bar -->
  <header class="sticky top-0 z-50 bg-background border-b">
    <!-- Main Nav -->
    <div class="flex items-center justify-between px-4 h-14">
      <!-- Logo + Village Selector -->
      <div class="flex items-center gap-4">
        <a href="/game/village" class="flex items-center gap-2">
          <span class="text-xl">🐘</span>
          <span class="font-bold hidden sm:inline">Tusk & Horn</span>
        </a>

        <!-- Village Selector -->
        <select
          bind:value={selectedVillageId}
          onchange={(e) => handleVillageChange(e.currentTarget.value)}
          class="h-9 rounded-md border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
          disabled={loading}
        >
          {#each villages as village}
            <option value={village.id}>
              {village.is_capital ? '👑' : '🏠'} {village.name} ({village.x}|{village.y})
            </option>
          {/each}
        </select>
      </div>

      <!-- Navigation -->
      <nav class="flex items-center gap-1">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors
                   {isActive(item.href)
                     ? 'bg-primary text-primary-foreground'
                     : 'hover:bg-muted'}"
          >
            <span>{item.icon}</span>
            <span class="hidden sm:inline">{$t(item.labelKey)}</span>
          </a>
        {/each}
      </nav>

      <!-- Right Side -->
      <div class="flex items-center gap-2">
        <!-- Language Switcher -->
        <LanguageSwitcher />

        <!-- Reports -->
        <Button variant="ghost" size="icon" class="relative" onclick={() => reportsModalOpen = true}>
          <span class="text-lg">📜</span>
          {#if unreadReportsCount > 0}
            <span class="absolute -top-1 -right-1 w-4 h-4 bg-destructive text-destructive-foreground text-xs rounded-full flex items-center justify-center">
              {unreadReportsCount > 9 ? '9+' : unreadReportsCount}
            </span>
          {/if}
        </Button>

        <!-- Notifications -->
        <Button variant="ghost" size="icon" class="relative">
          <span class="text-lg">🔔</span>
        </Button>

        <!-- Profile -->
        <Button variant="ghost" size="icon">
          <span class="text-lg">👤</span>
        </Button>
      </div>
    </div>

    <!-- Resource Bar -->
    <ResourceBar {resources} {gold} />
  </header>

  <!-- Main Content -->
  <main class="flex-1">
    {@render children()}
  </main>

  <!-- Mobile Bottom Nav -->
  <nav class="md:hidden fixed bottom-0 left-0 right-0 bg-background border-t z-50">
    <div class="flex items-center justify-around h-14">
      {#each navItems as item}
        <a
          href={item.href}
          class="flex flex-col items-center gap-1 px-4 py-2 text-xs
                 {isActive(item.href) ? 'text-primary' : 'text-muted-foreground'}"
        >
          <span class="text-xl">{item.icon}</span>
          <span>{$t(item.labelKey)}</span>
        </a>
      {/each}
      <button class="flex flex-col items-center gap-1 px-4 py-2 text-xs text-muted-foreground">
        <span class="text-xl">⚔️</span>
        <span>{$t('army.send')}</span>
      </button>
      <button
        class="flex flex-col items-center gap-1 px-4 py-2 text-xs text-muted-foreground relative"
        onclick={() => reportsModalOpen = true}
      >
        <span class="text-xl">📜</span>
        <span>{$t('nav.reports')}</span>
        {#if unreadReportsCount > 0}
          <span class="absolute top-1 right-2 w-4 h-4 bg-destructive text-destructive-foreground text-xs rounded-full flex items-center justify-center">
            {unreadReportsCount > 9 ? '9+' : unreadReportsCount}
          </span>
        {/if}
      </button>
    </div>
  </nav>

  <!-- Spacer for mobile bottom nav -->
  <div class="md:hidden h-14"></div>
</div>

<!-- Reports Modal -->
<ReportsModal bind:open={reportsModalOpen} />
