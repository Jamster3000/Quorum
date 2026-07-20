<script lang="ts">
  import '../app.css';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
  import { listen } from '@tauri-apps/api/event';
  import { refreshAccessToken, isTokenValid } from '$lib/utils/auth';
  import { clearAuthStore } from '$lib/stores/authStore';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import Titlebar from '$lib/components/layout/Titlebar.svelte';

  onMount(() => {
  listen('auth:token-expired', async () => {
    const refreshed = await refreshAccessToken();
    if (!refreshed) {
      await clearAuthStore();
      goto('/login?from=expired');
    }
  });

  listen('auth:refresh-failed', async () => {
    await clearAuthStore();
    goto('/login?from=expired');
  });
});
</script>

<Titlebar />

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 550 }}>
    <slot />
  </div>
{/key}