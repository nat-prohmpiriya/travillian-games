<script lang="ts">
  import { t } from 'svelte-i18n';
  import { Button } from '$lib/components/ui/button';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/components/ui/card';
  import { Separator } from '$lib/components/ui/separator';
  import LoginModal from '$lib/components/modals/auth/LoginModal.svelte';
  import RegisterModal from '$lib/components/modals/auth/RegisterModal.svelte';
  import LanguageSwitcher from '$lib/components/LanguageSwitcher.svelte';

  let showLoginModal = $state(false);
  let showRegisterModal = $state(false);

  const features = [
    {
      title: 'Build Your Kingdom',
      description: 'Construct buildings, gather resources, and expand your territory across the land.',
      icon: '🏰'
    },
    {
      title: 'Train Mighty Armies',
      description: 'Command War Elephants, Kris Warriors, and Highland Cavalry to conquer your enemies.',
      icon: '⚔️'
    },
    {
      title: 'Form Alliances',
      description: 'Unite with other players, share resources, and dominate the world together.',
      icon: '🤝'
    },
    {
      title: 'Real-Time Strategy',
      description: 'Every decision matters. Plan your attacks, defend your villages, raid for resources.',
      icon: '🎯'
    }
  ];

  const tribes = [
    {
      name: 'Phasuttha',
      subtitle: 'The Mainland Empire',
      description: 'Masters of war elephants and diplomacy. Balanced in all aspects with powerful defensive walls.',
      color: 'bg-amber-500',
      bonus: '+20% Wall Defense, War Elephants'
    },
    {
      name: 'Nava',
      subtitle: 'The Maritime Raiders',
      description: 'Swift raiders from the archipelago. Excel at quick strikes and naval warfare.',
      color: 'bg-blue-500',
      bonus: '+25% Raid Speed, Naval Units'
    },
    {
      name: 'Kiri',
      subtitle: 'The Highland Warriors',
      description: 'Fierce defenders of the mountains. Specialists in traps and guerrilla warfare.',
      color: 'bg-emerald-500',
      bonus: '+30% Mountain Defense, Traps'
    }
  ];
</script>

<div class="min-h-screen bg-background">
  <!-- Navbar -->
  <nav class="fixed top-0 left-0 right-0 z-50 border-b bg-background/80 backdrop-blur-sm">
    <div class="container mx-auto px-4 h-16 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <span class="text-2xl">🐘</span>
        <span class="font-bold text-xl">Tusk & Horn</span>
      </div>
      <div class="flex items-center gap-3">
        <LanguageSwitcher />
        <Button variant="ghost" onclick={() => showLoginModal = true}>
          {$t('auth.login')}
        </Button>
        <Button onclick={() => showRegisterModal = true}>
          {$t('landing.playNow')}
        </Button>
      </div>
    </div>
  </nav>

  <!-- Hero Section -->
  <section class="pt-32 pb-20 px-4">
    <div class="container mx-auto text-center max-w-4xl">
      <div class="mb-6">
        <span class="text-7xl">🐘</span>
      </div>
      <h1 class="text-5xl md:text-6xl font-bold mb-6 tracking-tight">
        Tusk & Horn
      </h1>
      <p class="text-xl md:text-2xl text-muted-foreground mb-4">
        Southeast Asian Ancient Kingdoms Strategy Game
      </p>
      <p class="text-lg text-muted-foreground mb-8 max-w-2xl mx-auto">
        Build your empire, train legendary armies, and conquer the land.
        Choose your tribe and write your own history.
      </p>
      <div class="flex flex-col sm:flex-row gap-4 justify-center">
        <Button size="lg" class="text-lg px-8" onclick={() => showRegisterModal = true}>
          Start Your Journey
        </Button>
        <Button size="lg" variant="outline" class="text-lg px-8" onclick={() => showLoginModal = true}>
          Continue Playing
        </Button>
      </div>
    </div>
  </section>

  <Separator />

  <!-- Features Section -->
  <section class="py-20 px-4 bg-muted/30">
    <div class="container mx-auto max-w-6xl">
      <h2 class="text-3xl md:text-4xl font-bold text-center mb-12">
        Forge Your Legacy
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {#each features as feature}
          <Card class="text-center hover:shadow-lg transition-shadow">
            <CardHeader>
              <div class="text-4xl mb-2">{feature.icon}</div>
              <CardTitle class="text-lg">{feature.title}</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription class="text-sm">
                {feature.description}
              </CardDescription>
            </CardContent>
          </Card>
        {/each}
      </div>
    </div>
  </section>

  <Separator />

  <!-- Tribes Section -->
  <section class="py-20 px-4">
    <div class="container mx-auto max-w-6xl">
      <h2 class="text-3xl md:text-4xl font-bold text-center mb-4">
        Choose Your Tribe
      </h2>
      <p class="text-center text-muted-foreground mb-12 max-w-2xl mx-auto">
        Three ancient civilizations, each with unique strengths and legendary units.
      </p>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        {#each tribes as tribe}
          <Card class="overflow-hidden hover:shadow-xl transition-all hover:-translate-y-1">
            <div class="{tribe.color} h-2"></div>
            <CardHeader>
              <CardTitle class="text-2xl">{tribe.name}</CardTitle>
              <CardDescription class="text-primary font-medium">
                {tribe.subtitle}
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
              <p class="text-muted-foreground">
                {tribe.description}
              </p>
              <div class="bg-muted rounded-lg p-3 text-sm">
                <span class="font-semibold">Bonus:</span> {tribe.bonus}
              </div>
            </CardContent>
          </Card>
        {/each}
      </div>
    </div>
  </section>

  <Separator />

  <!-- CTA Section -->
  <section class="py-20 px-4 bg-primary text-primary-foreground">
    <div class="container mx-auto text-center max-w-3xl">
      <h2 class="text-3xl md:text-4xl font-bold mb-6">
        Ready to Conquer?
      </h2>
      <p class="text-lg mb-8 opacity-90">
        Join thousands of players in the ultimate Southeast Asian strategy experience.
        Your empire awaits.
      </p>
      <Button
        size="lg"
        variant="secondary"
        class="text-lg px-8"
        onclick={() => showRegisterModal = true}
      >
        Create Free Account
      </Button>
    </div>
  </section>

  <!-- Footer -->
  <footer class="py-8 px-4 border-t">
    <div class="container mx-auto text-center text-muted-foreground text-sm">
      <p>&copy; 2025 Tusk & Horn. All rights reserved.</p>
      <p class="mt-2">Inspired by Southeast Asian ancient kingdoms.</p>
    </div>
  </footer>
</div>

<!-- Modals -->
<LoginModal
  bind:open={showLoginModal}
  onSwitchToRegister={() => {
    showLoginModal = false;
    showRegisterModal = true;
  }}
/>

<RegisterModal
  bind:open={showRegisterModal}
  onSwitchToLogin={() => {
    showRegisterModal = false;
    showLoginModal = true;
  }}
/>
