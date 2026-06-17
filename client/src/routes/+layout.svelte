<script lang="ts">
  import '../app.css';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
  import { onMount } from 'svelte';
  import { isTokenValid } from '$lib/utils/auth';
  import { Store } from '@tauri-apps/plugin-store';

  onMount(() => {
      const interval = setInterval(async () => {
          const tokenValid = await isTokenValid();

          if (tokenValid) {
              const store = await Store.load('.auth.dat');
              const accessToken = await store.get('access_token');
          }
      }, (10 * 60 * 1000)); // Check every 10 minutes

      return () => clearInterval(interval);
  });

</script>

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 450 }}>
    <slot />
  </div>
{/key}