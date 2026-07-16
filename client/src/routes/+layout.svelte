<script lang="ts">
  import '../app.css';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
  import { onMount } from 'svelte';
  import { isTokenValid } from '$lib/utils/auth';
  import Titlebar from '$lib/components/layout/Titlebar.svelte';

  onMount(() => {
      const interval = setInterval(async () => {
          await isTokenValid();
      }, 10 * 60 * 1000);

      return () => clearInterval(interval);
  });
</script>

<Titlebar />

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 550 }}>
    <slot />
  </div>
{/key}