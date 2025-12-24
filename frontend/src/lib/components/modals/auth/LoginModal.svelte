<script lang="ts">
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import { Separator } from '$lib/components/ui/separator';
  import { authStore } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  interface Props {
    open: boolean;
    onSwitchToRegister?: () => void;
  }

  let { open = $bindable(false), onSwitchToRegister }: Props = $props();

  let email = $state('');
  let password = $state('');
  let loading = $state(false);
  let error = $state('');

  async function handleEmailLogin(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';

    try {
      // TODO: Implement email/password login with Firebase
      // await signInWithEmailAndPassword(auth, email, password);
      console.log('Email login:', email);
      // For now, just show not implemented
      error = 'Email login coming soon. Please use Google login.';
    } catch (err: any) {
      error = err.message || 'Login failed';
    } finally {
      loading = false;
    }
  }

  async function handleGoogleLogin() {
    loading = true;
    error = '';

    try {
      const { hasVillage } = await authStore.loginWithGoogle();
      open = false;

      // Redirect based on whether user has a village
      if (hasVillage) {
        goto('/game/village');
      } else {
        goto('/onboarding');
      }
    } catch (err: any) {
      error = err.message || 'Google login failed';
    } finally {
      loading = false;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <Dialog.Title class="text-2xl text-center">Welcome Back</Dialog.Title>
      <Dialog.Description class="text-center">
        Login to continue your conquest
      </Dialog.Description>
    </Dialog.Header>

    <div class="space-y-6 py-4">
      <!-- Google Login -->
      <Button
        variant="outline"
        class="w-full h-12 text-base"
        onclick={handleGoogleLogin}
        disabled={loading}
      >
        <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24">
          <path
            fill="currentColor"
            d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
          />
          <path
            fill="currentColor"
            d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
          />
          <path
            fill="currentColor"
            d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
          />
          <path
            fill="currentColor"
            d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
          />
        </svg>
        Continue with Google
      </Button>

      <div class="relative">
        <div class="absolute inset-0 flex items-center">
          <Separator class="w-full" />
        </div>
        <div class="relative flex justify-center text-xs uppercase">
          <span class="bg-background px-2 text-muted-foreground">Or continue with email</span>
        </div>
      </div>

      <!-- Email/Password Form -->
      <form onsubmit={handleEmailLogin} class="space-y-4">
        <div class="space-y-2">
          <Label for="email">Email</Label>
          <Input
            id="email"
            type="email"
            placeholder="you@example.com"
            bind:value={email}
            required
            disabled={loading}
          />
        </div>

        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <Label for="password">Password</Label>
            <button type="button" class="text-sm text-primary hover:underline">
              Forgot password?
            </button>
          </div>
          <Input
            id="password"
            type="password"
            placeholder="Enter your password"
            bind:value={password}
            required
            disabled={loading}
          />
        </div>

        {#if error}
          <p class="text-sm text-destructive">{error}</p>
        {/if}

        <Button type="submit" class="w-full h-12 text-base" disabled={loading}>
          {loading ? 'Logging in...' : 'Login'}
        </Button>
      </form>

      <p class="text-center text-sm text-muted-foreground">
        Don't have an account?
        <button
          type="button"
          class="text-primary hover:underline font-medium"
          onclick={onSwitchToRegister}
        >
          Sign up
        </button>
      </p>
    </div>
  </Dialog.Content>
</Dialog.Root>
