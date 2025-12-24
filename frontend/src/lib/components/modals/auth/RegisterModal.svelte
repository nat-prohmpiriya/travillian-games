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
    onSwitchToLogin?: () => void;
  }

  let { open = $bindable(false), onSwitchToLogin }: Props = $props();

  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  let loading = $state(false);
  let error = $state('');

  async function handleEmailRegister(e: Event) {
    e.preventDefault();

    if (password !== confirmPassword) {
      error = 'Passwords do not match';
      return;
    }

    if (password.length < 6) {
      error = 'Password must be at least 6 characters';
      return;
    }

    loading = true;
    error = '';

    try {
      // TODO: Implement email/password registration with Firebase
      // await createUserWithEmailAndPassword(auth, email, password);
      console.log('Email register:', email);
      // For now, just show not implemented
      error = 'Email registration coming soon. Please use Google login.';
    } catch (err: any) {
      error = err.message || 'Registration failed';
    } finally {
      loading = false;
    }
  }

  async function handleGoogleRegister() {
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
      error = err.message || 'Google sign up failed';
    } finally {
      loading = false;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <Dialog.Title class="text-2xl text-center">Create Account</Dialog.Title>
      <Dialog.Description class="text-center">
        Start your journey in the ancient kingdoms
      </Dialog.Description>
    </Dialog.Header>

    <div class="space-y-6 py-4">
      <!-- Google Sign Up -->
      <Button
        variant="outline"
        class="w-full h-12 text-base"
        onclick={handleGoogleRegister}
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
        Sign up with Google
      </Button>

      <div class="relative">
        <div class="absolute inset-0 flex items-center">
          <Separator class="w-full" />
        </div>
        <div class="relative flex justify-center text-xs uppercase">
          <span class="bg-background px-2 text-muted-foreground">Or sign up with email</span>
        </div>
      </div>

      <!-- Email/Password Form -->
      <form onsubmit={handleEmailRegister} class="space-y-4">
        <div class="space-y-2">
          <Label for="register-email">Email</Label>
          <Input
            id="register-email"
            type="email"
            placeholder="you@example.com"
            bind:value={email}
            required
            disabled={loading}
          />
        </div>

        <div class="space-y-2">
          <Label for="register-password">Password</Label>
          <Input
            id="register-password"
            type="password"
            placeholder="At least 6 characters"
            bind:value={password}
            required
            disabled={loading}
          />
        </div>

        <div class="space-y-2">
          <Label for="confirm-password">Confirm Password</Label>
          <Input
            id="confirm-password"
            type="password"
            placeholder="Confirm your password"
            bind:value={confirmPassword}
            required
            disabled={loading}
          />
        </div>

        {#if error}
          <p class="text-sm text-destructive">{error}</p>
        {/if}

        <Button type="submit" class="w-full h-12 text-base" disabled={loading}>
          {loading ? 'Creating account...' : 'Create Account'}
        </Button>
      </form>

      <p class="text-center text-sm text-muted-foreground">
        Already have an account?
        <button
          type="button"
          class="text-primary hover:underline font-medium"
          onclick={onSwitchToLogin}
        >
          Login
        </button>
      </p>

      <p class="text-center text-xs text-muted-foreground">
        By creating an account, you agree to our
        <a href="/terms" class="underline hover:text-foreground">Terms of Service</a>
        and
        <a href="/privacy" class="underline hover:text-foreground">Privacy Policy</a>
      </p>
    </div>
  </Dialog.Content>
</Dialog.Root>
