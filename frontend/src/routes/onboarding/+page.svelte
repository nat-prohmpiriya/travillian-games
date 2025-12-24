<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { goto } from '$app/navigation';
  import { api } from '$lib/api/client';

  interface CreateVillageResponse {
    id: string;
    name: string;
    x: number;
    y: number;
    is_capital: boolean;
  }

  // Generate random coordinates for new village
  function generateRandomCoordinates(): { x: number; y: number } {
    // Random position within -200 to 200 range
    const x = Math.floor(Math.random() * 401) - 200;
    const y = Math.floor(Math.random() * 401) - 200;
    return { x, y };
  }

  type TribeCode = 'phasuttha' | 'nava' | 'kiri';

  interface Tribe {
    code: TribeCode;
    name: string;
    subtitle: string;
    description: string;
    color: string;
    colorMuted: string;
    bonus: string;
    icon: string;
    enabled: boolean;
  }

  const tribes: Tribe[] = [
    {
      code: 'phasuttha',
      name: 'Phasuttha',
      subtitle: 'The Mainland Empire',
      description: 'Masters of war elephants and diplomacy. Balanced in all aspects with powerful defensive walls. Perfect for beginners and strategic players alike.',
      color: 'bg-amber-500',
      colorMuted: 'bg-amber-500/20',
      bonus: '+20% Wall Defense, War Elephants',
      icon: '🐘',
      enabled: true
    },
    {
      code: 'nava',
      name: 'Nava',
      subtitle: 'The Maritime Raiders',
      description: 'Swift raiders from the archipelago. Excel at quick strikes, naval warfare, and trade routes across the seas.',
      color: 'bg-blue-500',
      colorMuted: 'bg-blue-500/20',
      bonus: '+25% Raid Speed, Naval Units',
      icon: '⛵',
      enabled: false
    },
    {
      code: 'kiri',
      name: 'Kiri',
      subtitle: 'The Highland Warriors',
      description: 'Fierce defenders of the mountains. Specialists in traps, guerrilla warfare, and ambush tactics.',
      color: 'bg-emerald-500',
      colorMuted: 'bg-emerald-500/20',
      bonus: '+30% Mountain Defense, Traps',
      icon: '⛰️',
      enabled: false
    }
  ];

  let selectedTribe = $state<TribeCode>('phasuttha');
  let playerName = $state('');
  let loading = $state(false);
  let error = $state('');

  function selectTribe(tribe: Tribe) {
    if (tribe.enabled) {
      selectedTribe = tribe.code;
    }
  }

  async function handleStartGame(e: Event) {
    e.preventDefault();

    if (!playerName.trim()) {
      error = 'Please enter your player name';
      return;
    }

    if (playerName.length < 3) {
      error = 'Player name must be at least 3 characters';
      return;
    }

    if (playerName.length > 20) {
      error = 'Player name must be less than 20 characters';
      return;
    }

    loading = true;
    error = '';

    try {
      // Generate random coordinates for the new village
      const coords = generateRandomCoordinates();

      // Create village via API
      const village = await api.post<CreateVillageResponse>('/api/villages', {
        name: playerName,
        x: coords.x,
        y: coords.y,
      });

      console.log('Village created:', village);

      // Redirect to village
      goto('/game/village');
    } catch (err: any) {
      // Handle coordinate conflict - retry with new coordinates
      if (err.message?.includes('Coordinates already occupied')) {
        error = 'Location taken, please try again';
      } else {
        error = err.message || 'Failed to create village';
      }
    } finally {
      loading = false;
    }
  }

  const selectedTribeData = $derived(tribes.find(t => t.code === selectedTribe)!);
</script>

<svelte:head>
  <title>Choose Your Tribe - Tusk & Horn</title>
</svelte:head>

<div class="min-h-screen bg-background flex flex-col">
  <!-- Header -->
  <header class="border-b py-4 px-4">
    <div class="container mx-auto flex items-center gap-2">
      <span class="text-2xl">🐘</span>
      <span class="font-bold text-xl">Tusk & Horn</span>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 py-8 px-4">
    <div class="container mx-auto max-w-5xl">
      <!-- Title -->
      <div class="text-center mb-8">
        <h1 class="text-3xl md:text-4xl font-bold mb-2">Choose Your Tribe</h1>
        <p class="text-muted-foreground">
          Select your civilization and begin your conquest
        </p>
      </div>

      <!-- Tribe Selection -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        {#each tribes as tribe}
          <button
            type="button"
            onclick={() => selectTribe(tribe)}
            disabled={!tribe.enabled}
            class="text-left focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 rounded-xl transition-all
                   {!tribe.enabled ? 'opacity-60 cursor-not-allowed' : 'hover:-translate-y-1'}"
          >
            <Card class="h-full overflow-hidden transition-all
                        {selectedTribe === tribe.code ? 'ring-2 ring-primary shadow-lg' : ''}
                        {!tribe.enabled ? '' : 'hover:shadow-lg'}">
              <!-- Color Bar -->
              <div class="{tribe.color} h-2"></div>

              <!-- Coming Soon Badge -->
              {#if !tribe.enabled}
                <div class="absolute top-4 right-4">
                  <span class="bg-muted text-muted-foreground text-xs font-medium px-2 py-1 rounded-full">
                    Coming Soon
                  </span>
                </div>
              {/if}

              <CardHeader class="relative">
                <div class="flex items-center gap-3 mb-2">
                  <div class="{tribe.colorMuted} w-12 h-12 rounded-xl flex items-center justify-center text-2xl">
                    {tribe.icon}
                  </div>
                  {#if selectedTribe === tribe.code}
                    <div class="ml-auto">
                      <div class="w-6 h-6 rounded-full bg-primary flex items-center justify-center">
                        <svg class="w-4 h-4 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                      </div>
                    </div>
                  {/if}
                </div>
                <CardTitle class="text-xl">{tribe.name}</CardTitle>
                <CardDescription class="text-primary font-medium">
                  {tribe.subtitle}
                </CardDescription>
              </CardHeader>

              <CardContent class="space-y-4">
                <p class="text-sm text-muted-foreground leading-relaxed">
                  {tribe.description}
                </p>
                <div class="{tribe.colorMuted} rounded-lg p-3 text-sm">
                  <span class="font-semibold">Bonus:</span> {tribe.bonus}
                </div>
              </CardContent>
            </Card>
          </button>
        {/each}
      </div>

      <!-- Player Name Form -->
      <Card class="max-w-md mx-auto">
        <CardHeader>
          <CardTitle>Your Identity</CardTitle>
          <CardDescription>
            Choose a name for your empire
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onsubmit={handleStartGame} class="space-y-4">
            <div class="space-y-2">
              <Label for="player-name">Player Name</Label>
              <Input
                id="player-name"
                type="text"
                placeholder="Enter your name"
                bind:value={playerName}
                disabled={loading}
                maxlength={20}
              />
              <p class="text-xs text-muted-foreground">
                3-20 characters. This will be visible to other players.
              </p>
            </div>

            {#if error}
              <p class="text-sm text-destructive">{error}</p>
            {/if}

            <!-- Selected Tribe Summary -->
            <div class="flex items-center gap-3 p-3 bg-muted rounded-lg">
              <div class="{selectedTribeData.colorMuted} w-10 h-10 rounded-lg flex items-center justify-center text-xl">
                {selectedTribeData.icon}
              </div>
              <div>
                <p class="font-medium">{selectedTribeData.name}</p>
                <p class="text-xs text-muted-foreground">{selectedTribeData.bonus}</p>
              </div>
            </div>

            <Button type="submit" class="w-full h-12 text-base" disabled={loading || !playerName.trim()}>
              {#if loading}
                <svg class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Creating your empire...
              {:else}
                Start Your Journey
              {/if}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  </main>

  <!-- Footer -->
  <footer class="py-4 px-4 border-t">
    <div class="container mx-auto text-center text-muted-foreground text-sm">
      <p>Your tribe choice affects your units and bonuses throughout the game.</p>
    </div>
  </footer>
</div>
