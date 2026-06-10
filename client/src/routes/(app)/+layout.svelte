<script lang="ts">
  import { fade } from 'svelte/transition';
  import { navigating } from '$app/stores';
</script>

{#key $navigating?.to?.url.pathname}
  <div in:fade={{ duration: 1050 }}>
    <slot />
  </div>
{/key}