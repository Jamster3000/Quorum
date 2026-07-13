<script lang="ts">
  import '../app.css';
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
  import { onMount } from 'svelte';
  import { isTokenValid } from '$lib/utils/auth';

  onMount(() => {
      const interval = setInterval(async () => {
          await isTokenValid();
      }, 10 * 60 * 1000);

      return () => clearInterval(interval);
  });
</script>

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 450 }}>
    <slot />
  </div>
{/key}