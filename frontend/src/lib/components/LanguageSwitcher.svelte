<script lang="ts">
  import { locale } from 'svelte-i18n';
  import { Button } from '$lib/components/ui/button';

  const languages = [
    { code: 'en', label: 'EN', flag: '🇬🇧' },
    { code: 'th', label: 'TH', flag: '🇹🇭' }
  ];

  function switchLanguage() {
    const current = $locale?.substring(0, 2) || 'en';
    const next = current === 'en' ? 'th' : 'en';
    locale.set(next);
  }

  $effect(() => {
    if ($locale) {
      document.documentElement.lang = $locale.substring(0, 2);
    }
  });
</script>

<Button variant="ghost" size="sm" onclick={switchLanguage} class="gap-1 px-2">
  {#if $locale?.startsWith('th')}
    <span>🇹🇭</span>
    <span class="text-xs">TH</span>
  {:else}
    <span>🇬🇧</span>
    <span class="text-xs">EN</span>
  {/if}
</Button>
