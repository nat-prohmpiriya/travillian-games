<script lang="ts">
  import type { Snippet } from 'svelte';
  import { t } from 'svelte-i18n';
  import { Button } from '$lib/components/ui/button';
  import ResourceBar from '$lib/components/game/ResourceBar.svelte';
  import LanguageSwitcher from '$lib/components/LanguageSwitcher.svelte';
  import { page } from '$app/stores';

  let { children }: { children: Snippet } = $props();

  // Mock data - จะเปลี่ยนเป็น store/API ทีหลัง
  const mockResources = {
    wood: 1250,
    clay: 980,
    iron: 750,
    crop: 1100,
    warehouseCapacity: 2000,
    granaryCapacity: 2000,
    woodProduction: 120,
    clayProduction: 100,
    ironProduction: 80,
    cropProduction: 150,
    cropConsumption: 45
  };

  const mockGold = 50;

  const mockVillages = [
    { id: '1', name: 'Capital', x: 100, y: 100, isCapital: true },
    { id: '2', name: 'Outpost', x: 105, y: 98, isCapital: false }
  ];

  let selectedVillageId = $state('1');

  const navItems = [
    { href: '/game/village', labelKey: 'nav.village', icon: '🏘️' },
    { href: '/game/map', labelKey: 'nav.map', icon: '🗺️' }
  ];

  function isActive(href: string): boolean {
    return $page.url.pathname.startsWith(href);
  }
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
          class="h-9 rounded-md border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
        >
          {#each mockVillages as village}
            <option value={village.id}>
              {village.isCapital ? '👑' : '🏠'} {village.name} ({village.x}|{village.y})
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

        <!-- Notifications -->
        <Button variant="ghost" size="icon" class="relative">
          <span class="text-lg">🔔</span>
          <span class="absolute -top-1 -right-1 w-4 h-4 bg-destructive text-destructive-foreground text-xs rounded-full flex items-center justify-center">
            3
          </span>
        </Button>

        <!-- Profile -->
        <Button variant="ghost" size="icon">
          <span class="text-lg">👤</span>
        </Button>
      </div>
    </div>

    <!-- Resource Bar -->
    <ResourceBar resources={mockResources} gold={mockGold} />
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
      <button class="flex flex-col items-center gap-1 px-4 py-2 text-xs text-muted-foreground">
        <span class="text-xl">📜</span>
        <span>{$t('nav.reports')}</span>
      </button>
    </div>
  </nav>

  <!-- Spacer for mobile bottom nav -->
  <div class="md:hidden h-14"></div>
</div>
